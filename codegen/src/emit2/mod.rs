use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::ErrorKind::AlreadyExists;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use anyhow::bail;
use parking_lot::lock_api::RwLockReadGuard;
use parking_lot::RawRwLock;
use tokio::task::JoinSet;
use crate::algebra2::basis::BasisElement;
use crate::algebra2::basis::grades::Grades;
use crate::algebra2::multivector::{MultiVec, MultiVecRepository};
use crate::ast2::datatype::{ExpressionType, MultiVector};
use crate::ast2::traits::{RawTraitDefinition, RawTraitImplementation, TraitImplRegistry, TraitKey, TraitTypeConsensus};

#[derive(Copy, Clone)]
pub enum ExtensiveFile {
    OverwriteAllInOneFile,
    CustomizableStubWithIncludes,
}
#[derive(Copy, Clone)]
pub enum DataTypesVsTraits {
    Adjacent,
    SeparateFolders,
    OneGinormousFile,
}
#[derive(Copy, Clone)]
pub enum DataTypesBelong {
    AllTogether,
    FilePerGrade,
    FilePerType,
    FilePerGradeThenPerType,
}
#[derive(Copy, Clone)]
pub enum TraitDefsBelong {
    AllTogether,
    FilePerArity,
    FilePerDef,
    FilePerArityThenPerDef,
}
#[derive(Copy, Clone)]
pub enum TraitImplsBelong {
    WithTraitDef,
    WithOwnerType,
}
#[derive(Clone)]
pub struct FileOrganizing {
    pub algebra_name: &'static str,
    pub overall_split: DataTypesVsTraits,
    pub data_types: (DataTypesBelong, ExtensiveFile),
    pub trait_defs: (TraitDefsBelong, ExtensiveFile),
    pub trait_impls: TraitImplsBelong,
    pub override_data_types: Arc<HashMap<MultiVector, (DataTypesBelong, ExtensiveFile)>>,
    pub override_trait_defs: Arc<HashMap<TraitKey,    (TraitDefsBelong, ExtensiveFile)>>,
}
impl FileOrganizing {
    pub fn recommended_for_rust(algebra_name: &'static str) -> Self {
        Self {
            algebra_name,
            overall_split: DataTypesVsTraits::Adjacent,
            data_types: (DataTypesBelong::FilePerType, ExtensiveFile::CustomizableStubWithIncludes),
            trait_defs: (TraitDefsBelong::FilePerArityThenPerDef, ExtensiveFile::OverwriteAllInOneFile),
            trait_impls: TraitImplsBelong::WithTraitDef,
            override_data_types: Default::default(),
            override_trait_defs: Default::default(),
        }
    }

    pub fn recommended_for_shaders(algebra_name: &'static str) -> Self {
        Self {
            algebra_name,
            overall_split: DataTypesVsTraits::OneGinormousFile,
            data_types: (DataTypesBelong::AllTogether, ExtensiveFile::OverwriteAllInOneFile),
            trait_defs: (TraitDefsBelong::AllTogether, ExtensiveFile::OverwriteAllInOneFile),
            trait_impls: TraitImplsBelong::WithTraitDef,
            override_data_types: Default::default(),
            override_trait_defs: Default::default(),
        }
    }

    pub async fn go_do_it<P: AsRef<Path> + AsRef<OsStr>, E: AstEmitter, const AntiScalar: BasisElement>(
        self,
        root: P,
        multi_vecs: Arc<MultiVecRepository<AntiScalar>>,
        impls: Arc<TraitImplRegistry>,
    ) -> anyhow::Result<()> {
        let root: &Path = root.as_ref();
        fs::create_dir_all(root)?;
        match self.overall_split {
            DataTypesVsTraits::Adjacent => {
                // TODO scan through all materials and divide them up by their destination
                //  files, then from there we can scan complete list of dependencies for the
                //  imports at top
            }
            DataTypesVsTraits::SeparateFolders => {

            }
            DataTypesVsTraits::OneGinormousFile => {
                let file_name = root
                    .join(Path::new(self.algebra_name))
                    .with_extension(E::file_extension());
                let mut file = File::create(file_name)?;
                for mv in multi_vecs.declarations() {
                    E::emit_multi_vector(&mut file, &self, mv)?;
                }
                //
            }
        }
        Ok(())
    }

    async fn write_file_smart<
        P: AsRef<Path> + AsRef<OsStr>, S: Into<String>, E: AstEmitter, const AntiScalar: BasisElement
    >(
        &self,
        join_set: &mut JoinSet<anyhow::Result<()>>,
        folder_owning_file: P,
        file_name_no_extension: S,
        is_per_type: bool,
        is_per_trait: bool,
        types: Vec<&'static MultiVec<AntiScalar>>,
        trait_declarations: Vec<Arc<RawTraitDefinition>>,
        trait_implementations: Vec<Arc<RawTraitImplementation>>,
    ) -> anyhow::Result<()> {

        let mut use_customizable_stub = false;
        for multi_vec in types.iter() {
            let mv = MultiVector::from(*multi_vec);
            let (_, extensive_file) = match self.override_data_types.get(&mv) {
                None => self.data_types,
                Some(stuff) => *stuff,
            };
            if let ExtensiveFile::CustomizableStubWithIncludes = extensive_file {
                use_customizable_stub = true;
                break;
            }
        }
        if !use_customizable_stub {
            for td in trait_declarations.iter() {
                let k = td.names.trait_key;
                let (_, extensive_file) = match self.override_trait_defs.get(&k) {
                    None => self.trait_defs,
                    Some(stuff) => *stuff,
                };
                if let ExtensiveFile::CustomizableStubWithIncludes = extensive_file {
                    use_customizable_stub = true;
                    break;
                }
            }
        }

        let mut trait_deps = vec![];
        let mut mv_deps = vec![];
        if E::supports_imports() {
            for td in trait_declarations.iter() {
                let out = td.output.read();
                match *out {
                    TraitTypeConsensus::AllAgree(ExpressionType::Class(mv)) => {
                        let mv: Option<&'static MultiVec<AntiScalar>> = mv.into();
                        let mv = mv.expect("Must use correct AntiScalar");
                        mv_deps.push(mv);
                    }
                    _ => {}
                }
            }
            for ti in trait_implementations.iter() {
                for m in ti.multivector_dependencies.iter() {
                    let mv: Option<&'static MultiVec<AntiScalar>> = (*m).into();
                    let mv = mv.expect("Must use correct AntiScalar");
                    mv_deps.push(mv);
                }
                for (_, d) in ti.traits10_dependencies.iter() {
                    trait_deps.push(d.definition.clone());
                }
                for (_, d) in ti.traits11_dependencies.iter() {
                    trait_deps.push(d.definition.clone());
                }
                for (_, d) in ti.traits21_dependencies.iter() {
                    trait_deps.push(d.definition.clone());
                }
                for (_, d) in ti.traits22_dependencies.iter() {
                    trait_deps.push(d.definition.clone());
                }
            }
        }

        let file_name_no_extension = file_name_no_extension.into();
        let file_name = PathBuf::from(&folder_owning_file)
            .join(Path::new(file_name_no_extension.as_str()))
            .with_extension(E::file_extension());
        if !use_customizable_stub || !E::supports_includes() {
            let slf = self.clone();
            join_set.spawn(async move {
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(file_name)?;
                E::emit_comment(
                    &mut file,
                    "AUTO-GENERATED - DO NOT MODIFY BY HAND\n\
                    Changes to this file may be clobbered by code generation at any time."
                )?;
                slf.write_file_dumb::<&mut File, E, AntiScalar>(
                    &mut file, mv_deps, trait_deps, types, trait_declarations, trait_implementations
                ).await
            });
            return Ok(())
        }
        let maybe_stub_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file_name);
        let mut stub_file = match maybe_stub_file {
            Ok(f) => Some(f),
            Err(e) if e.kind() == AlreadyExists => None,
            Err(e) => Err(e)?
        };

        if let Some(stub_file) = &mut stub_file {
            E::emit_comment(
                stub_file,
                "This file may `include` other files in its contents.\n\
                    This file will not be clobbered by code generation, \n\
                    and so can be manually customized or documented by hand.\n\
                    Any `included` files seen across this file WILL be \n\
                    clobbered by code generation.\n\
                    If you accidentally break this file, or the content \n\
                    it relies upon changes, you can regenerate this file \n\
                    from scratch by deleting it first."
            )?;
            self.write_file_dumb::<&mut File, E, AntiScalar>(
                stub_file, mv_deps, trait_deps, vec![], vec![], vec![]
            ).await?;
        }
        for mv in types {
            if let Some(stub_file) = &mut stub_file {
                E::emit_comment(stub_file, "TODO custom documentation")?;
            }
            let included_file_name = if is_per_type {
                let mut n = file_name_no_extension.clone();
                n.push_str("_datatype");
                PathBuf::from(&folder_owning_file)
                    .join(Path::new(n.as_str()))
                    .with_extension(E::file_extension())
            } else {
                let mut n = file_name_no_extension.clone();
                n.push_str("_");
                n.push_str(TraitKey::new(mv.name).as_lower_snake().as_str());
                PathBuf::from(&folder_owning_file)
                    .join(Path::new(file_name_no_extension.as_str()))
                    .with_extension(E::file_extension())
            };
            if let Some(stub_file) = &mut stub_file {
                E::include_file(stub_file, &included_file_name)?;
            }
            let slf = self.clone();
            join_set.spawn(async move {
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(included_file_name)?;
                slf.write_file_dumb::<&mut File, E, AntiScalar>(
                    &mut file, vec![], vec![], vec![mv], vec![], vec![]
                ).await
            });
        }
        for def in trait_declarations {
            if let Some(stub_file) = &mut stub_file {
                E::emit_comment(stub_file, "TODO custom documentation")?;
            }
            let included_file_name = if is_per_trait {
                let mut n = file_name_no_extension.clone();
                n.push_str("_def");
                PathBuf::from(&folder_owning_file)
                    .join(Path::new(n.as_str()))
                    .with_extension(E::file_extension())
            } else {
                let mut n = file_name_no_extension.clone();
                n.push_str("_");
                n.push_str(def.names.trait_key.as_lower_snake().as_str());
                PathBuf::from(&folder_owning_file)
                    .join(Path::new(file_name_no_extension.as_str()))
                    .with_extension(E::file_extension())
            };
            if let Some(stub_file) = &mut stub_file {
                E::include_file(stub_file, &included_file_name)?;
            }
            let slf = self.clone();
            join_set.spawn(async move {
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(included_file_name)?;
                slf.write_file_dumb::<&mut File, E, AntiScalar>(
                    &mut file, vec![], vec![], vec![], vec![def], vec![]
                ).await
            });
        }
        if !trait_implementations.is_empty() {
            let included_file_name = {
                let mut n = file_name_no_extension.clone();
                n.push_str("_impls");
                PathBuf::from(&folder_owning_file)
                    .join(Path::new(file_name_no_extension.as_str()))
                    .with_extension(E::file_extension())
            };
            if let Some(stub_file) = &mut stub_file {
                E::include_file(stub_file, &included_file_name)?;
            }
            let slf = self.clone();
            join_set.spawn(async move {
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(included_file_name)?;
                slf.write_file_dumb::<&mut File, E, AntiScalar>(
                    &mut file, vec![], vec![], vec![], vec![], trait_implementations
                ).await
            });
        }
        Ok(())
    }

    async fn write_file_dumb<W: Write, E: AstEmitter, const AntiScalar: BasisElement>(
        &self,
        mut file: W,
        type_dependencies: Vec<&'static MultiVec<AntiScalar>>,
        trait_dependencies: Vec<Arc<RawTraitDefinition>>,
        types: Vec<&'static MultiVec<AntiScalar>>,
        trait_declarations: Vec<Arc<RawTraitDefinition>>,
        trait_implementations: Vec<Arc<RawTraitImplementation>>,
    ) -> anyhow::Result<()> {
        for dep in type_dependencies {
            E::import_multi_vector(&mut file, self, dep)?;
        }
        for dep in trait_dependencies {
            E::import_trait_def(&mut file, self, dep)?;
        }
        for multi_vec in types {
            E::emit_multi_vector(&mut file, self, multi_vec)?;
        }
        for td in trait_declarations {
            E::emit_trait_def(&mut file, self, td)?;
        }
        for ti in trait_implementations {
            E::emit_trait_impl(&mut file, self, ti)?;
        }
        Ok(())
    }
}

fn folder_of_grades(gr: Grades) -> &'static str {
    let bits = gr.into_bits();
    // Grade 0 takes 1 bit of grades
    // So grade 0 = 0x1
    // Grade 1 = 0x2
    // and NO GRADES that is to say NOT EVEN GRADE 0 is represented as 0x0
    match bits {
        1 => "scalar",
        2 => "vector",
        4 => "bivector",
        8 => "trivector",
        16 => "quadvector",
        32 => "vector_gr5",
        64 => "vector_gr6",
        128 => "vector_gr7",
        256 => "vector_gr8",
        512 => "vector_gr9",
        1024 => "vector_gr10",
        2048 => "vector_gr11",
        4096 => "vector_gr12",
        8192 => "vector_gr13",
        16384 => "vector_gr14",
        32768 => "vector_gr15",
        65536 => "vector_gr16",
        _ => "mixed_grade"
    }
}


impl IdentifierQualifier for FileOrganizing {
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
                let mut gr = Grades::none;
                for el in data_type.elements() {
                    gr |= el.grades();
                }
                path.join(Path::new(folder_of_grades(gr)))
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
                let mut gr = Grades::none;
                for el in data_type.elements() {
                    gr |= el.grades();
                }
                let n = TraitKey::new(data_type.name).as_lower_snake();
                path.join(Path::new(folder_of_grades(gr))).join(Path::new(&n))
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
            TraitDefsBelong::FilePerArity => {
                match trait_def.arity {
                    0 => path.join(Path::new("arity0")),
                    1 => path.join(Path::new("arity1")),
                    2 => path.join(Path::new("arity2")),
                    _ => panic!("High arity traits are not supported yet")
                }
            }
            TraitDefsBelong::FilePerDef => {
                let n = k.as_lower_snake();
                path.join(Path::new(&n))
            }
            TraitDefsBelong::FilePerArityThenPerDef => {
                path = match trait_def.arity {
                    0 => path.join(Path::new("arity0")),
                    1 => path.join(Path::new("arity1")),
                    2 => path.join(Path::new("arity2")),
                    _ => panic!("High arity traits are not supported yet")
                };
                let n = k.as_lower_snake();
                path.join(Path::new(&n))
            }
        }
    }
}


pub trait IdentifierQualifier {
    fn qualifying_path_of_data_type<const AntiScalar: BasisElement>(&self, data_type: &'static MultiVec<AntiScalar>) -> PathBuf;
    fn qualifying_path_of_trait_def(&self, trait_def: Arc<RawTraitDefinition>) -> PathBuf;
}

pub trait AstEmitter {
    fn file_extension() -> &'static str;
    fn supports_includes() -> bool { false }
    fn include_file<W: Write, P: AsRef<Path>>(w: &mut W, p: P) -> anyhow::Result<()> { bail!("Includes are not supported") }
    fn supports_imports() -> bool { false }
    fn import_multi_vector<W: Write, Q: IdentifierQualifier, const AntiScalar: BasisElement>(
        w: &mut W,
        q: &Q,
        multi_vec: &'static MultiVec<AntiScalar>,
    ) -> anyhow::Result<()> { bail!("Imports not supported") }
    fn import_trait_def<W: Write, Q: IdentifierQualifier>(
        w: &mut W,
        q: &Q,
        defs: Arc<RawTraitDefinition>,
    ) -> anyhow::Result<()> { bail!("Imports not supported") }
    fn emit_multi_vector<W: Write, Q: IdentifierQualifier, const AntiScalar: BasisElement>(
        w: &mut W,
        q: &Q,
        multi_vec: &'static MultiVec<AntiScalar>,
    ) -> anyhow::Result<()>;
    fn emit_trait_def<W: Write, Q: IdentifierQualifier>(
        w: &mut W,
        q: &Q,
        defs: Arc<RawTraitDefinition>,
    ) -> anyhow::Result<()>;
    fn emit_trait_impl<W: Write, Q: IdentifierQualifier>(
        w: &mut W,
        q: &Q,
        impls: Arc<RawTraitImplementation>,
    ) -> anyhow::Result<()>;
    fn emit_comment<W: Write, S: Into<String>>(w: &mut W, s: S) -> anyhow::Result<()>;
}










//