use std::io::Write;

pub fn emit_shader_support<W: Write>(w: &mut W, algebra_name: &str, shader_extension: &str) -> anyhow::Result<()> {
    let upper_extension = shader_extension.to_uppercase();
    let lower_extension = shader_extension.to_lowercase();
    let camel_extension = {
        let mut c= lower_extension.chars();
        c.next().expect("Provide an extension").to_uppercase().collect::<String>() + c.as_str()
    };

    /*
    TODO output the spv files in "OUT_DIR" if that makes more sense
     */

    let upper_snake_case_name = algebra_name.to_uppercase();
    let mut file = w;
    write!(&mut file, r#"
use std::fs;
use std::path::Path;

use naga::back::spv::{{Options, PipelineOptions, WriterFlags, ZeroInitializeWorkgroupMemoryMode}};
use naga::ShaderStage;
use naga::valid::{{Capabilities, ValidationFlags, Validator}};

/// Include the full {lower_extension} source file (which may be several megabytes in size) in your rust binary.
/// It is recommended to compose and prune your shaders during your app build, instead of your app
/// runtime. (Hint: Enable this feature in [build-dependencies], but not [dependencies].)
/// Despite this recommendation, you can still include this in your app binary if you really want
/// or need to recompile shaders at app runtime for some reason.
pub const {upper_snake_case_name}_{upper_extension}_SRC: &str = include_str!("integrations/{lower_extension}/{algebra_name}.{lower_extension}");

pub fn {lower_extension}_composable_module_descriptor() -> naga_oil::compose::ComposableModuleDescriptor<'static> {{
    naga_oil::compose::ComposableModuleDescriptor {{
        source: {upper_snake_case_name}_{upper_extension}_SRC,
        file_path: "{algebra_name}/src/integrations/{lower_extension}/{algebra_name}.{lower_extension}",
        language: naga_oil::compose::ShaderLanguage::{camel_extension},
        ..Default::default()
    }}
}}

pub fn {lower_extension}_compose_with_entrypoints(naga_module_descriptor: naga_oil::compose::NagaModuleDescriptor) -> Result<naga::Module, naga_oil::compose::error::ComposerError> {{
    let mut composer =  naga_oil::compose::Composer::default();
    composer.add_composable_module({lower_extension}_composable_module_descriptor()).unwrap();
    let mut naga_module = composer.make_naga_module(naga_module_descriptor)?;
    let mut pruner = naga_oil::prune::Pruner::new(&naga_module);
    for ep in naga_module.entry_points.iter() {{
        pruner.add_entrypoint(ep, std::collections::HashMap::new(), Some(naga_oil::prune::PartReq::All));
    }}
    naga_module = pruner.rewrite();
    return Ok(naga_module);
}}

/// Compose {lower_extension}, validate the module, and output a SPIR-V file for each entry point.
/// Half for utility, half for example, a pattern like this is useful in build.rs.
/// If you'd like to customize any of the options, you can copy and/or inline this function.
// TODO it is kind of a presumptuous/risky move to choose default options, even though the
//  succinctness can be nice. So it's probably best to atomize this function into
//  smaller itty-bitty parts that only add in assumptions layer by layer. Maybe the factory/builder
//  pattern can help keep things succinct.
pub fn {lower_extension}_compose_validate_and_spirv<P: AsRef<Path>, S: Into<String>>(
    {lower_extension}_file_path: &str,
    spirv_outputs: Vec<(P, S, ShaderStage)>
) -> anyhow::Result<()> {{
    let shader_src = fs::read_to_string({lower_extension}_file_path)?;
    let naga_module_descriptor = naga_oil::compose::NagaModuleDescriptor {{
        source: shader_src.as_str(),
        file_path: {lower_extension}_file_path,
        ..Default::default()
    }};
    let naga_module = {lower_extension}_compose_with_entrypoints(naga_module_descriptor)?;
    let validator_flags = ValidationFlags::default();
    let capabilities = Capabilities::default();
    let mut validator = Validator::new(validator_flags, capabilities);
    let naga_module_info = validator.validate(&naga_module)?;
    let options = Options {{
        lang_version: (1, 6),
        flags: WriterFlags::empty(),
        binding_map: Default::default(),
        capabilities: None,
        bounds_check_policies: Default::default(),
        zero_initialize_workgroup_memory: ZeroInitializeWorkgroupMemoryMode::Native,
        debug_info: None,
    }};
    for (spirv_path, entry_point, shader_stage) in spirv_outputs {{
        let pipeline_options = PipelineOptions {{
            shader_stage,
            entry_point: entry_point.into(),
        }};
        let spv = naga::back::spv::write_vec(
            &naga_module, &naga_module_info,
            &options, Some(&pipeline_options)
        )?;
        fs::write(spirv_path, bytemuck::cast_slice(spv.as_slice()))?;
    }}
    Ok(())
}}
"#)?;
    Ok(())
}

pub fn emit_slang_support<W: Write>(w: &mut W, algebra_name: &str) -> anyhow::Result<()> {
    let upper_algebra_name = algebra_name.to_uppercase();
    write!(w, r#"
    use std::path::Path;
    use std::process::Command;

    pub const {upper_algebra_name}_SLANG: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/integrations/slang/{algebra_name}.slang");
    pub const {upper_algebra_name}_SLANG_MODULE: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/integrations/slang-module/{algebra_name}.slang-module");

    /// This requires slangc to be installed and added to your path
    /// Warning, this can be an extremely slow operation. Give it time.
    pub fn compile_binary_slang_module() {{
        let mut cmd = Command::new("slangc");
        cmd.arg({upper_algebra_name}_SLANG);
        cmd.arg("-o");
        cmd.arg({upper_algebra_name}_SLANG_MODULE);
        // TODO better error handling and propagation
        cmd.spawn().expect("failed to run slangc");
    }}

    #[derive(Clone, Copy)]
    pub struct Target {{
        pub file_extension: &'static str,
        pub target_argument: &'static str,
    }}
    impl Target {{
        const fn new(
            file_extension: &'static str,
            target_argument: &'static str,
        ) -> Self {{
            Self {{ file_extension, target_argument }}
        }}

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
    }}

    #[derive(Clone, Copy)]
    pub struct Stage {{
        pub stage_argument: &'static str,
    }}
    impl Stage {{
        const fn new(
            stage_argument: &'static str,
        ) -> Self {{
            Self {{ stage_argument }}
        }}
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
    }}

    pub struct EntryPoint<S> {{
        pub name: S,
        pub stage: Stage,
    }}

    // https://github.com/shader-slang/slang/blob/master/docs/command-line-slangc-reference.md#-o
    pub enum OutputScheme<'a, P, S> {{
        // If no -target or -stage is specified, one may be inferred from file extension
        InferStage(Target, P),

        // If multiple -target options and a single -entry are present, each -o associates
        // with the first -target to its left.
        OneEntryManyTargets {{
            entry: EntryPoint<S>,
            targets: &'a [(Target, P)],
        }},

        // Otherwise, if multiple -entry options are present, each -o associates with the first
        // -entry to its left, and with the -target that matches the one inferred from <path>.
        TargetsPerEntries {{
            entries: &'a [(EntryPoint<S>, &'a [(Target, P)])],
        }},
    }}

    /// Use slangc to compile an application shader that depends on {algebra_name}.slang and no
    /// other slang dependencies. If you need to compile with other dependencies, you can
    /// use slangc normally in the command line instead of using this function.
    pub fn compile_application_shader<'a, P1: AsRef<Path>, P2: AsRef<Path>, S: Clone + Into<String>>(
        app_src_file: P1,
        output_scheme: OutputScheme<'a, P2, S>,
    ) {{
        let mut cmd = Command::new("slangc");
        cmd.arg("-I");
        cmd.arg({upper_algebra_name}_SLANG);

        match output_scheme {{
            OutputScheme::InferStage(t, output_path) => {{
                let ext = t.file_extension;
                let mut output_path = output_path.as_ref().to_path_buf();
                if !output_path.to_string_lossy().ends_with(ext) {{
                    output_path = output_path.join(Path::new(ext));
                }}
                let o = output_path.to_string_lossy();
                cmd.arg("-target");
                cmd.arg(t.target_argument);
                cmd.arg("-o");
                cmd.arg(o.as_ref());
            }}
            OutputScheme::OneEntryManyTargets {{ entry, targets }} => {{
                cmd.arg("-entry");
                cmd.arg(entry.name.into());
                cmd.arg("-stage");
                cmd.arg(entry.stage.stage_argument);

                for (t, output_path) in targets {{
                    let mut output_path = output_path.as_ref().to_path_buf();
                    let ext = t.file_extension;
                    if !output_path.to_string_lossy().ends_with(ext) {{
                        output_path = output_path.join(Path::new(ext));
                    }}
                    let o = output_path.to_string_lossy();
                    cmd.arg("-target");
                    cmd.arg(t.target_argument);
                    cmd.arg("-o");
                    cmd.arg(o.as_ref());
                }}
            }}
            OutputScheme::TargetsPerEntries {{ entries }} => {{
                for (entry, outputs) in entries {{
                    cmd.arg("-entry");
                    cmd.arg(entry.name.clone().into());
                    cmd.arg("-stage");
                    cmd.arg(entry.stage.stage_argument);
                    for (t, output_path) in outputs.as_ref() {{
                        let mut output_path = output_path.as_ref().to_path_buf();
                        let ext = t.file_extension;
                        if !output_path.to_string_lossy().ends_with(ext) {{
                            output_path = output_path.join(Path::new(ext));
                        }}
                        let o = output_path.to_string_lossy();
                        // cmd.arg("-target");
                        // cmd.arg(t.target_arg());
                        cmd.arg("-o");
                        cmd.arg(o.as_ref());
                    }}
                }}
            }}
        }}

        cmd.arg("--");
        cmd.arg(app_src_file.as_ref());

        // TODO better error handling and propagation
        cmd.spawn().expect("failed to run slangc");
    }}

    "#)?;
    Ok(())
}