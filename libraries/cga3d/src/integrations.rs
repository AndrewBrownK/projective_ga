
#[cfg(feature = "slang")]
pub mod slang {

    use std::path::Path;
    use std::process::Command;

    pub const CGA3D_SLANG: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/integrations/slang/cga3d.slang");
    pub const CGA3D_SLANG_MODULE: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/integrations/slang-module/cga3d.slang-module");

    /// This requires slangc to be installed and added to your path
    /// Warning, this can be an extremely slow operation. Give it time.
    pub fn compile_binary_slang_module() {
        let mut cmd = Command::new("slangc");
        cmd.arg(CGA3D_SLANG);
        cmd.arg("-o");
        cmd.arg(CGA3D_SLANG_MODULE);
        // TODO better error handling and propagation
        cmd.spawn().expect("failed to run slangc");
    }

    #[derive(Clone, Copy)]
    pub struct Target {
        pub file_extension: &'static str,
        pub target_argument: &'static str,
    }
    impl Target {
        const fn new(file_extension: &'static str, target_argument: &'static str) -> Self {
            Self { file_extension, target_argument }
        }

        // TODO commented out ones I am not sure about the file extension yet,
        //  only sure of the slangc argument

        pub const HLSL: Target = Target::new("hlsl", "hlsl");
        // pub const DXBC: Target = Target::new("", "dxbc");
        // pub const DXBC_ASSEMBLY: Target = Target::new("", "dxbc-asm");
        // pub const DXIL: Target = Target::new("", "dxil");
        // pub const DXIL_ASSEMBLY: Target = Target::new("", "dixl-asm");
        pub const GLSL: Target = Target::new("glsl", "glsl");
        // pub const GLSL_VULKAN: Target = Target::new("", "glsl-vulkan");
        // pub const GLSL_VULKAN_ONE_DESC: Target = Target::new("", "glsl-vulkan-one-desc");
        pub const SPIRV: Target = Target::new("spv", "spirv");
        // pub const SPIRV_ASSEMBLY: Target = Target::new("", "spirv-asm");
        pub const C: Target = Target::new("c", "c");
        pub const CPP: Target = Target::new("cpp", "cpp");
        // pub const TORCH: Target = Target::new("", "torch");
        // pub const HOST_CPP: Target = Target::new("", "host-cpp");
        pub const EXE: Target = Target::new("exe", "exe");
        // pub const SHADER_SHARED_LIB: Target = Target::new("", "shader-sharedlib");
        // pub const SHAREDLIB: Target = Target::new("", "sharedlib");
        // pub const CUDA: Target = Target::new("", "cuda");
        // pub const PTX: Target = Target::new("", "ptx");
        // pub const CUOBJ: Target = Target::new("", "cuobj");
        // pub const HOST_CALLABLE: Target = Target::new("", "host-callable");
        // pub const OBJECT_CODE: Target = Target::new("", "object-code");
        // pub const HOST_HOST_CALLABLE: Target = Target::new("", "host-host-callable");
        // pub const METAL: Target = Target::new("", "metal");
        // pub const METALLIB: Target = Target::new("", "metallib");
    }

    #[derive(Clone, Copy)]
    pub struct Stage {
        pub stage_argument: &'static str,
    }
    impl Stage {
        const fn new(stage_argument: &'static str) -> Self {
            Self { stage_argument }
        }
        pub const VERTEX: Stage = Stage::new("vertex");
        pub const HULL: Stage = Stage::new("hull");
        pub const DOMAIN: Stage = Stage::new("domain");
        pub const GEOMETRY: Stage = Stage::new("geometry");
        pub const PIXEL: Stage = Stage::new("pixel");
        pub const FRAGMENT: Stage = Stage::new("fragment");
        pub const COMPUTE: Stage = Stage::new("compute");
        pub const RAY_GENERATION: Stage = Stage::new("raygeneration");
        pub const INTERSECTION: Stage = Stage::new("intersection");
        pub const ANYHIT: Stage = Stage::new("anyhit");
        pub const CLOSESTHIT: Stage = Stage::new("closesthit");
        pub const MISS: Stage = Stage::new("miss");
        pub const CALLABLE: Stage = Stage::new("callable");
        pub const MESH: Stage = Stage::new("mesh");
        pub const AMPLIFICATION: Stage = Stage::new("application");
    }

    pub struct EntryPoint<S> {
        pub name: S,
        pub stage: Stage,
    }

    // https://github.com/shader-slang/slang/blob/master/docs/command-line-slangc-reference.md#-o
    pub enum OutputScheme<'a, P, S> {
        // If no -target or -stage is specified, one may be inferred from file extension
        InferStage(Target, P),

        // If multiple -target options and a single -entry are present, each -o associates
        // with the first -target to its left.
        OneEntryManyTargets { entry: EntryPoint<S>, targets: &'a [(Target, P)] },

        // Otherwise, if multiple -entry options are present, each -o associates with the first
        // -entry to its left, and with the -target that matches the one inferred from <path>.
        TargetsPerEntries { entries: &'a [(EntryPoint<S>, &'a [(Target, P)])] },
    }

    /// Use slangc to compile an application shader that depends on cga3d.slang and no
    /// other slang dependencies. If you need to compile with other dependencies, you can
    /// use slangc normally in the command line instead of using this function.
    pub fn compile_application_shader<'a, P1: AsRef<Path>, P2: AsRef<Path>, S: Clone + Into<String>>(app_src_file: P1, output_scheme: OutputScheme<'a, P2, S>) {
        let mut cmd = Command::new("slangc");
        cmd.arg("-I");
        cmd.arg(CGA3D_SLANG);

        match output_scheme {
            OutputScheme::InferStage(t, output_path) => {
                let ext = t.file_extension;
                let mut output_path = output_path.as_ref().to_path_buf();
                if !output_path.to_string_lossy().ends_with(ext) {
                    output_path = output_path.join(Path::new(ext));
                }
                let o = output_path.to_string_lossy();
                cmd.arg("-target");
                cmd.arg(t.target_argument);
                cmd.arg("-o");
                cmd.arg(o.as_ref());
            }
            OutputScheme::OneEntryManyTargets { entry, targets } => {
                cmd.arg("-entry");
                cmd.arg(entry.name.into());
                cmd.arg("-stage");
                cmd.arg(entry.stage.stage_argument);

                for (t, output_path) in targets {
                    let mut output_path = output_path.as_ref().to_path_buf();
                    let ext = t.file_extension;
                    if !output_path.to_string_lossy().ends_with(ext) {
                        output_path = output_path.join(Path::new(ext));
                    }
                    let o = output_path.to_string_lossy();
                    cmd.arg("-target");
                    cmd.arg(t.target_argument);
                    cmd.arg("-o");
                    cmd.arg(o.as_ref());
                }
            }
            OutputScheme::TargetsPerEntries { entries } => {
                for (entry, outputs) in entries {
                    cmd.arg("-entry");
                    cmd.arg(entry.name.clone().into());
                    cmd.arg("-stage");
                    cmd.arg(entry.stage.stage_argument);
                    for (t, output_path) in outputs.as_ref() {
                        let mut output_path = output_path.as_ref().to_path_buf();
                        let ext = t.file_extension;
                        if !output_path.to_string_lossy().ends_with(ext) {
                            output_path = output_path.join(Path::new(ext));
                        }
                        let o = output_path.to_string_lossy();
                        // cmd.arg("-target");
                        // cmd.arg(t.target_arg());
                        cmd.arg("-o");
                        cmd.arg(o.as_ref());
                    }
                }
            }
        }

        cmd.arg("--");
        cmd.arg(app_src_file.as_ref());

        // TODO better error handling and propagation
        cmd.spawn().expect("failed to run slangc");
    }
}
