use std::path::{Path, PathBuf};
use crate::emit::Emitter;

pub fn emit_shader_support(
    emitter: &mut Emitter<std::fs::File>,
    base_file_path: &PathBuf,
    algebra_name: &str
) -> std::io::Result<()> {

    let upper_snake_case_name = algebra_name.to_uppercase();


    emitter.new_rust_collector(&base_file_path.join(Path::new("shaders.rs")));
    let rust_source = format!("

use std::fs;
use std::path::Path;

use naga::back::spv::{{Options, PipelineOptions, WriterFlags, ZeroInitializeWorkgroupMemoryMode}};
use naga::ShaderStage;
use naga::valid::{{Capabilities, ValidationFlags, Validator}};

/// Include the full wgsl source file (which may be several megabytes in size) in your rust binary.
/// It is recommended to compose and prune your shaders during your app build, instead of your app
/// runtime. (Hint: Enable this feature in [build-dependencies], but not [dependencies].)
/// Despite this recommendation, you can still include this in your app binary if you really want
/// or need to recompile shaders at app runtime for some reason.
#[cfg(feature = \"wgsl-compose\")]
pub const {upper_snake_case_name}_WGSL_SRC: &str = include_str!(\"shaders/{algebra_name}.wgsl\");

/// Include the full glsl source file (which may be several megabytes in size) in your rust binary.
/// It is recommended to compose and prune your shaders during your app build, instead of your app
/// runtime. (Hint: Enable this feature in [build-dependencies], but not [dependencies].)
/// Despite this recommendation, you can still include this in your app binary if you really want
/// or need to recompile shaders at app runtime for some reason.
#[cfg(feature = \"glsl-compose\")]
pub const {upper_snake_case_name}_GLSL_SRC: &str = include_str!(\"shaders/{algebra_name}.glsl\");

#[cfg(feature = \"wgsl-compose\")]
pub fn wgsl_composable_module_descriptor() -> naga_oil::compose::ComposableModuleDescriptor<'static> {{
    naga_oil::compose::ComposableModuleDescriptor {{
        source: {upper_snake_case_name}_WGSL_SRC,
        file_path: \"{algebra_name}/src/shaders/{algebra_name}.wgsl\",
        language: naga_oil::compose::ShaderLanguage::Wgsl,
        ..Default::default()
    }}
}}

#[cfg(feature = \"wgsl-compose\")]
pub fn wgsl_compose_with_entrypoints(naga_module_descriptor: naga_oil::compose::NagaModuleDescriptor) -> Result<naga::Module, naga_oil::compose::error::ComposerError> {{
    let mut composer =  naga_oil::compose::Composer::default();
    composer.add_composable_module(wgsl_composable_module_descriptor()).unwrap();
    let mut naga_module = composer.make_naga_module(naga_module_descriptor)?;
    let mut pruner = naga_oil::prune::Pruner::new(&naga_module);
    for ep in naga_module.entry_points.iter() {{
        pruner.add_entrypoint(ep, std::collections::HashMap::new(), Some(naga_oil::prune::PartReq::All));
    }}
    naga_module = pruner.rewrite();
    return Ok(naga_module);
}}


/// Compose wgsl, validate the module, and output a SPIR-V file for each entry point.
/// Half for utility, half for example, a pattern like this is useful in build.rs.
/// If you'd like to customize any of the options, you can copy and/or inline this function.
// TODO it is kind of a presumptuous/risky move to choose default options, even though the
//  succinctness can be nice. So it's probably best to atomize this function into
//  smaller itty-bitty parts that only add in assumptions layer by layer. Maybe the factory/builder
//  pattern can help keep things succinct.
#[cfg(feature = \"wgsl-compose\")]
pub fn wgsl_compose_validate_and_spirv<P: AsRef<Path>, S: Into<String>>(
    wgsl_file_path: &str,
    spirv_outputs: Vec<(P, S, ShaderStage)>
) -> anyhow::Result<()> {{
    let shader_src = fs::read_to_string(wgsl_file_path)?;
    let naga_module_descriptor = naga_oil::compose::NagaModuleDescriptor {{
        source: shader_src.as_str(),
        file_path: wgsl_file_path,
        ..Default::default()
    }};
    let naga_module = wgsl_compose_with_entrypoints(naga_module_descriptor)?;
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

#[cfg(feature = \"glsl-compose\")]
pub fn glsl_composable_module_descriptor() -> naga_oil::compose::ComposableModuleDescriptor<'static> {{
    naga_oil::compose::ComposableModuleDescriptor {{
        source: {upper_snake_case_name}_GLSL_SRC,
        file_path: \"{algebra_name}/src/shaders/{algebra_name}.glsl\",
        language: naga_oil::compose::ShaderLanguage::Glsl,
        ..Default::default()
    }}
}}

#[cfg(feature = \"glsl-compose\")]
pub fn glsl_compose_with_entrypoints(naga_module_descriptor: naga_oil::compose::NagaModuleDescriptor) -> Result<naga::Module, naga_oil::compose::error::ComposerError> {{
    let mut composer =  naga_oil::compose::Composer::default();
    composer.add_composable_module(glsl_composable_module_descriptor()).unwrap();
    let mut naga_module = composer.make_naga_module(naga_module_descriptor)?;

    // Pruning seems to corrupt glsl modules, giving us absurd/incorrect
    //  \"Initializer doesn't match the variable type\" validation errors
    // let mut pruner = naga_oil::prune::Pruner::new(&naga_module);
    // for ep in naga_module.entry_points.iter() {{
    //     pruner.add_entrypoint(ep, std::collections::HashMap::new(), Some(naga_oil::prune::PartReq::All));
    // }}
    // naga_module = pruner.rewrite();

    return Ok(naga_module);
}}



");
    emitter.emit_rust_preamble(rust_source.as_str())?;

    Ok(())
}


