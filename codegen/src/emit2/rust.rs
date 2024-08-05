use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;

use anyhow::{bail, Error};
use tokio::task::JoinSet;

use crate::algebra2::basis::BasisElement;
use crate::algebra2::basis::grades::Grades;
use crate::algebra2::multivector::{MultiVec, MultiVecRepository};
use crate::ast2::datatype::{ExpressionType, MultiVector};
use crate::ast2::expressions::{AnyExpression, FloatExpr, IntExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr};
use crate::ast2::traits::{CommentOrVariableDeclaration, RawTraitDefinition, RawTraitImplementation, TraitArity, TraitImplRegistry, TraitKey, TraitTypeConsensus};
use crate::emit2::{AstEmitter, DataTypesBelong, DataTypesVsTraits, sort_trait_impls, TraitDefsBelong};
use crate::utility::CollectResults;

#[derive(Copy, Clone)]
pub struct Rust {
    pub prefer_fancy_infix: bool,
    pub point_based: bool,
    pub censor_grades: bool,
}


impl Rust {

    pub async fn write_crate<P: AsRef<Path>>(crate_folder: P) -> anyhow::Result<()> {
        //
        Ok(())
    }
    pub async fn write_src<P: AsRef<Path>, const AntiScalar: BasisElement>(
        self,
        src_folder: P,
        multi_vecs: Arc<MultiVecRepository<AntiScalar>>,
        impls: Arc<TraitImplRegistry>,
    ) -> anyhow::Result<()> {

        let src_folder = src_folder.as_ref();
        let folder_data = src_folder.join(Path::new("data"));
        let folder_data_impls = src_folder.join(Path::new("data/impls"));
        let folder_traits = src_folder.join(Path::new("traits"));
        let folder_traits_impls = src_folder.join(Path::new("traits/impls"));

        fs::create_dir_all(&folder_data_impls)?;
        fs::create_dir_all(&folder_traits_impls)?;

        let defs = impls.get_defs().await;
        let impls = impls.get_impls().await;
        let mvs = multi_vecs.declarations();


        let mut join_set = JoinSet::new();

        let mut data_mods = HashMap::new();


        for multi_vec in mvs {
            let mv = MultiVector::from(multi_vec);
            let n = mv.name();
            join_set.spawn(async {
                let mut mv_file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(folder_data.join(Path::new(n)).with_extension("rs"))?;
                writeln!(&mut mv_file, "use crate::data::*")?;
                self.emit_multi_vector(&mut mv_file, multi_vec)?;
                writeln!(&mut mv_file, "include!(\"./data/impls/{n}.rs\");")
            });

            if let Some(gr) = mv.grade() {
                if !self.censor_grades {
                    data_mods.entry(format!("grade_{gr}"))
                        .and_modify(|v| v.push(multi_vec))
                        .or_insert(vec![multi_vec]);
                    if gr == 1 {
                        data_mods.entry("base".to_string())
                            .and_modify(|v| v.push(multi_vec))
                            .or_insert(vec![multi_vec]);
                    }
                }
                let as_gr = AntiScalar.grade();
                let (m, j) = if self.point_based {
                    ((as_gr - 1) - gr, gr - 1)
                } else {
                    (gr - 1, (as_gr - 1) - gr)
                };
                data_mods.entry(format!("join_{j}"))
                    .and_modify(|v| v.push(multi_vec))
                    .or_insert(vec![multi_vec]);
                data_mods.entry(format!("meet_{m}"))
                    .and_modify(|v| v.push(multi_vec))
                    .or_insert(vec![multi_vec]);
            }

            use crate::algebra2::basis::grades::*;
            let gr = mv.grades();
            let k_reflection = if self.point_based {
                point_based_k_reflections::<AntiScalar>()
            } else {
                plane_based_k_reflections
            }.into_iter()
                .enumerate()
                .find(|(i, it)| it.contains(gr))
                .map(|(i, _)| i);
            if let Some(i) = k_reflection {
                data_mods.entry(format!("k_reflection_{i}"))
                    .and_modify(|v| v.push(multi_vec))
                    .or_insert(vec![multi_vec]);
            }
        }

        for td in defs {
            let k = td.names.trait_key;
            let n = k.as_upper_camel();
            join_set.spawn(async {
                let mut mv_file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(folder_traits.join(Path::new(n.as_str())).with_extension("rs"))?;
                writeln!(&mut mv_file, "use crate::data::*")?;
                self.emit_trait_def(&mut mv_file, td)?;
                writeln!(&mut mv_file, "include!(\"./traits/impls/{n}.rs\");")
            });
        }


        let mut impl_files: HashMap<String, Vec<Arc<RawTraitImplementation>>> = HashMap::new();

        for i in impls {
            let k = i.definition.names.trait_key;
            let (folder, name) = match k.as_upper_camel().as_str() {
                "From" | "Add" | "Sub" | "Mul" | "Div"
                | "Shl" | "Shr" | "BitAnd" | "BitOr" | "BitXor"
                | "Neg" | "Not" => {
                    let ExpressionType::Class(mv) = i.owner else { continue };
                    let n = mv.name();
                    ("data", n.to_string())
                }
                _ => ("traits", k.as_upper_camel()),
            };
            let i2 = i.clone();
            impl_files.entry(format!("{folder}/impls/{name}.rs"))
                .and_modify(move |v| v.push(i2))
                .or_insert(vec![i]);
        }


        for (file_path, mut impls) in impl_files {
            join_set.spawn(async move {
                sort_trait_impls(&mut impls, HashSet::new())?;
                let file_path = src_folder.join(Path::new(file_path.as_str())).with_extension("rs");
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(file_path)?;
                for i in impls {
                    self.emit_trait_impl(&mut file, i)?;
                }
            });
        }

        join_set.spawn(async move {

            let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(src_folder.join(Path::new("data.rs")))?;
            for (the_mod, mvs) in data_mods {
                writeln!(&mut file, "pub mod {the_mod}")
            }
        });

        join_set.collect_results().await
    }



    fn write_type<W: Write>(&self, w: &mut W, data_type: ExpressionType) -> anyhow::Result<()> {
        match data_type {
            ExpressionType::Int(i) => write!(w, "usize")?,
            ExpressionType::Float(f) => write!(w, "f32")?,
            ExpressionType::Vec2(v) => write!(w, "Simd32x2")?,
            ExpressionType::Vec3(v) => write!(w, "Simd32x3")?,
            ExpressionType::Vec4(v) => write!(w, "Simd32x4")?,
            ExpressionType::Class(mv) => {
                let n = mv.name();
                write!(w, "{n}")?;
            }
        }
        Ok(())
    }

    fn write_expression<W: Write>(&self, w: &mut W, expr: &AnyExpression) -> anyhow::Result<()> {
        match expr {
            AnyExpression::Int(e) => self.write_int(w, e)?,
            AnyExpression::Float(e) => self.write_float(w, e)?,
            AnyExpression::Vec2(e) => self.write_vec2(w, e)?,
            AnyExpression::Vec3(e) => self.write_vec3(w, e)?,
            AnyExpression::Vec4(e) => self.write_vec4(w, e)?,
            AnyExpression::Class(e) => self.write_multi_vec(w, e)?,
        }
        Ok(())
    }

    fn write_int<W: Write>(&self, w: &mut W, expr: &IntExpr) -> anyhow::Result<()> {
        match expr {
            IntExpr::Variable(v) => {
                let name = &v.decl.name.0;
                let mut no = v.decl.name.1;
                if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    no += 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            IntExpr::Literal(l) => {
                write!(w, "{l}")?;
            }
            IntExpr::TraitInvoke10ToInt(t, mv) => {
                let n = mv.name();
                let method = t.as_lower_snake();
                write!(w, "{n}::{method}()")?;
            }
        }
        Ok(())
    }

    fn write_float<W: Write>(&self, w: &mut W, expr: &FloatExpr) -> anyhow::Result<()> {
        match expr {
            FloatExpr::Variable(v) => {
                let name = &v.decl.name.0;
                let mut no = v.decl.name.1;
                if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    no += 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            FloatExpr::Literal(l) => {
                write!(w, "{l}")?;
            }
            FloatExpr::AccessVec2(v, i) => {
                self.write_vec2(w, v.as_ref())?;
                write!(w, "[{i}]")?;
            }
            FloatExpr::AccessVec3(v, i) => {
                self.write_vec3(w, v.as_ref())?;
                write!(w, "[{i}]")?;
            }
            FloatExpr::AccessVec4(v, i) => {
                self.write_vec4(w, v.as_ref())?;
                write!(w, "[{i}]")?;
            }
            FloatExpr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                write!(w, ".group{i}()")?;
            }
            FloatExpr::AccessMultiVecFlat(mv, i) => {
                self.write_multi_vec(w, mv)?;
                write!(w, "[{i}]")?;
            }
            FloatExpr::TraitInvoke11ToFloat(t, arg) => {
                self.write_multi_vec(w, arg)?;
                let method = t.as_lower_snake();
                write!(w, ".{method}()")?;
            }
            FloatExpr::Product(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty product that should have been simplified");
                }
                if v.len() > 1 {
                    write!(w, "(")?;
                }
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " * ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_float(w, e)?;
                }
                if v.len() > 1 {
                    write!(w, ")")?;
                }
            }
            FloatExpr::Sum(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                if v.len() > 1 {
                    write!(w, "(")?;
                }
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " + ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_float(w, e)?;
                }
                if v.len() > 1 {
                    write!(w, ")")?;
                }
            }
            FloatExpr::Divide(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty division that should have been simplified");
                }
                if v.len() > 1 {
                    write!(w, "(")?;
                }
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " / ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    // TODO but division is not associative, so I should reconsider if
                    //  FloatExpr::Divide should contain a Vec to begin with
                    self.write_float(w, e)?;
                }
                if v.len() > 1 {
                    write!(w, ")")?;
                }
            }
            FloatExpr::Pow(a, b) => {
                write!(w, "f32::pow(")?;
                self.write_float(w, a.as_ref())?;
                write!(w, ", ")?;
                self.write_float(w, b.as_ref())?;
                write!(w, ")")?;
            }
        }
        Ok(())
    }

    fn write_vec2<W: Write>(&self, w: &mut W, expr: &Vec2Expr) -> anyhow::Result<()> {
        match expr {
            Vec2Expr::Variable(v) => {
                let name = &v.decl.name.0;
                let mut no = v.decl.name.1;
                if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    no += 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            Vec2Expr::Gather1(f) => {
                write!(w, "Simd32x2::from(")?;
                self.write_float(w, f)?;
                write!(w, ")")?;
            }
            Vec2Expr::Gather2(f0, f1) => {
                write!(w, "Simd32x2::from([")?;
                self.write_float(w, f0)?;
                write!(w, ", ")?;
                self.write_float(w, f1)?;
                write!(w, "])")?;
            }
            Vec2Expr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                write!(w, ".group{i}()")?;
            }
            Vec2Expr::Product(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty product that should have been simplified");
                }
                if v.len() > 1 {
                    write!(w, "(")?;
                }
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " * ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec2(w, e)?;
                }
                if v.len() > 1 {
                    write!(w, ")")?;
                }
            }
            Vec2Expr::Sum(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                if v.len() > 1 {
                    write!(w, "(")?;
                }
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " + ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec2(w, e)?;
                }
                if v.len() > 1 {
                    write!(w, ")")?;
                }
            }
            Vec2Expr::SwizzleVec2(box v, i0, i1) => {
                write!(w, "swizzle!(")?;
                self.write_vec2(w, v)?;
                write!(w, ", {i0}, {i1})")?;
            }
        }
        Ok(())
    }

    fn write_vec3<W: Write>(&self, w: &mut W, expr: &Vec3Expr) -> anyhow::Result<()> {
        match expr {
            Vec3Expr::Variable(v) => {
                let name = &v.decl.name.0;
                let mut no = v.decl.name.1;
                if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    no += 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            Vec3Expr::Gather1(f) => {
                write!(w, "Simd32x3::from(")?;
                self.write_float(w, f)?;
                write!(w, ")")?;
            }
            Vec3Expr::Gather3(f0, f1, f2) => {
                write!(w, "Simd32x3::from([")?;
                self.write_float(w, f0)?;
                write!(w, ", ")?;
                self.write_float(w, f1)?;
                write!(w, ", ")?;
                self.write_float(w, f2)?;
                write!(w, "])")?;
            }
            Vec3Expr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                write!(w, ".group{i}()")?;
            }
            Vec3Expr::Product(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty product that should have been simplified");
                }
                if v.len() > 1 {
                    write!(w, "(")?;
                }
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " * ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec3(w, e)?;
                }
                if v.len() > 1 {
                    write!(w, ")")?;
                }
            }
            Vec3Expr::Sum(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                if v.len() > 1 {
                    write!(w, "(")?;
                }
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " + ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec3(w, e)?;
                }
                if v.len() > 1 {
                    write!(w, ")")?;
                }
            }
            Vec3Expr::SwizzleVec3(box v, i0, i1, i2) => {
                write!(w, "swizzle!(")?;
                self.write_vec3(w, v)?;
                write!(w, ", {i0}, {i1}, {i2})")?;
            }
        }
        Ok(())
    }

    fn write_vec4<W: Write>(&self, w: &mut W, expr: &Vec4Expr) -> anyhow::Result<()> {
        match expr {
            Vec4Expr::Variable(v) => {
                let name = &v.decl.name.0;
                let mut no = v.decl.name.1;
                if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    no += 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            Vec4Expr::Gather1(f) => {
                write!(w, "Simd32x4::from(")?;
                self.write_float(w, f)?;
                write!(w, ")")?;
            }
            Vec4Expr::Gather4(f0, f1, f2, f3) => {
                write!(w, "Simd32x4::from([")?;
                self.write_float(w, f0)?;
                write!(w, ", ")?;
                self.write_float(w, f1)?;
                write!(w, ", ")?;
                self.write_float(w, f2)?;
                write!(w, ", ")?;
                self.write_float(w, f3)?;
                write!(w, "])")?;
            }
            Vec4Expr::AccessMultiVecGroup(mv, i) => {
                self.write_multi_vec(w, mv)?;
                write!(w, ".group{i}()")?;
            }
            Vec4Expr::Product(v) => {
                if v.is_empty() {
                    // TODO these sorts of cases should really be handled by making Vec4Expr::Product
                    //  (and other similar Expr types obviously) have an additional element
                    //  that is not inside the Vec, so that it can never be empty. Heck... maybe
                    //  even 2 elements. But we'll see.
                    bail!("Attempted to write an empty product that should have been simplified");
                }
                if v.len() > 1 {
                    write!(w, "(")?;
                }
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " * ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec4(w, e)?;
                }
                if v.len() > 1 {
                    write!(w, ")")?;
                }
            }
            Vec4Expr::Sum(v) => {
                if v.is_empty() {
                    bail!("Attempted to write an empty sum that should have been simplified");
                }
                if v.len() > 1 {
                    write!(w, "(")?;
                }
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, " + ")?;
                    }
                    // This recursion is unlikely to cause a stack overflow,
                    // because expression simplification flattens out associative operations.
                    self.write_vec4(w, e)?;
                }
                if v.len() > 1 {
                    write!(w, ")")?;
                }
            }
            Vec4Expr::SwizzleVec4(box v, i0, i1, i2, i3) => {
                write!(w, "swizzle!(")?;
                self.write_vec4(w, v)?;
                write!(w, ", {i0}, {i1}, {i2}, {i3})")?;
            }
        }
        Ok(())
    }

    fn write_multi_vec<W: Write>(&self, w: &mut W, expr: &MultiVectorExpr) -> anyhow::Result<()> {
        let mv = expr.mv_class;
        match &*expr.expr {
            MultiVectorVia::Variable(v) => {
                let name = &v.decl.name.0;
                let mut no = v.decl.name.1;
                if no == 0 {
                    write!(w, "{name}")?;
                } else {
                    no += 1;
                    write!(w, "{name}_{no}")?;
                }
            }
            MultiVectorVia::Construct(v) => {
                let n = mv.name();
                write!(w, "{n}::from_groups(")?;
                let groups = mv.groups();
                for (i, g) in v.iter().enumerate() {
                    if i > 0 {
                        write!(w, ", ")?;
                    }
                    write!(w, "/* ")?;
                    for (i, el) in groups[i].into_vec().into_iter().enumerate() {
                        if i > 0 {
                            write!(w, ", ")?;
                        }
                        write!(w, "{el}")?;
                    }
                    write!(w, " */")?;
                    match g {
                        MultiVectorGroupExpr::JustFloat(f) => self.write_float(w, f)?,
                        MultiVectorGroupExpr::Vec2(g) => self.write_vec2(w, g)?,
                        MultiVectorGroupExpr::Vec3(g) => self.write_vec3(w, g)?,
                        MultiVectorGroupExpr::Vec4(g) => self.write_vec4(w, g)?,
                    }
                }
                write!(w, ")")?;
            }
            MultiVectorVia::TraitInvoke11ToClass(t, arg) => {
                self.write_multi_vec(w, arg)?;
                let method = t.as_lower_snake();
                write!(w, ".{method}()")?;
            }
            MultiVectorVia::TraitInvoke21ToClass(t, arg, mv) => {
                self.write_multi_vec(w, arg)?;
                let method = t.as_lower_snake();
                let b = mv.name();
                write!(w, ".{method}::<{b}>()")?;
            }
            MultiVectorVia::TraitInvoke22ToClass(t, a, b) => {
                // TODO fancy infix
                self.write_multi_vec(w, a)?;
                let method = t.as_lower_snake();
                write!(w, ".{method}(")?;
                self.write_multi_vec(w, b)?;
                write!(w, ")")?;
            }
        }
        Ok(())
    }

    fn supports_includes(&self) -> bool { true }
    fn include_file<W: Write, P: AsRef<Path>>(&self, w: &mut W, p: P) -> anyhow::Result<()> {
        let p = p.as_ref();
        let path_str = p.to_string_lossy().replace("\\", "/");
        writeln!(w, "include!(\"{path_str}\");")?;
        Ok(())
    }
    fn supports_imports(&self) -> bool { true }
    fn import_multi_vector<W: Write, const AntiScalar: BasisElement>(
        &self, w: &mut W, multi_vec: &'static MultiVec<AntiScalar>
    ) -> anyhow::Result<()> {
        let n = multi_vec.name;
        let p = q.qualifying_path_of_data_type(multi_vec);
        let path_str = p.to_string_lossy().replace("/", "::").replace("\\", "::");
        writeln!(w, "use crate::{path_str}::{n};")?;
        Ok(())
    }
    fn import_trait_def<W: Write>(
        &self, w: &mut W, def: Arc<RawTraitDefinition>
    ) -> anyhow::Result<()> {
        let ucc = def.names.trait_key.as_upper_camel();
        let p = q.qualifying_path_of_trait_def(def);
        let path_str = p.to_string_lossy().replace("/", "::");
        writeln!(w, "use crate::{path_str}::{ucc};")?;
        Ok(())
    }

    fn qualifying_path_of_data_type<const AntiScalar: BasisElement>(&self, data_type: &'static MultiVec<AntiScalar>) -> PathBuf {
        let mut path = match self.overall_split {
            DataTypesVsTraits::Adjacent => PathBuf::new(),
            DataTypesVsTraits::SeparateFolders => Path::new("data").to_path_buf(),
            DataTypesVsTraits::OneGinormousFile => PathBuf::new(),
        };
        let mv = MultiVector::from(data_type);
        let (belong, _) = match self.override_data_types.get(&mv) {
            None => self.data_types,
            Some(stuff) => *stuff,
        };
        return match belong {
            DataTypesBelong::AllTogether => path,
            DataTypesBelong::FilePerGrade => {
                path.join(Path::new(folder_of_grades::<AntiScalar>(data_type.grades)))
            }
            DataTypesBelong::FilePerType => {
                // Hi if you are reading this line of code because you defined
                // a MultiVec with a non-UpperCamelCase name and the error said
                // something about TraitKeys, but you debugged your way here,
                // sorry for the inconvenience. I'll reorganize camel/snake
                // conversions eventually.
                let n = TraitKey::new(data_type.name).as_lower_snake();
                path.join(Path::new(&n))
            }
            DataTypesBelong::FilePerGradeThenPerType => {
                let n = TraitKey::new(data_type.name).as_lower_snake();
                path.join(Path::new(folder_of_grades::<AntiScalar>(data_type.grades))).join(Path::new(&n))
            }
        }
    }

    fn qualifying_path_of_trait_def(&self, trait_def: Arc<RawTraitDefinition>) -> PathBuf {
        let mut path = match self.overall_split {
            DataTypesVsTraits::Adjacent => PathBuf::new(),
            DataTypesVsTraits::SeparateFolders => Path::new("traits").to_path_buf(),
            DataTypesVsTraits::OneGinormousFile => PathBuf::new(),
        };
        let k = trait_def.names.trait_key;
        let (belong, _) = match self.override_trait_defs.get(&k) {
            None => self.trait_defs,
            Some(stuff) => *stuff,
        };
        match belong {
            TraitDefsBelong::AllTogether => path,
            TraitDefsBelong::FilePerArity => path.join(Path::new(trait_def.arity.as_str())),
            TraitDefsBelong::FilePerDef => {
                let n = k.as_lower_snake();
                path.join(Path::new(&n))
            }
            TraitDefsBelong::FilePerArityThenPerDef => {
                path = path.join(Path::new(trait_def.arity.as_str()));
                let n = k.as_lower_snake();
                path.join(Path::new(&n))
            }
        }
    }
}


// TODO allow grouping by k-reflection instead of grades
fn folder_of_grades<const AntiScalar: BasisElement>(gr: Grades) -> &'static str {
    let bits = gr.into_bits();
    // Grade 0 takes 1 bit of grades
    // So grade 0 = 0x1
    // Grade 1 = 0x2
    // and NO GRADES that is to say NOT EVEN GRADE 0 is represented as 0x0
    // match bits {
    //     1 => "scalar",
    //     2 => "vector",
    //     4 => "bivector",
    //     8 => "trivector",
    //     16 => "quadvector",
    //     32 => "vector_gr5",
    //     64 => "vector_gr6",
    //     128 => "vector_gr7",
    //     256 => "vector_gr8",
    //     512 => "vector_gr9",
    //     1024 => "vector_gr10",
    //     2048 => "vector_gr11",
    //     4096 => "vector_gr12",
    //     8192 => "vector_gr13",
    //     16384 => "vector_gr14",
    //     32768 => "vector_gr15",
    //     65536 => "vector_gr16",
    //     _ => "mixed_grade"
    // }
    let d = AntiScalar.signature().bits().count_ones();
    match bits {
        1 if d < 10 => "vector_0",
        2 if d < 10 => "vector_1",
        4 if d < 10 => "vector_2",
        8 if d < 10 => "vector_3",
        16 if d < 10 => "vector_4",
        32 if d < 10 => "vector_5",
        64 if d < 10 => "vector_6",
        128 if d < 10 => "vector_7",
        256 if d < 10 => "vector_8",
        512 if d < 10 => "vector_9",
        1 => "vector_00",
        2 => "vector_01",
        4 => "vector_02",
        8 => "vector_03",
        16 => "vector_04",
        32 => "vector_05",
        64 => "vector_06",
        128 => "vector_07",
        256 => "vector_08",
        512 => "vector_09",
        1024 => "vector_10",
        2048 => "vector_11",
        4096 => "vector_12",
        8192 => "vector_13",
        16384 => "vector_14",
        32768 => "vector_15",
        65536 => "vector_16",
        _ => "vector_mixed"
    }
}




impl AstEmitter for Rust {
    fn file_extension() -> &'static str { "rs" }

    fn emit_multi_vector<W: Write, const AntiScalar: BasisElement>(
        &self, w: &mut W, multi_vec: &'static MultiVec<AntiScalar>
    ) -> anyhow::Result<()> {
        let name = TraitKey::new(multi_vec.name);
        let ucc = name.as_upper_camel();
        let lcc = name.as_lower_camel();
        let lsc = name.as_lower_snake();
        let ssc = name.as_screaming_snake();
        // TODO built in documentation, statistics, and traits that output this type
        writeln!(w, "/// TODO documentation")?;

        // TODO special traits like serde and bytemuck etc
        writeln!(w, "#[derive(Clone, Copy)]")?;
        writeln!(w, "pub union {ucc} {{")?;
        writeln!(w, "    groups: {ucc}Groups,")?;
        write!(w, "    /// ")?;
        let mut total_len = 0;
        for (i, g) in  multi_vec.groups().into_iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            }
            let mut g = g.into_vec();
            while g.len() < 4 {
                g.push(BasisElement::zero());
            }
            for (i, el) in g.into_iter().enumerate() {
                if i > 0 {
                    write!(w, ", ")?;
                }
                write!(w, "{el}")?;
            }
            total_len += 4;
        }
        writeln!(w, "\n    elements: [f32; {total_len}],")?;
        writeln!(w, "}}")?;

        // TODO special traits like serde and bytemuck etc
        writeln!(w, "#[derive(Clone, Copy)]")?;
        writeln!(w, "pub struct {ucc}Groups {{")?;
        for (g, group) in multi_vec.groups().into_iter().enumerate() {
            write!(w, "    /// ")?;
            for (i, el) in group.into_iter().enumerate() {
                if i > 0 {
                    write!(w, ", ")?;
                }
                write!(w, "{el}")?;
            }
            write!(w, "\n    g{g}: ")?;
            self.write_type(w, group.expr_type())?;
            writeln!(w, ",")?;
        }
        writeln!(w, "}}")?;

        writeln!(w, "impl {ucc} {{")?;
        writeln!(w, "#[allow(clippy::too_many_arguments)]")?;
        writeln!(w, "pub const fn from_elements(")?;
        for (i, el) in multi_vec.elements().into_iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            } else {
                write!(w, "    ")?;
            }
            write!(w, "{el}: f32")?;
        }
        writeln!(w, "\n) -> Self {{")?;
        write!(w, "    Self {{ elements: [")?;
        for (i, g) in multi_vec.groups().into_iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            }
            let mut count = 0;
            for (i, el) in g.into_iter().enumerate() {
                if i > 0 {
                    write!(w, ", ")?;
                }
                write!(w, "{el}")?;
                count += 1;
            }
            while count < 4 {
                write!(w, ", 0.0")?;
                count += 1;
            }
        }
        writeln!(w, "] }};")?;
        writeln!(w, "}}")?;
        writeln!(w, "pub const fn from_groups(")?;
        for (i, g) in multi_vec.groups().into_iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            } else {
                write!(w, "    ")?;
            }
            write!(w, "g{i}: ")?;
            self.write_type(w, g.expr_type())?;
        }
        writeln!(w, "\n) -> Self {{")?;
        writeln!(w, "    Self {{\n        groups: {ucc}Groups {{")?;
        for (i, _) in multi_vec.groups().into_iter().enumerate() {
            if i > 0 {
                write!(w, ", ")?;
            } else {
                write!(w, "            ")?;
            }
            write!(w, "g{i}")?;
        }
        writeln!(w, "\n        }}\n    }}\n}}")?;

        for (i, g) in multi_vec.groups().into_iter().enumerate() {
            let t = g.expr_type();

            writeln!(w, "#[inline(always)]")?;
            write!(w, "pub fn group{i}(&self) -> ")?;
            self.write_type(w, t)?;
            writeln!(w, " {{")?;
            writeln!(w, "    unsafe {{ self.groups.g{i} }}")?;
            writeln!(w, "}}")?;

            writeln!(w, "#[inline(always)]")?;
            write!(w, "pub fn group{i}_mut(&mut self) -> &mut ")?;
            self.write_type(w, t)?;
            writeln!(w, " {{")?;
            writeln!(w, "    unsafe {{ &mut self.groups.g{i} }}")?;
            writeln!(w, "}}")?;
        }
        writeln!(w, "}}")?;





        let element_count = multi_vec.elements().len();
        write!(w, "const {ssc}_INDEX_REMAP: [usize; {element_count}] = [")?;
        let mut i = 0;
        for (j, g) in multi_vec.groups().into_iter().enumerate() {
            if j > 0 {
                write!(w, ", ")?;
            }
            let g = g.into_vec();
            for (k, _) in g.into_iter().enumerate() {
                if k > 0 {
                    write!(w, ", ")?;
                }
                write!(w, "{i}")?;
                i += 1;
            }
            i += 4 - g.len();
        }
        writeln!(w, "];")?;


        writeln!(w, "impl std::ops::Index<usize> for {ucc} {{")?;
        writeln!(w, "type Output = f32;")?;
        writeln!(w, "fn index(&self, index: usize) -> &Self::Output {{")?;
        writeln!(w, "    unsafe {{ &self.elements[{ssc}_INDEX_REMAP[index]] }}")?;
        writeln!(w, "}}\n}}")?;

        writeln!(w, "impl std::ops::IndexMut<usize> for {ucc} {{")?;
        writeln!(w, "fn index_mut(&mut self, index: usize) -> &mut Self::Output {{")?;
        writeln!(w, "    unsafe {{ &mut self.elements[{ssc}_INDEX_REMAP[index]] }}")?;
        writeln!(w, "}}\n}}")?;


        writeln!(w, "impl std::convert::From<{ucc}> for [f32; {element_count}] {{")?;
        writeln!(w, "fn from(vector: {ucc}) -> Self {{")?;
        write!(w, "    unsafe {{\n        [")?;
        let mut i = 0;
        for (j, g) in multi_vec.groups().into_iter().enumerate() {
            if j > 0 {
                write!(w, ", ")?;
            }
            let g = g.into_vec();
            for (k, _) in g.into_iter().enumerate() {
                if k > 0 {
                    write!(w, ", ")?;
                }
                write!(w, "vector.elements[{i}]")?;
                i += 1;
            }
            i += 4 - g.len();
        }
        writeln!(w, "] }}\n    }}\n}}")?;


        writeln!(w, "impl std::convert::From<[f32; {element_count}]> for {ucc} {{")?;
        writeln!(w, "    fn from(array: [f32; {element_count}]) -> Self {{")?;
        write!(w, "        Self {{ elements: [")?;
        let mut i = 0;
        for (j, g) in multi_vec.groups().into_iter().enumerate() {
            if j > 0 {
                write!(w, ", ")?;
            }
            let g = g.into_vec();
            for (k, _) in g.into_iter().enumerate() {
                if k > 0 {
                    write!(w, ", ")?;
                }
                let l = j + k;
                write!(w, "array[{l}]")?;
                i += 1;
            }
            while i % 4 > 0 {
                write!(w, ", 0.0")?;
                i += 1;
            }
        }
        writeln!(w, "] }}\n    }}\n}}")?;


        writeln!(w, "impl std::fmt::Debug for {ucc} {{")?;
        writeln!(w, "fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {{")?;
        writeln!(w, "    formatter.debug_struct(\"{ucc}\")")?;
        for (i, el) in multi_vec.elements().into_iter().enumerate() {
            writeln!(w, "        .field(\"{el}\", &self[{i}])")?;
        }
        writeln!(w, "        .finish()\n}}\n}}")?;



        Ok(())
    }

    fn emit_trait_def<W: Write>(
        &self, w: &mut W, def: Arc<RawTraitDefinition>
    ) -> anyhow::Result<()> {
        let ucc = def.names.trait_key.as_upper_camel();
        let lsc = def.names.trait_key.as_lower_snake();
        self.emit_comment(w, true, &def.documentation)?;
        // todo alias documentation
        write!(w, "pub trait {ucc}")?;
        if let TraitArity::Two = def.arity {
            write!(w, "<T>")?;
        }
        writeln!(w, " {{")?;

        let output_ty = def.output.read();
        match *output_ty {
            TraitTypeConsensus::AlwaysSelf | TraitTypeConsensus::AllAgree(_, _) => {
                // We don't actually output it here
                // self.write_type(w, et)?;
            }
            TraitTypeConsensus::NoVotes | TraitTypeConsensus::Disagreement => {
                writeln!(w, "    type Output;")?;
            }
        }
        write!(w, "    fn {lsc}(")?;
        match def.arity {
            TraitArity::Zero => {}
            TraitArity::One => write!(w, "self")?,
            TraitArity::Two => write!(w, "self, other: T")?,
        }
        write!(w, ") -> ")?;
        match *output_ty {
            TraitTypeConsensus::AlwaysSelf => write!(w, "Self")?,
            TraitTypeConsensus::AllAgree(et, _) => self.write_type(w, et)?,
            TraitTypeConsensus::NoVotes | TraitTypeConsensus::Disagreement => write!(w, "Self::Output")?,
        }
        writeln!(w, ";\n}}")?;

        // TODO fancy infix


        Ok(())
    }

    fn emit_trait_impl<W: Write>(
        &self, w: &mut W, impls: Arc<RawTraitImplementation>
    ) -> anyhow::Result<()> {
        let def = &impls.definition;
        let ucc = def.names.trait_key.as_upper_camel();
        let lsc = def.names.trait_key.as_lower_snake();
        let output_kind = def.output.read();
        let output_ty = impls.return_expr.expression_type();
        let owner_ty = &impls.owner;
        if impls.other_var_params.len() > 1 || impls.other_type_params.len() > 1 {
            bail!("We do not support high arity traits yet");
        }
        let mut var_param = None;
        if !impls.other_var_params.is_empty() {
            let ty_param = &impls.other_type_params[0];
            let v_param = &impls.other_var_params[0];
            if ty_param != v_param {
                // TODO I feel like this is a representation problem, need to review and maybe
                //  refactor the algebraic data types involved here
                bail!("Type of trait implementation does not agree");
            }
            var_param = Some(v_param);
        }
        // todo alias documentation
        write!(w, "impl {ucc}")?;
        if let (TraitArity::Two, Some(var_param)) = (def.arity, var_param) {
            write!(w, "<")?;
            self.write_type(w, *var_param)?;
            write!(w, ">")?;
        }
        write!(w, " for ")?;
        self.write_type(w, *owner_ty)?;
        writeln!(w, " {{")?;
        if let TraitTypeConsensus::Disagreement = output_kind.deref() {
            write!(w, "    type Output = ")?;
            self.write_type(w, output_ty)?;
            writeln!(w, ";")?;
        }
        write!(w, "    fn {lsc}(")?;
        match (def.arity, var_param) {
            (TraitArity::Zero, _) => {}
            (TraitArity::One, _) => write!(w, "self")?,
            (TraitArity::Two, Some(other_ty)) => {
                write!(w, "self, other: ")?;
                self.write_type(w, *other_ty)?;
            }
            _ => panic!("Arity 2 should always have other type")
        }
        write!(w, ") -> ")?;
        match output_kind.deref() {
            TraitTypeConsensus::AlwaysSelf => write!(w, "Self")?,
            TraitTypeConsensus::Disagreement => write!(w, "Self::Output")?,
            TraitTypeConsensus::AllAgree(mv, _) => self.write_type(w, *mv)?,
            TraitTypeConsensus::NoVotes => {
                // Currently, we have no use for traits that do not return values
                bail!("Unsupported or invalid trait def implementation: {ucc} for {owner_ty:?}");
            }
        }
        writeln!(w, " {{")?;
        for line in impls.lines.iter() {
            match line {
                CommentOrVariableDeclaration::Comment(c) => {
                    self.emit_comment(w, false, c.to_string())?;
                }
                CommentOrVariableDeclaration::VarDec(var_dec) => {
                    let Some(expr) = &var_dec.expr else { continue };
                    if let Some(c) = &var_dec.comment {
                        self.emit_comment(w, false, c.to_string())?;
                    }
                    let name = var_dec.name.0.to_string();
                    let mut no = var_dec.name.1;
                    if no == 0 {
                        write!(w, "let {name} = ")?;
                    } else {
                        no += 1;
                        write!(w, "let {name}_{no} = ")?;
                    }
                    self.write_expression(w, expr)?;
                    writeln!(w, ";")?;
                }
            }
        }
        if let Some(c) = &impls.return_comment {
            self.emit_comment(w, false, c.to_string())?;
        }
        write!(w, "        return ")?;
        self.write_expression(w, &impls.return_expr)?;
        writeln!(w, ";")?;
        writeln!(w, "    }}\n}}")?;
        Ok(())
    }

    fn emit_comment<W: Write, S: Into<String>>(
        &self, w: &mut W, is_documentation: bool, s: S
    ) -> anyhow::Result<()> {
        let slashy = if is_documentation { "/// " } else { "// " };
        let s = s.into();
        let comment = s.trim();
        if comment.is_empty() {
            writeln!(w, "\n{slashy}")?;
            return Ok(())
        }
        let mut comment_iter = comment.split("\n")
            .map(|it| it.trim())
            .skip_while(|it| it.is_empty())
            .peekable();
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

    fn format_file<P: AsRef<Path>>(&self, p: P) -> anyhow::Result<()> {
        let mut cmd = Command::new("rustfmt");
        cmd.arg(p.as_ref().to_string_lossy().to_string());
        match cmd.spawn() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e))
        }
    }
}