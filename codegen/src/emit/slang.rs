// TODO integrate with "slang" shader language
//  hypothetically should be nice to avoid monolith file

// I could emit to slang directly,
// but might also want to check out rust-slang integrations like https://github.com/tangmi/slang-rs/

use std::{fs, thread};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::io::{BufRead, BufReader, BufWriter, ErrorKind, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use anyhow::bail;
use indicatif::ProgressFinish;
use tokio::task::JoinSet;

use crate::algebra::basis::BasisElement;
use crate::algebra::multivector::{MultiVec, MultiVecRepository};
use crate::ast::datatype::{ExpressionType, MultiVector};
use crate::ast::expressions::{AnyExpression, FloatExpr, IntExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr};
use crate::ast::RawVariableDeclaration;
use crate::ast::traits::{BinaryOps, CommentOrVariableDeclaration, progress_style, RawTraitDefinition, RawTraitImplementation, TraitArity, TraitImplRegistry, TraitKey, TraitParam, TraitTypeConsensus};
use crate::emit::sort_trait_impls;
use crate::utility::CollectResults;

#[derive(Copy, Clone)]
pub struct Slang {
    pub prefer_fancy_infix: bool,
    // Internal/private stuff
    fancy_infix: Option<BinaryOps>,
}

impl Slang {
    pub fn new() -> Self {
        Slang {
            prefer_fancy_infix: false,
            fancy_infix: None,
        }
    }

    #[allow(non_upper_case_globals)]
    pub fn write_src<P: AsRef<Path>, const AntiScalar: BasisElement>(
        self,
        src_folder: P,
        algebra_name: &'static str,
        multi_vecs: Arc<MultiVecRepository<AntiScalar>>,
        impls: Arc<TraitImplRegistry>,
    ) {

        let rt = tokio::runtime::Runtime::new().expect("Tokio must work");
        let result = rt.block_on(async {
            self.write_src_inner(src_folder, algebra_name, multi_vecs, impls).await
        });
        if let Err(e) = result {
            panic!("Slang Errors: {e:?}");
        }
    }

    #[allow(non_upper_case_globals)]
    async fn write_src_inner<P: AsRef<Path>, const AntiScalar: BasisElement>(
        mut self,
        src_folder: P,
        algebra_name: &'static str,
        multi_vecs: Arc<MultiVecRepository<AntiScalar>>,
        impls: Arc<TraitImplRegistry>,
    ) -> anyhow::Result<()> {
        let src_folder = src_folder.as_ref().to_path_buf().join(Path::new("integrations/slang"));
        let folder_data = src_folder.join(Path::new("data"));
        let folder_data_impls = src_folder.join(Path::new("data/impls"));
        let folder_traits = src_folder.join(Path::new("traits"));
        let folder_traits_impls = src_folder.join(Path::new("traits/impls"));

        fs::create_dir_all(&folder_data_impls)?;
        fs::create_dir_all(&folder_traits_impls)?;

        let mut defs = impls.get_defs().await;
        defs.sort_by(|a, b| a.names.trait_key.cmp(&b.names.trait_key));
        let defs = defs;
        let fancy_infix = *impls.infix_trick.lock();
        self.fancy_infix = fancy_infix;
        let impls = impls.get_impls().await;
        let mut mvs = multi_vecs.declarations();
        mvs.sort_by(|a, b| a.name.cmp(&b.name));
        let mvs = mvs;
        let mv_docs = multi_vecs.documentation();

        let file_path = src_folder.join(Path::new("generated-files.txt"));
        if let Ok(file) = fs::OpenOptions::new().read(true).open(&file_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                let path = Path::new(line);
                match fs::remove_file(path) {
                    Ok(_) => {}
                    Err(e) if e.kind() == ErrorKind::NotFound => {}
                    e => e?,
                };
            }
        }

        let multi_progress = Arc::new(indicatif::MultiProgress::new());
        let mut join_set: JoinSet<anyhow::Result<()>> = JoinSet::new();
        let (started_file, mut rx) = tokio::sync::mpsc::unbounded_channel::<PathBuf>();
        join_set.spawn(async move {
            let file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
            let mut file = BufWriter::new(file);
            let mut paths = vec![];
            while let Some(p) = rx.recv().await {
                use std::fmt::Write;
                let mut path = String::new();
                write!(&mut path, ".")?;
                for p in p.iter() {
                    let p = p.to_string_lossy();
                    write!(&mut path, "/{p}")?;
                }
                paths.push(path);
            }
            paths.sort();
            for p in paths {
                writeln!(&mut file, "{p}")?;
            }
            Ok(())
        });

        let qty_mvs = mvs.len() as u64;
        let data_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_mvs).with_finish(ProgressFinish::AndLeave)));
        data_pb.set_style(progress_style());
        data_pb.set_message("Slang - Data Definitions");

        let qty_defs = defs.len() as u64;
        let trait_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_defs).with_finish(ProgressFinish::AndLeave)));
        trait_pb.set_style(progress_style());
        trait_pb.set_message("Slang - Trait Definitions");

        let qty_impls = impls.len() as u64;
        let impls_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_impls)));
        impls_pb.set_style(progress_style());
        impls_pb.set_message("Slang - Distributing Trait Implementations");

        // let qty_files = qty_mvs + qty_defs + 4; // traits.slang, data.slang, lib.slang, simd.slang
        // let fmt_pb = Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_files).with_finish(ProgressFinish::AndLeave)));
        // fmt_pb.set_style(progress_style());
        // fmt_pb.set_message("Slang - rustfmt");

        // let (finished_file, mut rx) = tokio::sync::mpsc::unbounded_channel::<PathBuf>();
        // let fmt_pb2 = fmt_pb.clone();
        // join_set.spawn(async move {
        //     while let Some(p) = rx.recv().await {
        //         Self::format_file(p).await?;
        //         fmt_pb2.inc(1);
        //     }
        //     fmt_pb2.finish();
        //     Ok(())
        // });

        // data definitions
        for multi_vec in mvs.iter() {
            let multi_vec = *multi_vec;
            let mv = MultiVector::from(multi_vec);
            let n = mv.name();
            let lsc = TraitKey::new(n).as_lower_snake();
            let folder_data = folder_data.clone();
            let doc = mv_docs.get(&n.to_string()).cloned();
            let tx2 = started_file.clone();
            // let tx3 = finished_file.clone();

            let pb2 = data_pb.clone();
            join_set.spawn(async move {
                let file_path = folder_data.join(Path::new(&lsc)).with_extension("slang");
                tx2.send(file_path.clone())?;
                let file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
                let mut file = BufWriter::new(file);
                writeln!(&mut file, "implementing {algebra_name};")?;
                writeln!(&mut file, "using data;")?;
                self.declare_multi_vector(&mut file, multi_vec, doc)?;
                writeln!(&mut file, "__include \"impls/{lsc}\";")?;
                // tx3.send(file_path)?;
                pb2.inc(1);
                Ok(())
            });
        }

        // trait definitions
        for td in defs.iter() {
            let td = td.clone();
            if let TraitTypeConsensus::NoVotes = *td.output.read() {
                trait_pb.inc(1);
                continue;
            }
            let k = td.names.trait_key;
            let n = k.as_upper_camel();
            let lsc = k.as_lower_snake();
            let folder_traits = folder_traits.clone();
            match n.as_str() {
                "Add" | "Sub" | "Mul" | "Div" | "Shl" | "Shr" | "BitAnd" | "BitOr" | "BitXor" | "Neg" | "Not" => {
                    trait_pb.inc(1);
                    continue;
                }
                // "Into" | "TryInto" => {
                //     trait_pb.inc(1);
                //     continue;
                // }
                _ => {}
            }
            let tx2 = started_file.clone();
            // let tx3 = finished_file.clone();
            let pb2 = trait_pb.clone();
            join_set.spawn(async move {
                let file_path = folder_traits.join(Path::new(lsc.as_str())).with_extension("slang");
                tx2.send(file_path.clone())?;
                let file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
                let mut file = BufWriter::new(file);
                writeln!(&mut file, "implementing {algebra_name};")?;
                // TODO remove these imports when they are unused
                writeln!(&mut file, "using data;")?;
                self.declare_trait_def(&mut file, td)?;
                writeln!(&mut file, "__include \"./impls/{lsc}\";")?;
                // tx3.send(file_path)?;
                pb2.inc(1);
                Ok(())
            });
        }

        let mut impl_files: HashMap<String, (Vec<Arc<RawTraitImplementation>>, BTreeSet<TraitKey>)> = HashMap::new();

        // trait impl distribution
        for i in impls {
            let k = i.definition.names.trait_key;
            let (folder, name) = match k.as_upper_camel().as_str() {
                "Add" | "Sub" | "Mul" | "Div" | "Shl" | "Shr" | "BitAnd" | "BitOr" | "BitXor" | "Neg" | "Not" => {
                    let ExpressionType::Class(mv) = i.owner else { continue };
                    let n = TraitKey::new(mv.name()).as_lower_snake();
                    ("data", n)
                }
                // "Into" | "TryInto" => {
                //     let Some(ExpressionType::Class(mv)) = i.other_type_params.get(0) else { continue };
                //     let n = TraitKey::new(mv.name()).as_lower_snake();
                //     ("data", n)
                // }
                _ => ("traits", k.as_lower_snake()),
            };
            let i2 = i.clone();
            impl_files
                .entry(format!("{folder}/impls/{name}.slang"))
                .and_modify(move |(v, deps)| {
                    for t_dep in i2.definition.dependencies.lock().iter().cloned() {
                        deps.insert(t_dep);
                    }
                    v.push(i2);
                })
                .or_insert_with(|| {
                    let t_deps: BTreeSet<_> = i.definition.dependencies.lock().iter().cloned().collect();
                    (vec![i], t_deps)
                });
            impls_pb.inc(1);
        }
        impls_pb.finish();
        // fmt_pb.inc_length(impl_files.len() as u64);

        // trait implementations
        let (closed_types, mut closed_types_rx) = tokio::sync::mpsc::unbounded_channel::<String>();
        for (file_path, (mut impls, deps)) in impl_files {
            let src_folder = src_folder.clone();
            let tx2 = started_file.clone();
            // let tx3 = finished_file.clone();
            let multi_progress = multi_progress.clone();
            join_set.spawn(async move {
                let skip_dependencies = file_path.ends_with("constraint_valid.slang");
                let file_path = src_folder.join(Path::new(file_path.as_str())).with_extension("slang");

                let qty_impls = impls.len() as u64;
                let qty_deps = deps.len() as u64;
                let mut pb = None;
                if qty_impls > 100 {
                    pb = Some(Arc::new(multi_progress.add(indicatif::ProgressBar::new(qty_impls + qty_deps + 2))));
                }
                if let Some(pb) = &pb {
                    pb.set_style(progress_style());
                    let fpd = file_path.display();
                    pb.set_message(format!("Slang - {fpd}"));
                }

                tx2.send(file_path.clone())?;
                let file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
                let mut file = BufWriter::new(file);
                writeln!(&mut file, "implementing {algebra_name};")?;
                writeln!(&mut file, "using traits;")?;
                let mut deps_set = HashSet::new();
                for dep in deps {
                    if let Some(pb) = &pb {
                        pb.inc(1);
                    }
                    deps_set.insert(dep);
                    if skip_dependencies { continue }
                    // if self.prefer_fancy_infix {
                    //     let lsc = dep.as_lower_snake();
                    //     writeln!(&mut file, "using traits;")?;
                    // } else {
                    //     let ucc = dep.as_upper_camel();
                    //     writeln!(&mut file, "using traits;")?;
                    // }
                }
                sort_trait_impls(&mut impls, deps_set)?;
                if let Some(pb) = &pb {
                    pb.inc(1);
                }


                // writeln!(&mut file, "use crate::data::*;")?;
                // writeln!(&mut file, "use crate::simd::*;")?;
                let mut already_granted_infix = BTreeSet::new();
                for i in impls {
                    let ucc = i.definition.names.trait_key.as_upper_camel();
                    match ucc.as_str() {
                        "Into" => self.write_trait_from(&mut file, i.clone())?,
                        "TryInto" => self.write_trait_try_from(&mut file, i.clone())?,
                        _ => self.declare_trait_impl(&mut file, i, &mut already_granted_infix)?,
                    };
                    if let Some(pb) = &pb {
                        pb.inc(1);
                    }
                }
                // tx3.send(file_path)?;
                if let Some(pb) = &pb {
                    pb.inc(1);
                    pb.finish_and_clear();
                }
                Ok(())
            });
        }

        // data.slang
        let src_folder2 = src_folder.clone();
        let tx2 = started_file.clone();
        // let tx3 = finished_file.clone();
        join_set.spawn(async move {
            let file_path = src_folder2.join(Path::new("data.slang"));
            tx2.send(file_path.clone())?;
            let file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
            let mut file = BufWriter::new(file);
            writeln!(&mut file, "implementing {algebra_name};")?;
            write!(&mut file, r#"
internal bool lessThanHelper<T: IComparable>(T a, T b) {{
    return a.lessThan(b);
}}
internal bool equalsHelper<T: IComparable>(T a, T b) {{
    return a.equals(b);
}}
internal bool lessThanOrEqualsHelper<T: IComparable>(T a, T b) {{
    return a.lessThanOrEquals(b);
}}
            "#)?;

            for mv in mvs {
                let n = mv.name;
                let lsc = TraitKey::new(n).as_lower_snake();
                writeln!(&mut file, "__include \"data/{lsc}\";")?;
            }
            Ok(())
        });

        // traits.slang
        let src_folder2 = src_folder.clone();
        let tx2 = started_file.clone();
        // let tx3 = finished_file.clone();
        join_set.spawn(async move {
            let file_path = src_folder2.join(Path::new("traits.slang"));
            tx2.send(file_path.clone())?;
            let file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
            let mut file = BufWriter::new(file);
            writeln!(&mut file, "implementing {algebra_name};")?;
            for td in defs.iter() {
                if let TraitTypeConsensus::NoVotes = *td.output.read() {
                    continue;
                }
                let td = td.clone();
                let k = td.names.trait_key;
                let n = k.as_upper_camel();
                let lsc = k.as_lower_snake();
                match n.as_str() {
                    "Add" | "Sub" | "Mul" | "Div" | "Shl" | "Shr" | "BitAnd" | "BitOr" | "BitXor" | "Neg" | "Not" => continue,
                    // "Into" | "TryInto" => continue,
                    _ => {}
                }

                writeln!(&mut file, "__include \"traits/{lsc}\";")?;
            }
            // tx3.send(file_path)?;
            Ok(())
        });

        // lib.slang
        let src_folder2 = src_folder.clone();
        let tx2 = started_file.clone();
        // let tx3 = finished_file.clone();
        join_set.spawn(async move {
            let n = format!("{algebra_name}.slang");
            let file_path = src_folder2.join(Path::new(n.as_str()));
            tx2.send(file_path.clone())?;
            let file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path)?;
            let mut file = BufWriter::new(file);
            writeln!(&mut file, "namespace data {{")?;
            writeln!(&mut file, "    __include data;")?;
            writeln!(&mut file, "}}")?;
            writeln!(&mut file, "namespace traits {{")?;
            writeln!(&mut file, "    __include traits;")?;
            writeln!(&mut file, "}}")?;
            Ok(())
        });

        drop(started_file);
        // drop(finished_file);
        let result = join_set.collect_results().await;

        tokio::task::spawn_blocking(move || {
            thread::sleep(Duration::from_secs(5));
            drop(closed_types);
            while let Some(thing) = closed_types_rx.blocking_recv() {
                println!("{thing}");
            }
        });

        result
    }

    fn write_type<W: Write>(&self, w: &mut W, data_type: ExpressionType) -> anyhow::Result<()> {
        match data_type {
            ExpressionType::Int(_) => write!(w, "uint")?,
            ExpressionType::Float(_) => write!(w, "float")?,
            ExpressionType::Vec2(_) => write!(w, "float2")?,
            ExpressionType::Vec3(_) => write!(w, "float3")?,
            ExpressionType::Vec4(_) => write!(w, "float4")?,
            ExpressionType::Class(mv) => {
                let n = mv.name();
                write!(w, "{n}")?;
            }
        }
        Ok(())
    }

    fn write_expression<W: Write>(&self, w: &mut W, expr: &AnyExpression, grouping_provided: bool) -> anyhow::Result<()> {
        match expr {
            AnyExpression::Int(e) => self.write_int(w, e)?,
            AnyExpression::Float(e) => self.write_float(w, e, grouping_provided)?,
            AnyExpression::Vec2(e) => self.write_vec2(w, e, grouping_provided)?,
            AnyExpression::Vec3(e) => self.write_vec3(w, e, grouping_provided)?,
            AnyExpression::Vec4(e) => self.write_vec4(w, e, grouping_provided)?,
            AnyExpression::Class(e) => self.write_multi_vec(w, e)?,
        }
        Ok(())
    }

    fn write_int<W: Write>(&self, w: &mut W, expr: &IntExpr) -> anyhow::Result<()> {
        match expr {
            IntExpr::Variable(v) => {
                let name = &v.decl.name.0;
                let no = v.decl.name.1;
                if name.as_str() == "self" && no == 0 {
                    write!(w, "this")?;
                } else if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    let no = no + 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            IntExpr::Literal(l) => {
                write!(w, "{l}")?;
            }
            IntExpr::TraitInvoke10ToInt(t, mv) => {
                let n = mv.name();
                let method = t.as_lower_snake();
                write!(w, "{n}.{method}()")?;
            }
        }
        Ok(())
    }

    fn write_f32<W: Write>(&self, w: &mut W, f: f32) -> anyhow::Result<()> {
        if f.fract() == 0.0 {
            write!(w, "{f:.1}")?;
        } else {
            write!(w, "{f}")?;
        }
        Ok(())
    }

    fn write_float<W: Write>(&self, w: &mut W, expr: &FloatExpr, grouping_provided: bool) -> anyhow::Result<()> {
        match expr {
            FloatExpr::Variable(v) => {
                let name = &v.decl.name.0;
                let no = v.decl.name.1;
                if name.as_str() == "self" && no == 0 {
                    write!(w, "this")?;
                } else if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    let no = no + 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            FloatExpr::Literal(l) => self.write_f32(w, *l)?,
            FloatExpr::FromInt(i) => {
                write!(w, "float(")?;
                self.write_int(w, i)?;
                write!(w, ")")?;
            }
            FloatExpr::AccessVec2(v, i) => {
                self.write_vec2(w, v.as_ref(), false)?;
                write!(w, "[{i}]")?;
            }
            FloatExpr::AccessVec3(v, i) => {
                self.write_vec3(w, v.as_ref(), false)?;
                write!(w, "[{i}]")?;
            }
            FloatExpr::AccessVec4(v, i) => {
                self.write_vec4(w, v.as_ref(), false)?;
                write!(w, "[{i}]")?;
            }
            FloatExpr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                let el = mv.mv_class.elements()[*i as usize];
                write!(w, ".{el}")?;
            }
            FloatExpr::AccessMultiVecFlat(mv, i) => {
                self.write_multi_vec(w, mv)?;
                let el = mv.mv_class.elements()[*i as usize];
                write!(w, ".{el}")?;
            }
            FloatExpr::TraitInvoke11ToFloat(t, arg) => {
                let method = t.as_lower_snake();
                // TODO fancy infix can conflict with variable names
                match (&self.fancy_infix, self.prefer_fancy_infix) {
                    (Some(infix), true) => {
                        let op = infix.slang_operator();
                        write!(w, "(")?;
                        self.write_multi_vec(w, arg)?;
                        write!(w, " {op}{method}")?;
                        write!(w, ")")?;
                    }
                    _ => {
                        self.write_multi_vec(w, arg)?;
                        write!(w, ".{method}()")?;
                    }
                }
            }
            FloatExpr::Product(v, last_factor) => {
                let has_last_factor = *last_factor != 1.0;
                if v.is_empty() && !has_last_factor {
                    bail!("Attempted to write an empty product that should have been simplified");
                }
                let mut len = v.len();
                if has_last_factor {
                    len += 1;
                }
                if len > 1 && !grouping_provided {
                    write!(w, "(")?;
                }
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    match (*exponent, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => self.write_float(w, factor, false)?,
                        (-1.0, false) => {
                            if !grouping_provided {
                                write!(w, "(")?;
                            }
                            write!(w, "1.0/")?;
                            self.write_float(w, factor, false)?;
                            if !grouping_provided {
                                write!(w, ")")?;
                            }
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                if e == 2 && factor.is_memory_read_and_not_compute() {
                                    self.write_float(w, factor, false)?;
                                    write!(w, " * ")?;
                                    self.write_float(w, factor, false)?;
                                } else {
                                    write!(w, "pow(")?;
                                    self.write_float(w, factor, true)?;
                                    write!(w, ", {e})")?;
                                }
                            } else {
                                write!(w, "pow(")?;
                                self.write_float(w, factor, true)?;
                                write!(w, ", {e})")?;
                            }
                        }

                        (1.0, true) => {
                            write!(w, " * ")?;
                            self.write_float(w, factor, false)?;
                        }
                        (-1.0, true) => {
                            write!(w, " / (")?;
                            self.write_float(w, factor, true)?;
                            write!(w, ")")?;
                        }
                        (e, true) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, " * pow(")?;
                                self.write_float(w, factor, true)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, " * pow(")?;
                                self.write_float(w, factor, true)?;
                                write!(w, ", {e})")?;
                            }
                        }
                    }
                }
                match (*last_factor, len > 1) {
                    (fl, _) if fl == 1.0 => {}
                    (fl, false) => self.write_f32(w, fl)?,
                    (fl, true) => {
                        write!(w, " * ")?;
                        self.write_f32(w, fl)?
                    }
                }
                if len > 1 && !grouping_provided {
                    write!(w, ")")?;
                }
            }
            FloatExpr::Sum(v, last_addend) => {
                let has_last_addend = *last_addend != 0.0;
                if v.is_empty() && !has_last_addend {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                let mut len = v.len();
                if has_last_addend {
                    len += 1;
                }
                if len > 1 && !grouping_provided {
                    write!(w, "(")?;
                }
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => {}
                        (-1.0, false) => write!(w, "-")?,
                        (f, false) => {
                            self.write_f32(w, f)?;
                            write!(w, "*")?
                        }

                        (1.0, true) => write!(w, " + ")?,
                        (-1.0, true) => write!(w, " - ")?,
                        (f, true) if f > 0.0 => {
                            write!(w, " + ")?;
                            self.write_f32(w, f)?;
                            write!(w, " * ")?;
                        }
                        (f, true) if f < 0.0 => {
                            let f = -f;
                            write!(w, " - ")?;
                            self.write_f32(w, f)?;
                            write!(w, " * ")?;
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_float(w, addend, false)?;
                }
                match (*last_addend, len > 1) {
                    (fl, _) if fl == 0.0 => {}
                    (fl, false) => self.write_f32(w, fl)?,
                    (fl, true) if fl > 0.0 => {
                        write!(w, " + ")?;
                        self.write_f32(w, fl)?
                    }
                    (fl, true) if fl < 0.0 => {
                        let fl = -fl;
                        write!(w, " - ")?;
                        self.write_f32(w, fl)?
                    }
                    _ => {}
                }
                if len > 1 && !grouping_provided {
                    write!(w, ")")?;
                }
            }
            FloatExpr::Exp(factor, exponent, last_exponent) => {

                if exponent.is_none() && last_exponent.fract() == 0.0 {
                    write!(w, "pow(")?;
                } else {
                    write!(w, "pow(")?;
                }
                self.write_float(w, factor, true)?;
                write!(w, ", ")?;
                if let Some(exponent) = exponent {
                    self.write_float(w, exponent, *last_exponent == 1.0)?;
                    if *last_exponent != 1.0 {
                        write!(w, " * ")?;
                    }
                }
                if *last_exponent != 1.0 {
                    write!(w, "{last_exponent}")?;
                }
                write!(w, ")")?;
            }
        }
        Ok(())
    }

    fn write_vec2<W: Write>(&self, w: &mut W, expr: &Vec2Expr, grouping_provided: bool) -> anyhow::Result<()> {
        match expr {
            Vec2Expr::Variable(v) => {
                let name = &v.decl.name.0;
                let no = v.decl.name.1;
                if name.as_str() == "self" && no == 0 {
                    write!(w, "this")?;
                } else if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    let no = no + 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            Vec2Expr::Gather1(f) => {
                write!(w, "float2(")?;
                self.write_float(w, f, true)?;
                write!(w, ")")?;
            }
            Vec2Expr::Gather2(f0, f1) => {
                write!(w, "float2(")?;
                self.write_float(w, f0, true)?;
                write!(w, ", ")?;
                self.write_float(w, f1, true)?;
                write!(w, ")")?;
            }
            Vec2Expr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                write!(w, ".group{i}.xy")?;
            }
            Vec2Expr::Product(v, last_factor) => {
                let has_last_factor = *last_factor != [1.0; 2];
                if v.is_empty() && !has_last_factor {
                    bail!("Attempted to write an empty product that should have been simplified");
                }
                let mut len = v.len();
                if has_last_factor {
                    len += 1;
                }
                if len > 1 && !grouping_provided {
                    write!(w, "(")?;
                }
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    match (*exponent, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => self.write_vec2(w, factor, false)?,
                        (-1.0, false) => {
                            write!(w, "(float2(1.0) / ")?;
                            self.write_vec2(w, factor, false)?;
                            write!(w, ")")?;
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, "Simd32x2::pow(")?;
                                self.write_vec2(w, factor, true)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, "Simd32x2::pow(")?;
                                self.write_vec2(w, factor, true)?;
                                write!(w, ", {e})")?;
                            }
                        }

                        (1.0, true) => {
                            write!(w, " * ")?;
                            self.write_vec2(w, factor, false)?
                        }
                        (-1.0, true) => {
                            write!(w, " / (")?;
                            self.write_vec2(w, factor, true)?;
                            write!(w, ")")?;
                        }
                        (e, true) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, " * Simd32x2::pow(")?;
                                self.write_vec2(w, factor, true)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, " * Simd32x2::pow(")?;
                                self.write_vec2(w, factor, true)?;
                                write!(w, ", {e})")?;
                            }
                        }
                    }
                }
                if *last_factor != [1.0; 2] {
                    if len > 1 {
                        write!(w, " * ")?;
                    }
                    let a = last_factor[0];
                    let b = last_factor[1];
                    if a == b {
                        write!(w, "float2(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "float2(")?;
                        self.write_f32(w, a)?;
                        write!(w, ", ")?;
                        self.write_f32(w, b)?;
                        write!(w, ")")?;
                    }
                }
                if len > 1 && !grouping_provided {
                    write!(w, ")")?;
                }
            }
            Vec2Expr::Sum(v, last_addend) => {
                let has_last_addend = *last_addend != [0.0; 2];
                if v.is_empty() && !has_last_addend {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                let mut len = v.len();
                if has_last_addend {
                    len += 1;
                }
                if len > 1 && !grouping_provided {
                    write!(w, "(")?;
                }
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => {}
                        (-1.0, false) => write!(w, "-")?,
                        (f, false) => {
                            write!(w, "float2(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }

                        (1.0, true) => write!(w, " + ")?,
                        (-1.0, true) => write!(w, " - ")?,
                        (f, true) if f > 0.0 => {
                            write!(w, " + float2(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        (f, true) if f < 0.0 => {
                            let f = -f;
                            write!(w, " - float2(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec2(w, addend, false)?;
                }
                let a0 = last_addend[0];
                let a1 = last_addend[1];
                if *last_addend != [0.0; 2] {
                    if len > 1 {
                        write!(w, " + ")?;
                    }
                    if a0 == a1 {
                        write!(w, "float2(")?;
                        self.write_f32(w, a0)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "float2(")?;
                        self.write_f32(w, a0)?;
                        write!(w, ", ")?;
                        self.write_f32(w, a1)?;
                        write!(w, ")")?;
                    }
                }
                if len > 1 && !grouping_provided {
                    write!(w, ")")?;
                }
            }
            Vec2Expr::SwizzleVec2(box v, i0, i1) => {
                match v {
                    Vec2Expr::Truncate3to2(box v3) => self.write_vec3(w, v3, false)?,
                    Vec2Expr::Truncate4to2(box v4) => self.write_vec4(w, v4, false)?,
                    Vec2Expr::AccessMultiVecGroup(mv, i) => {
                        self.write_multi_vec(w, mv)?;
                        write!(w, ".group{i}")?;
                    }
                    _ => self.write_vec2(w, v, false)?,
                }
                let x = swizzle_term(i0)?;
                let y = swizzle_term(i1)?;
                write!(w, ".{x}{y}")?;
            },
            Vec2Expr::Truncate3to2(box Vec3Expr::SwizzleVec3(box v3, i0, i1, _)) => {
                self.write_vec3(w, v3, false)?;
                let x = swizzle_term(i0)?;
                let y = swizzle_term(i1)?;
                write!(w, ".{x}{y}")?;
            }
            Vec2Expr::Truncate3to2(box v3) => {
                self.write_vec3(w, v3, false)?;
                write!(w, ".xy")?;
            }
            Vec2Expr::Truncate4to2(box Vec4Expr::SwizzleVec4(box v4, i0, i1, _, _)) => {
                self.write_vec4(w, v4, false)?;
                let x = swizzle_term(i0)?;
                let y = swizzle_term(i1)?;
                write!(w, ".{x}{y}")?;
            }
            Vec2Expr::Truncate4to2(box v4) => {
                self.write_vec4(w, v4, false)?;
                write!(w, ".xy")?;
            }
        }
        Ok(())
    }

    fn write_vec3<W: Write>(&self, w: &mut W, expr: &Vec3Expr, grouping_provided: bool) -> anyhow::Result<()> {
        match expr {
            Vec3Expr::Variable(v) => {
                let name = &v.decl.name.0;
                let no = v.decl.name.1;
                if name.as_str() == "self" && no == 0 {
                    write!(w, "this")?;
                } else if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    let no = no + 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            Vec3Expr::Gather1(f) => {
                write!(w, "float3(")?;
                self.write_float(w, f, true)?;
                write!(w, ")")?;
            }
            Vec3Expr::Gather3(f0, f1, f2) => {
                write!(w, "float3(")?;
                self.write_float(w, f0, true)?;
                write!(w, ", ")?;
                self.write_float(w, f1, true)?;
                write!(w, ", ")?;
                self.write_float(w, f2, true)?;
                write!(w, ")")?;
            }
            Vec3Expr::Extend2to3(v2, f1) => {
                write!(w, "float3(")?;
                self.write_vec2(w, v2, false)?;
                write!(w, ", ")?;
                self.write_float(w, f1, true)?;
                write!(w, ")")?;
            }
            Vec3Expr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                write!(w, ".group{i}.xyz")?;
            }
            Vec3Expr::Product(v, last_factor) => {
                let has_last_factor = *last_factor != [1.0; 3];
                if v.is_empty() && !has_last_factor {
                    bail!("Attempted to write an empty product that should have been simplified");
                }
                let mut len = v.len();
                if has_last_factor {
                    len += 1;
                }
                if len > 1 && !grouping_provided {
                    write!(w, "(")?;
                }
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    match (*exponent, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => self.write_vec3(w, factor, false)?,
                        (-1.0, false) => {
                            write!(w, "(float3(1.0) / ")?;
                            self.write_vec3(w, factor, false)?;
                            write!(w, ")")?;
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, "pow(")?;
                                self.write_vec3(w, factor, true)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, "pow(")?;
                                self.write_vec3(w, factor, true)?;
                                write!(w, ", {e})")?;
                            }
                        }

                        (1.0, true) => {
                            write!(w, " * ")?;
                            self.write_vec3(w, factor, false)?
                        }
                        (-1.0, true) => {
                            write!(w, " / (")?;
                            self.write_vec3(w, factor, true)?;
                            write!(w, ")")?;
                        }
                        (e, true) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, " * pow(")?;
                                self.write_vec3(w, factor, true)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, " * pow(")?;
                                self.write_vec3(w, factor, true)?;
                                write!(w, ", {e})")?;
                            }
                        }
                    }
                }
                if *last_factor != [1.0; 3] {
                    if len > 1 {
                        write!(w, " * ")?;
                    }
                    let a = last_factor[0];
                    let b = last_factor[1];
                    let c = last_factor[2];
                    if a == b && b == c {
                        write!(w, "float3(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "float3(")?;
                        self.write_f32(w, a)?;
                        write!(w, ", ")?;
                        self.write_f32(w, b)?;
                        write!(w, ", ")?;
                        self.write_f32(w, c)?;
                        write!(w, ")")?;
                    }
                }
                if len > 1 && !grouping_provided {
                    write!(w, ")")?;
                }
            }
            Vec3Expr::Sum(v, last_addend) => {
                let has_last_addend = *last_addend != [0.0; 3];
                if v.is_empty() && !has_last_addend {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                let mut len = v.len();
                if has_last_addend {
                    len += 1;
                }
                if len > 1 && !grouping_provided {
                    write!(w, "(")?;
                }
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => {}
                        (-1.0, false) => write!(w, "-")?,
                        (f, false) => {
                            write!(w, "float3(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }

                        (1.0, true) => write!(w, " + ")?,
                        (-1.0, true) => write!(w, " - ")?,
                        (f, true) if f > 0.0 => {
                            write!(w, " + float3(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        (f, true) if f < 0.0 => {
                            let f = -f;
                            write!(w, " - float3(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec3(w, addend, false)?;
                }
                if *last_addend != [0.0; 3] {
                    if len > 1 {
                        write!(w, " + ")?;
                    }
                    let a = last_addend[0];
                    let b = last_addend[1];
                    let c = last_addend[2];
                    if a == b && b == c {
                        write!(w, "float3(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "float3(")?;
                        self.write_f32(w, a)?;
                        write!(w, ", ")?;
                        self.write_f32(w, b)?;
                        write!(w, ", ")?;
                        self.write_f32(w, c)?;
                        write!(w, ")")?;
                    }
                }
                if len > 1 && !grouping_provided {
                    write!(w, ")")?;
                }
            }
            Vec3Expr::SwizzleVec3(box v, i0, i1, i2) => {
                match v {
                    Vec3Expr::Truncate4to3(box v4) => self.write_vec4(w, v4, false)?,
                    Vec3Expr::AccessMultiVecGroup(mv, i) => {
                        self.write_multi_vec(w, mv)?;
                        write!(w, ".group{i}")?;
                    }
                    _ => self.write_vec3(w, v, false)?,
                }
                let x = swizzle_term(i0)?;
                let y = swizzle_term(i1)?;
                let z = swizzle_term(i2)?;
                write!(w, ".{x}{y}{z}")?;
            },
            Vec3Expr::Truncate4to3(box Vec4Expr::SwizzleVec4(box v4, i0, i1, i2, _)) => {
                self.write_vec4(w, v4, false)?;
                let x = swizzle_term(i0)?;
                let y = swizzle_term(i1)?;
                let z = swizzle_term(i2)?;
                write!(w, ".{x}{y}{z}")?;
            }
            Vec3Expr::Truncate4to3(box v4) => {
                self.write_vec4(w, v4, false)?;
                write!(w, ".xyz")?;
            }
        }
        Ok(())
    }

    fn write_vec4<W: Write>(&self, w: &mut W, expr: &Vec4Expr, grouping_provided: bool) -> anyhow::Result<()> {
        match expr {
            Vec4Expr::Variable(v) => {
                let name = &v.decl.name.0;
                let no = v.decl.name.1;
                if name.as_str() == "self" && no == 0 {
                    write!(w, "this")?;
                } else if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    let no = no + 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            Vec4Expr::Gather1(f) => {
                write!(w, "float4(")?;
                self.write_float(w, f, true)?;
                write!(w, ")")?;
            }
            Vec4Expr::Gather4(f0, f1, f2, f3) => {
                write!(w, "float4(")?;
                self.write_float(w, f0, true)?;
                write!(w, ", ")?;
                self.write_float(w, f1, true)?;
                write!(w, ", ")?;
                self.write_float(w, f2, true)?;
                write!(w, ", ")?;
                self.write_float(w, f3, true)?;
                write!(w, ")")?;
            }
            Vec4Expr::Extend2to4(v2, f1, f2) => {
                write!(w, "float4(")?;
                self.write_vec2(w, v2, false)?;
                write!(w, ", ")?;
                self.write_float(w, f1, true)?;
                write!(w, ", ")?;
                self.write_float(w, f2, true)?;
                write!(w, ")")?;
            }
            Vec4Expr::Extend3to4(v3, f1) => {
                write!(w, "float4(")?;
                self.write_vec3(w, v3, false)?;
                write!(w, ", ")?;
                self.write_float(w, f1, true)?;
                write!(w, ")")?;
            }
            Vec4Expr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                write!(w, ".group{i}")?;
            }
            Vec4Expr::Product(v, last_factor) => {
                let has_last_factor = *last_factor != [1.0; 4];
                if v.is_empty() && !has_last_factor {
                    bail!("Attempted to write an empty product that should have been simplified");
                }
                let mut len = v.len();
                if has_last_factor {
                    len += 1;
                }
                if len > 1 && !grouping_provided {
                    write!(w, "(")?;
                }
                for (i, (factor, exponent)) in v.iter().enumerate() {
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    match (*exponent, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => self.write_vec4(w, factor, false)?,
                        (-1.0, false) => {
                            write!(w, "(float4(1.0) / ")?;
                            self.write_vec4(w, factor, false)?;
                            write!(w, ")")?;
                        }
                        (e, false) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, "pow(")?;
                                self.write_vec4(w, factor, true)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, "pow(")?;
                                self.write_vec4(w, factor, true)?;
                                write!(w, ", {e})")?;
                            }
                        }

                        (1.0, true) => {
                            write!(w, " * ")?;
                            self.write_vec4(w, factor, false)?
                        }
                        (-1.0, true) => {
                            write!(w, " / (")?;
                            self.write_vec4(w, factor, true)?;
                            write!(w, ")")?;
                        }
                        (e, true) => {
                            if e.fract() == 0.0 && e <= i32::MAX as f32 && e >= i32::MIN as f32 {
                                let e = e as i32;
                                write!(w, " * pow(")?;
                                self.write_vec4(w, factor, true)?;
                                write!(w, ", {e})")?;
                            } else {
                                write!(w, " * pow(")?;
                                self.write_vec4(w, factor, true)?;
                                write!(w, ", {e})")?;
                            }
                        }
                    }
                }
                if *last_factor != [1.0; 4] {
                    if len > 1 {
                        write!(w, " * ")?;
                    }
                    let a = last_factor[0];
                    let b = last_factor[1];
                    let c = last_factor[2];
                    let d = last_factor[3];
                    if a == b && b == c && c == d {
                        write!(w, "float4(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "float4(")?;
                        self.write_f32(w, a)?;
                        write!(w, ", ")?;
                        self.write_f32(w, b)?;
                        write!(w, ", ")?;
                        self.write_f32(w, c)?;
                        write!(w, ", ")?;
                        self.write_f32(w, d)?;
                        write!(w, ")")?;
                    }
                }
                if len > 1 && !grouping_provided {
                    write!(w, ")")?;
                }
            }
            Vec4Expr::Sum(v, last_addend) => {
                let has_last_addend = *last_addend != [0.0; 4];
                if v.is_empty() && !has_last_addend {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                let mut len = v.len();
                if has_last_addend {
                    len += 1;
                }
                if len > 1 && !grouping_provided {
                    write!(w, "(")?;
                }
                for (i, (addend, factor)) in v.iter().enumerate() {
                    match (*factor, i > 0) {
                        (f, _) if f == 0.0 => continue,

                        (1.0, false) => {}
                        (-1.0, false) => write!(w, "-")?,
                        (f, false) => {
                            write!(w, "float4(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }

                        (1.0, true) => write!(w, " + ")?,
                        (-1.0, true) => write!(w, " - ")?,
                        (f, true) if f > 0.0 => {
                            write!(w, " + float4(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        (f, true) if f < 0.0 => {
                            let f = -f;
                            write!(w, " - float4(")?;
                            self.write_f32(w, f)?;
                            write!(w, ")*")?;
                        }
                        _ => unreachable!("This match is complete across if conditions (unless NaN?)"),
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec4(w, addend, false)?;
                }
                if *last_addend != [0.0; 4] {
                    if len > 1 {
                        write!(w, " + ")?;
                    }
                    let a = last_addend[0];
                    let b = last_addend[1];
                    let c = last_addend[2];
                    let d = last_addend[3];
                    if a == b && b == c && c == d {
                        write!(w, "float4(")?;
                        self.write_f32(w, a)?;
                        write!(w, ")")?;
                    } else {
                        write!(w, "float4(")?;
                        self.write_f32(w, a)?;
                        write!(w, ", ")?;
                        self.write_f32(w, b)?;
                        write!(w, ", ")?;
                        self.write_f32(w, c)?;
                        write!(w, ", ")?;
                        self.write_f32(w, d)?;
                        write!(w, ")")?;
                    }
                }
                if len > 1 && !grouping_provided {
                    write!(w, ")")?;
                }
            }
            Vec4Expr::SwizzleVec4(box v, i0, i1, i2, i3) => {
                self.write_vec4(w, v, false)?;
                let x = swizzle_term(i0)?;
                let y = swizzle_term(i1)?;
                let z = swizzle_term(i2)?;
                let w2 = swizzle_term(i3)?;
                write!(w, ".{x}{y}{z}{w2}")?;
            }
        }
        Ok(())
    }

    fn write_multi_vec<W: Write>(&self, w: &mut W, expr: &MultiVectorExpr) -> anyhow::Result<()> {
        let mv = expr.mv_class;
        match &*expr.expr {
            MultiVectorVia::Variable(v) => {
                let name = &v.decl.name.0;
                let no = v.decl.name.1;
                if name.as_str() == "self" && no == 0 {
                    write!(w, "this")?;
                } else if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    let no = no + 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            MultiVectorVia::Construct(v) => {
                let n = mv.name();
                write!(w, "{n}(")?;
                let groups = mv.groups();
                for (i, g) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, ", ")?;
                    }
                    write!(w, "\n            /* ")?;
                    for (i, el) in groups[i].into_vec().into_iter().enumerate() {
                        if i > 0 {
                            write!(w, ", ")?;
                        }
                        write!(w, "{el}")?;
                    }
                    write!(w, " */\n            ")?;
                    match g {
                        MultiVectorGroupExpr::JustFloat(f) => self.write_float(w, f, true)?,
                        MultiVectorGroupExpr::Vec2(g) => self.write_vec2(w, g, true)?,
                        MultiVectorGroupExpr::Vec3(g) => self.write_vec3(w, g, true)?,
                        MultiVectorGroupExpr::Vec4(g) => self.write_vec4(w, g, true)?,
                    }
                }
                write!(w, "\n        )")?;
            }
            MultiVectorVia::TraitInvoke11ToClass(t, arg) => {
                let method = t.as_lower_snake();
                // TODO fancy infix can conflict with variable names
                match (&self.fancy_infix, self.prefer_fancy_infix) {
                    (Some(infix), true) => {
                        let op = infix.slang_operator();
                        write!(w, "(")?;
                        self.write_multi_vec(w, arg)?;
                        write!(w, " {op}{method}")?;
                        write!(w, ")")?;
                    }
                    _ => {
                        self.write_multi_vec(w, arg)?;
                        write!(w, ".{method}()")?;
                    }
                }
            }
            MultiVectorVia::TraitInvoke21ToClass(t, arg, mv) => {
                self.write_multi_vec(w, arg)?;
                let method = t.as_lower_snake();
                let b = mv.name();
                // TODO
                write!(w, ".{method}::<{b}>()")?;
            }
            MultiVectorVia::TraitInvoke22ToClass(t, a, b) => {
                let method = t.as_lower_snake();
                // TODO fancy infix can conflict with variable names
                match (&self.fancy_infix, self.prefer_fancy_infix) {
                    (Some(infix), true) => {
                        let op = infix.slang_operator();
                        write!(w, "(")?;
                        self.write_multi_vec(w, a)?;
                        write!(w, " {op}")?;
                        write!(w, "{method}")?;
                        write!(w, "{op} ")?;
                        self.write_multi_vec(w, b)?;
                        write!(w, ")")?;
                    }
                    _ => {
                        self.write_multi_vec(w, a)?;
                        write!(w, ".{method}(")?;
                        self.write_multi_vec(w, b)?;
                        write!(w, ")")?;
                    }
                }
            }
            MultiVectorVia::TraitInvoke12iToClass(t, a, b) => {
                let method = t.as_lower_snake();
                // TODO fancy infix can conflict with variable names
                match (&self.fancy_infix, self.prefer_fancy_infix) {
                    (Some(infix), true) => {
                        let op = infix.rust_operator();
                        write!(w, "(")?;
                        self.write_multi_vec(w, a)?;
                        write!(w, " {op}")?;
                        write!(w, "{method}")?;
                        write!(w, "{op} ")?;
                        self.write_int(w, b)?;
                        write!(w, ")")?;
                    }
                    _ => {
                        self.write_multi_vec(w, a)?;
                        write!(w, ".{method}(")?;
                        self.write_int(w, b)?;
                        write!(w, ")")?;
                    }
                }
            }
            MultiVectorVia::TraitInvoke12fToClass(t, a, b) => {
                let method = t.as_lower_snake();
                // TODO fancy infix can conflict with variable names
                match (&self.fancy_infix, self.prefer_fancy_infix) {
                    (Some(infix), true) => {
                        let op = infix.slang_operator();
                        write!(w, "(")?;
                        self.write_multi_vec(w, a)?;
                        write!(w, " {op}")?;
                        write!(w, "{method}")?;
                        write!(w, "{op} ")?;
                        self.write_float(w, b, false)?;
                        write!(w, ")")?;
                    }
                    _ => {
                        self.write_multi_vec(w, a)?;
                        write!(w, ".{method}(")?;
                        self.write_float(w, b, true)?;
                        write!(w, ")")?;
                    }
                }
            }
        }
        Ok(())
    }

    fn write_trait_from<W: Write>(&self, w: &mut W, impls: Arc<RawTraitImplementation>) -> anyhow::Result<()> {
        // TODO
        let ExpressionType::Class(other) = impls.owner else {
            bail!("Owner of Into (Other of From) impl is not a MultiVector")
        };
        let Some(ExpressionType::Class(owner)) = impls.other_type_params.get(0) else {
            bail!("Other of Into (Owner of From) impl is not a MultiVector")
        };
        let other = other.name();
        let owner = owner.name();
        let other_lsc = TraitKey::new(other).as_lower_snake();
        let other_lsc = format!("from_{other_lsc}");

        // writeln!(w, "public extension {owner}: From<{other}> {{")?;
        writeln!(w, "public extension {owner} {{")?;
        writeln!(w, "    public static {owner} from({other} {other_lsc}) {{")?;
        let mut ret = impls.return_expr.clone();
        let old_var = Arc::new(RawVariableDeclaration {
            comment: None,
            name: ("self".to_string(), 0),
            expr: None,
            force_inline: Arc::new(AtomicBool::new(false)),
        });
        let new_var = Arc::new(RawVariableDeclaration {
            comment: None,
            name: (other_lsc, 0),
            expr: None,
            force_inline: Arc::new(AtomicBool::new(false)),
        });
        ret.substitute_variable(old_var, new_var);
        write!(w, "        return ")?;
        self.write_expression(w, &ret, true)?;
        writeln!(w, ";\n    }}")?;
        writeln!(w, "}}")?;
        writeln!(w, "public extension {other} {{")?;
        // writeln!(w, "public extension {other}: Into<{owner}> {{")?;
        writeln!(w, "    public {owner} into_{owner}() {{")?;
        writeln!(w, "        return {owner}.from(this);")?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_trait_try_from<W: Write>(&self, w: &mut W, impls: Arc<RawTraitImplementation>) -> anyhow::Result<()> {
        // TODO
        let ExpressionType::Class(other) = impls.owner else {
            bail!("Owner of Into (Other of From) impl is not a MultiVector")
        };
        let Some(ExpressionType::Class(owner)) = impls.other_type_params.get(0) else {
            bail!("Other of TryInto (Owner of TryFrom) impl is not a MultiVector")
        };
        let destination_elements: BTreeSet<_> = owner.elements().into_iter().collect();
        let misfit_elements: Vec<_> = other.elements().into_iter().filter(|el| !destination_elements.contains(el)).collect();
        let other = other.name();
        let owner = owner.name();
        let lsc = TraitKey::new(other).as_lower_snake();
        // writeln!(w, "public extension {owner}: TryFrom<{other}> {{")?;
        writeln!(w, "public extension {owner} {{")?;
        writeln!(w, "    public static Optional<{owner}> try_from({other} {lsc}) {{")?;
        let mut ret = impls.return_expr.clone();
        let old_var = Arc::new(RawVariableDeclaration {
            comment: None,
            name: ("self".to_string(), 0),
            expr: None,
            force_inline: Arc::new(AtomicBool::new(false)),
        });
        let new_var = Arc::new(RawVariableDeclaration {
            comment: None,
            name: (lsc.clone(), 0),
            expr: None,
            force_inline: Arc::new(AtomicBool::new(false)),
        });
        ret.substitute_variable(old_var, new_var);
        for el in misfit_elements {
            write!(
                w,
                r#"
        float disallowed_{el} = {lsc}.{el};
        if (disallowed_{el} != 0.0) {{
            return none;
        }}"#
            )?;
        }
        write!(w, "\n        return ")?;
        self.write_expression(w, &ret, true)?;
        writeln!(w, ";\n    }}\n}}")?;

        // writeln!(w, "public extension {other}: TryInto<{owner}> {{")?;
        writeln!(w, "public extension {other} {{")?;
        writeln!(w, "    public Optional<{owner}> try_into_{owner}() {{")?;
        writeln!(w, "        return {owner}.try_from(this);")?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;
        Ok(())
    }

    #[allow(non_upper_case_globals)]
    fn declare_multi_vector<W: Write, const AntiScalar: BasisElement>(
        &self, w: &mut W, multi_vec: &'static MultiVec<AntiScalar>, docs: Option<String>
    ) -> anyhow::Result<()> {
        let name = TraitKey::new(multi_vec.name);
        let ucc = name.as_upper_camel();
        // let lcc = name.as_lower_camel();
        // let lsc = name.as_lower_snake();
        // let ssc = name.as_screaming_snake();
        // TODO built in documentation, statistics, and traits that output this type
        let docs = docs.unwrap_or(ucc.clone());
        self.emit_comment(w, true, docs)?;

        // TODO hybrid rust/wgsl - only one struct, based on vecs, but with properties

        let name = TraitKey::new(multi_vec.name);
        let ucc = name.as_upper_camel();
        // let lcc = name.as_lower_camel();
        write!(w, "public struct {ucc} {{")?;
        for (i, g) in multi_vec.groups().into_iter().enumerate() {
            write!(w, "\n    // ")?;
            let g = g.into_vec();
            for (i, el) in g.clone().into_iter().enumerate() {
                if i > 0 {
                    write!(w, ", ")?;
                }
                write!(w, "{el}")?;
            }
            let l = g.len();
            match l {
                1 => write!(w, ", 0, 0, 0")?,
                2 => write!(w, ", 0, 0")?,
                3 => write!(w, ", 0")?,
                _ => (),
            }
            // TODO consider modifying visibility of groups in rust implementation
            write!(w, "\n    internal float4 group{i};")?;
        }


        writeln!(w, "\n\n    public __init(")?;
        for (i, el) in multi_vec.elements().into_iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            } else {
                write!(w, "        ")?;
            }
            write!(w, "float {el}")?;
        }
        writeln!(w, "\n    ) {{")?;


        for (outer_idx, g) in multi_vec.groups().into_iter().enumerate() {
            write!(w, "        group{outer_idx} = float4(")?;
            let mut count = 0;
            for (inner_idx, el) in g.into_iter().enumerate() {
                if inner_idx > 0 {
                    write!(w, ", ")?;
                }
                write!(w, "{el}")?;
                count += 1;
            }
            while count < 4 {
                write!(w, ", 0.0")?;
                count += 1;
            }
            writeln!(w, ");")?;
        }
        writeln!(w, "    }}")?;


        if !multi_vec.groups().into_iter().all(|it| it.simd_width() == 1) {
            writeln!(w, "    internal __init(")?;
            for (i, g) in multi_vec.groups().into_iter().enumerate() {
                if i > 0 {
                    write!(w, ", ")?;
                } else {
                    write!(w, "        ")?;
                }
                self.write_type(w, g.expr_type())?;
                write!(w, " g{i}")?;
            }
            writeln!(w, "\n    ) {{")?;
            for (i, g) in multi_vec.groups().into_iter().enumerate() {
                match g.simd_width() {
                    1 => writeln!(w, "        group{i} = float4(g{i}, 0.0, 0.0, 0.0);")?,
                    2 => writeln!(w, "        group{i} = float4(g{i}, 0.0, 0.0);")?,
                    3 => writeln!(w, "        group{i} = float4(g{i}, 0.0);")?,
                    4 => writeln!(w, "        group{i} = g{i};")?,
                    _ => {}
                }
            }
            writeln!(w, "    }}")?;
        }


        writeln!(w, "}}")?;



        writeln!(w, "public extension {ucc} {{")?;
        for (outer_idx, g) in multi_vec.groups().into_iter().enumerate() {
            let g = g.into_vec();
            for (inner_idx, el) in g.clone().into_iter().enumerate() {
                // TODO consider nicer property access API in rust implementation
                writeln!(w, "    public property {el}: float {{")?;
                writeln!(w, "        get {{ return group{outer_idx}[{inner_idx}]; }}")?;
                writeln!(w, "        set {{ group{outer_idx}[{inner_idx}] = newValue; }}")?;
                writeln!(w, "    }}")?;
            }
        }

        // writeln!(w, "    public static {ucc} from_elements(")?;
        // for (i, el) in multi_vec.elements().into_iter().enumerate() {
        //     if i > 0 {
        //         write!(w, ", ")?;
        //     } else {
        //         write!(w, "        ")?;
        //     }
        //     write!(w, "float {el}")?;
        // }
        // writeln!(w, "\n    ) {{")?;
        // write!(w, "        return {ucc}(")?;
        // for (outer_idx, g) in multi_vec.groups().into_iter().enumerate() {
        //     if outer_idx > 0 {
        //         write!(w, ", ")?;
        //     }
        //     write!(w, "float4(")?;
        //     let mut count = 0;
        //     for (inner_idx, el) in g.into_iter().enumerate() {
        //         if inner_idx > 0 {
        //             write!(w, ", ")?;
        //         }
        //         write!(w, "{el}")?;
        //         count += 1;
        //     }
        //     while count < 4 {
        //         write!(w, ", 0.0")?;
        //         count += 1;
        //     }
        //     write!(w, ")")?;
        // }
        // writeln!(w, ");")?;
        // writeln!(w, "    }}")?;
        // writeln!(w, "    internal static {ucc} from_groups(")?;
        // for (i, g) in multi_vec.groups().into_iter().enumerate() {
        //     if i > 0 {
        //         write!(w, ", ")?;
        //     } else {
        //         write!(w, "        ")?;
        //     }
        //     self.write_type(w, g.expr_type())?;
        //     write!(w, " g{i}")?;
        // }
        // writeln!(w, "\n    ) {{")?;
        // write!(w, "        return {ucc}(")?;
        // for (i, _g) in multi_vec.groups().into_iter().enumerate() {
        //     if i > 0 {
        //         write!(w, ", ")?;
        //     }
        //     write!(w, "g{i}")?;
        // }
        // writeln!(w, ");\n    }}")?;
        writeln!(w, "}}")?;


        writeln!(w, "public extension {ucc}: IComparable {{")?;
        writeln!(w, "    public bool equals({ucc} other) {{")?;
        // writeln!(w, "        {ucc} other = ({ucc})another;")?;
        write!(w, "        return ")?;
        for (outer_idx, _) in multi_vec.groups().into_iter().enumerate() {
            if outer_idx > 0 {
                write!(w, " && ")?;
            }
            write!(w, "equalsHelper(this.group{outer_idx}, other.group{outer_idx})")?;
        }
        writeln!(w, ";\n    }}")?;
        writeln!(w, "    public bool lessThan({ucc} other) {{")?;
        let len = multi_vec.groups().len();
        for (outer_idx, _) in multi_vec.groups().into_iter().enumerate() {
            if outer_idx < len - 1 {
                write!(w, "        if (!equalsHelper(this.group{outer_idx}, other.group{outer_idx}))\n    ")?;
            }
            writeln!(w, "        return lessThanHelper(this.group{outer_idx}, other.group{outer_idx});")?;
        }
        writeln!(w, "    }}")?;
        writeln!(w, "    public bool lessThanOrEquals({ucc} other) {{")?;
        let len = multi_vec.groups().len();
        for (outer_idx, _) in multi_vec.groups().into_iter().enumerate() {
            if outer_idx < len - 1 {
                write!(w, "        if (!equalsHelper(this.group{outer_idx}, other.group{outer_idx}))\n    ")?;
            }
            writeln!(w, "        return lessThanOrEqualsHelper(this.group{outer_idx}, other.group{outer_idx});")?;
        }
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;


        // TODO although I can kind of give or take nearly, I want clamp_zeros.
        //  So I guess that means I want to implement nearly.


        Ok(())
    }

    fn declare_trait_def<W: Write>(&self, w: &mut W, def: Arc<RawTraitDefinition>) -> anyhow::Result<()> {
        let ucc = def.names.trait_key.as_upper_camel();
        let lsc = def.names.trait_key.as_lower_snake();
        self.emit_comment(w, true, &def.documentation)?;

        // TODO we can re-introduce the interface when this issue is resolved
        //  https://github.com/shader-slang/slang/issues/5954

        // todo alias documentation
        // write!(w, "public interface {ucc}")?;
        // if let TraitArity::Two = def.arity {
        //     write!(w, "<T>")?;
        // }
        // if ucc == "Into" || ucc == "TryInto" {
        //     write!(w, "<Output>")?;
        // }
        // writeln!(w, " {{")?;
        //
        // let output_ty = def.output.read();
        // match *output_ty {
        //     TraitTypeConsensus::AlwaysSelf | TraitTypeConsensus::AllAgree(_, _) => {
        //         // We don't actually output it here
        //         // self.write_type(w, et)?;
        //     }
        //     TraitTypeConsensus::NoVotes | TraitTypeConsensus::Disagreement => {
        //         if ucc != "Into" && ucc != "TryInto" {
        //             writeln!(w, "    associatedtype {ucc}Output;")?;
        //         }
        //     }
        // }
        // write!(w, "    func {lsc}(")?;
        // match def.arity {
        //     TraitArity::Zero => {}
        //     TraitArity::One => {},
        //     TraitArity::Two => write!(w, "other: T")?,
        // }
        // write!(w, ") -> ")?;
        // if ucc == "TryInto" {
        //     write!(w, "Optional<Output>")?;
        // } else {
        //     match *output_ty {
        //         // TODO is "This" a valid type? If so, update 'From' and 'TryFrom' to be rid of redundant associated type
        //         TraitTypeConsensus::AlwaysSelf => write!(w, "This")?,
        //         TraitTypeConsensus::AllAgree(et, _) => self.write_type(w, et)?,
        //         TraitTypeConsensus::NoVotes | TraitTypeConsensus::Disagreement => write!(w, "{ucc}Output")?,
        //     }
        // }
        // writeln!(w, ";\n}}")?;
        //
        // if ucc == "Into" {
        //     writeln!(w, "public interface From<Other> {{")?;
        //     writeln!(w, "    static func from(Other other) -> This;")?;
        //     writeln!(w, "}}")?;
        //     return Ok(())
        // }
        // if ucc == "TryInto" {
        //     writeln!(w, "public interface TryFrom<Other> {{")?;
        //     writeln!(w, "    static func try_from(Other other) -> Optional<This>;")?;
        //     writeln!(w, "}}")?;
        //     return Ok(())
        // }

        let infix_term = match def.arity {
            TraitArity::Zero => None,
            TraitArity::One => Some("PrefixOrPostfix"),
            TraitArity::Two => Some("Infix"),
        };
        let Some(infix_term) = infix_term else { return Ok(()) };

        if let Some(_op) = &self.fancy_infix {
            // TODO work around the empty brace clutter.
            writeln!(w, "public static const {ucc}{infix_term} {lsc} = {ucc}{infix_term}();")?;
            writeln!(w, "public struct {ucc}{infix_term} {{}}")?;
            if let TraitArity::Two = def.arity {
                writeln!(w, "public struct {ucc}{infix_term}Partial<A> {{ internal A a; }}")?;
            }
        }

        Ok(())
    }

    pub(crate) fn declare_trait_impl<W: Write>(
        &self, w: &mut W, impls: Arc<RawTraitImplementation>, already_granted_infix: &mut BTreeSet<&'static str>
    ) -> anyhow::Result<()> {
        let def = &impls.definition;

        let output_ty = impls.return_expr.expression_type();
        let owner_ty = &impls.owner;
        if impls.other_var_params.len() > 1 || impls.other_type_params.len() > 1 {
            bail!("We do not support high arity traits yet");
        }

        let op = def.op.lock().clone();
        let ucc = def.names.trait_key.as_upper_camel();
        let mut lsc = def.names.trait_key.as_lower_snake();
        let mut do_assign_impl = false;
        let mut is_op = false;
        if let Some(op) = op {
            if op.rust_trait_name() == ucc.as_str() {
                is_op = true;
                lsc = op.slang_trait_method().to_string();
                do_assign_impl = def.arity == TraitArity::Two && *owner_ty == output_ty;
            }
        }
        let is_op = is_op;
        let lsc = lsc;
        let do_assign_impl = do_assign_impl;

        let mut var_param = None;
        if !impls.other_var_params.is_empty() {
            let v_param = &impls.other_var_params[0];
            if !impls.other_type_params.is_empty() {
                let ty_param = &impls.other_type_params[0];
                if ty_param != v_param {
                    // TODO I feel like this is a representation problem, need to review and maybe
                    //  refactor the algebraic data types involved here
                    bail!("Type of trait implementation does not agree");
                }
            }
            var_param = Some(v_param);
        }

        if let Some(op) = self.fancy_infix {
            let operator_method = op.slang_trait_method();
            let infix_term = match def.arity {
                TraitArity::Zero => None,
                TraitArity::One => Some("PrefixOrPostfix"),
                TraitArity::Two => Some("Infix"),
            };
            if let Some(infix_term) = infix_term {
                if let TraitParam::Class(mv) = &owner_ty {
                    let n = mv.name();
                    if !is_op {
                        // TODO use the [ForceInline] attribute

                        // write!(w, "public extension ")?;
                        // self.write_type(w, *owner_ty)?;
                        // writeln!(w, " {{")?;
                        if !already_granted_infix.contains(n) {
                            already_granted_infix.insert(n);
                            if let TraitArity::Two = def.arity {
                                writeln!(w, "// Fancy infix trick (first half)")?;
                                write!(w, "public {ucc}{infix_term}Partial<{n}> {operator_method}(")?;
                                self.write_type(w, *owner_ty)?;
                                writeln!(w, " lhs, {ucc}Infix rhs) {{")?;
                                writeln!(w, "    return {ucc}{infix_term}Partial<{n}>(lhs);")?;
                                writeln!(w, "}}")?;
                            }
                        }
                        if let TraitArity::One = def.arity {
                            writeln!(w, "// Fancy postfix trick")?;
                            write!(w, "public ")?;
                            self.write_type(w, output_ty)?;
                            write!(w, " {operator_method}(")?;
                            self.write_type(w, *owner_ty)?;
                            writeln!(w, " lhs, {ucc}{infix_term} rhs)  {{")?;
                            writeln!(w, "    return lhs.{lsc}();\n    }}")?;
                            // if &output_ty == owner_ty {
                            //     writeln!(w, "    // Fancy postfix self-assign")?;
                            //     writeln!(w, "    public {n}& {operator_method}=(const {ucc}{infix_term}& rhs) {{")?;
                            //     writeln!(w, "        this = this.{lsc}();")?;
                            //     writeln!(w, "        return *this;\n    }}")?;
                            // }
                        }
                        // writeln!(w, "}}")?;
                        if let (TraitArity::Two, Some(other_ty)) = (def.arity, var_param) {

                            // write!(w, "public extension {ucc}{infix_term}Partial<")?;
                            // self.write_type(w, *owner_ty)?;
                            // writeln!(w, "> {{")?;
                            writeln!(w, "// Fancy infix trick (second half)")?;
                            write!(w, "public ")?;
                            self.write_type(w, output_ty)?;
                            write!(w, " {operator_method}(")?;
                            write!(w, "{ucc}{infix_term}Partial<")?;
                            self.write_type(w, *owner_ty)?;
                            write!(w, "> lhs, ")?;
                            self.write_type(w, *other_ty)?;
                            writeln!(w, " rhs) {{")?;
                            writeln!(w, "    return lhs.a.{lsc}(rhs);")?;
                            writeln!(w, "}}")?;
                            // writeln!(w, "}}")?;
                        }
                        if let TraitArity::One = def.arity {
                            // writeln!(w, "public extension {ucc}{infix_term} {{")?;
                            writeln!(w, "// Fancy prefix trick")?;
                            write!(w, "public ")?;
                            self.write_type(w, output_ty)?;
                            write!(w, " {operator_method}({ucc}{infix_term} lhs, ")?;
                            self.write_type(w, *owner_ty)?;
                            writeln!(w, " rhs) {{")?;
                            writeln!(w, "    return rhs.{lsc}();")?;
                            writeln!(w, "}}")?;
                            // writeln!(w, "}}")?;
                        }
                    }
                }
            }
        }


        // todo alias documentation
        write!(w, "public extension ")?;
        self.write_type(w, *owner_ty)?;
        // if !is_op {
        //     write!(w, ": {ucc}")?;
        //     if let (TraitArity::Two, Some(var_param)) = (def.arity, var_param) {
        //         write!(w, "<")?;
        //         self.write_type(w, *var_param)?;
        //         write!(w, ">")?;
        //     }
        // }
        writeln!(w, " {{")?;
        // write!(w, "    public typedef ")?;
        // self.write_type(w, output_ty)?;
        // writeln!(w, " {ucc}Output;")?;
        write!(w, "    public ")?;
        self.write_type(w, output_ty)?;
        write!(w, " {lsc}(")?;
        match (def.arity, var_param) {
            (TraitArity::Zero, _) => {}
            (TraitArity::One, _) => {},
            (TraitArity::Two, Some(other_ty)) => {
                self.write_type(w, *other_ty)?;
                write!(w, " other")?;
            }
            _ => panic!("Arity 2 should always have other type"),
        }
        // write!(w, ") -> ")?;
        // self.write_type(w, output_ty)?;
        writeln!(w, ") {{")?;
        // write!(w, ") -> ")?;
        // self.write_type(w, *owner_ty)?;
        // writeln!(w, ".{ucc}Output {{")?;
        for line in impls.lines.iter() {
            match line {
                CommentOrVariableDeclaration::Comment(c) => {
                    self.emit_comment(w, false, c.to_string())?;
                }
                CommentOrVariableDeclaration::VarDec(var_dec) => {
                    let Some(var_dec) = var_dec.upgrade() else { continue };
                    let Some(expr) = &var_dec.expr else { continue };
                    if let Some(c) = &var_dec.comment {
                        self.emit_comment(w, false, c.to_string())?;
                    }
                    let name = var_dec.name.0.to_string();
                    let no = var_dec.name.1;
                    let expr = expr.read();
                    let expr = expr.deref();
                    write!(w, "        ")?;
                    self.write_type(w, expr.expression_type())?;
                    if no == 0 {
                        write!(w, " {name} = ")?;
                    } else {
                        let no = no + 1;
                        write!(w, " {name}_{no} = ")?;
                    }
                    self.write_expression(w, expr, true)?;
                    writeln!(w, ";")?;
                }
            }
        }
        if let Some(c) = &impls.return_comment {
            self.emit_comment(w, false, c.to_string())?;
        }
        write!(w, "        return ")?;
        self.write_expression(w, &impls.return_expr, true)?;
        writeln!(w, ";")?;
        writeln!(w, "    }}\n}}")?;



        // if !do_assign_impl {
        //     return Ok(());
        // }
        //
        // write!(w, "public extension ")?;
        // self.write_type(w, *owner_ty)?;
        // writeln!(w, " {{")?;
        // write!(w, "    public ")?;
        // self.write_type(w, output_ty)?;
        // write!(w, "& {lsc}=(")?;
        // match (def.arity, var_param) {
        //     (TraitArity::Zero, _) => {}
        //     (TraitArity::One, _) => {},
        //     (TraitArity::Two, Some(other_ty)) => {
        //         write!(w, "const ")?;
        //         self.write_type(w, *other_ty)?;
        //         write!(w, "& other")?;
        //     }
        //     _ => panic!("Arity 2 should always have other type"),
        // }
        // writeln!(w, ") {{")?;
        // for line in impls.lines.iter() {
        //     match line {
        //         CommentOrVariableDeclaration::Comment(c) => {
        //             self.emit_comment(w, false, c.to_string())?;
        //         }
        //         CommentOrVariableDeclaration::VarDec(var_dec) => {
        //             let Some(var_dec) = var_dec.upgrade() else { continue };
        //             let Some(expr) = &var_dec.expr else { continue };
        //             if let Some(c) = &var_dec.comment {
        //                 self.emit_comment(w, false, c.to_string())?;
        //             }
        //             let name = var_dec.name.0.to_string();
        //             let no = var_dec.name.1;
        //             let expr = expr.read();
        //             let expr = expr.deref();
        //             write!(w, "        ")?;
        //             self.write_type(w, expr.expression_type())?;
        //             if no == 0 {
        //                 write!(w, " {name} = ")?;
        //             } else {
        //                 let no = no + 1;
        //                 write!(w, " {name}_{no} = ")?;
        //             }
        //             self.write_expression(w, expr, true)?;
        //             writeln!(w, ";")?;
        //         }
        //     }
        // }
        // if let Some(c) = &impls.return_comment {
        //     self.emit_comment(w, false, c.to_string())?;
        // }
        // write!(w, "        *this = ")?;
        // self.write_expression(w, &impls.return_expr, true)?;
        // writeln!(w, ";")?;
        // writeln!(w, "    }}\n}}")?;

        Ok(())
    }

    fn emit_comment<W: Write, S: Into<String>>(&self, w: &mut W, is_documentation: bool, s: S) -> anyhow::Result<()> {
        let slashy = if is_documentation { "/// " } else { "// " };
        let s = s.into();
        let comment = s.trim();
        if comment.is_empty() {
            writeln!(w, "\n{slashy}")?;
            return Ok(());
        }
        let mut comment_iter = comment.split("\n").map(|it| it.trim()).skip_while(|it| it.is_empty()).peekable();
        writeln!(w)?;
        while let Some(line) = comment_iter.next() {
            if line.is_empty() {
                if let Some(next_line) = comment_iter.peek() {
                    if !next_line.is_empty() {
                        writeln!(w, "{slashy}{next_line}")?;
                    }
                }
            } else {
                writeln!(w, "{slashy}{line}")?;
            }
        }
        Ok(())
    }
}

fn swizzle_term(idx: &u8) -> anyhow::Result<&'static str> {
    match *idx {
        0 => Ok("x"),
        1 => Ok("y"),
        2 => Ok("z"),
        3 => Ok("w"),
        _ => bail!("swizzle index out of bounds")
    }
}