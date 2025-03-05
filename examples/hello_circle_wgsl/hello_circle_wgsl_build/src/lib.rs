
pub const APP_WGSL: &'static str = "src/shader.wgsl";
pub const VERTEX_SHADER_SPIRV: &'static str = "res/shader.vs.spv";
pub const FRAGMENT_SHADER_SPIRV: &'static str = "res/shader.fs.spv";
pub const VERTEX_SHADER_MAIN: &'static str = "vs_main";
pub const FRAGMENT_SHADER_MAIN: &'static str = "fs_main";

pub fn compile_shaders() -> anyhow::Result<()> {
    compile_shaders_in(".")
}
pub fn compile_shaders_in(_app_root: &'static str) -> anyhow::Result<()> {
    // let app_wgsl = Box::new(format!("{app_root}/{APP_WGSL}")).leak();
    // let vertex_spirv = format!("{app_root}/{VERTEX_SHADER_SPIRV}");
    // let fragment_spirv = format!("{app_root}/{FRAGMENT_SHADER_SPIRV}");
    // cga3d::integrations::wgsl::wgsl_compose_validate_and_spirv(
    //     app_wgsl,
    //     vec![
    //         (vertex_spirv, VERTEX_SHADER_MAIN, naga::ShaderStage::Vertex),
    //         (fragment_spirv, FRAGMENT_SHADER_MAIN, naga::ShaderStage::Fragment),
    //     ],
    // )?;
    Ok(())
}
