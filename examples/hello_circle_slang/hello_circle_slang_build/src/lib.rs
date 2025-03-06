use cga3d::integrations::slang::{EntryPoint, OutputScheme, Stage, Target};

pub const APP_SLANG: &'static str = "src/shader.slang";
pub const VERTEX_SHADER_SPIRV: &'static str = "res/shader.vs.spv";
pub const FRAGMENT_SHADER_SPIRV: &'static str = "res/shader.fs.spv";
pub const VERTEX_SHADER_MAIN: &'static str = "vs_main";
pub const FRAGMENT_SHADER_MAIN: &'static str = "fs_main";

pub fn compile_shaders() -> anyhow::Result<()> {
    compile_shaders_in(".")
}
pub fn compile_shaders_in(app_root: &'static str) -> anyhow::Result<()> {
    // let app_slang = format!("{app_root}/{APP_SLANG}");
    // let vertex_spirv = format!("{app_root}/{VERTEX_SHADER_SPIRV}");
    // let fragment_spirv = format!("{app_root}/{FRAGMENT_SHADER_SPIRV}");

    cga3d::integrations::slang::compile_binary_slang_module();

    // cga3d::integrations::slang::compile_application_shader(
    //     app_slang,
    //     OutputScheme::TargetsPerEntries {
    //         entries: &[
    //             (
    //                 EntryPoint { name: VERTEX_SHADER_MAIN, stage: Stage::VERTEX },
    //                 &[(Target::SPIRV, vertex_spirv)]
    //             ),
    //             (
    //                 EntryPoint { name: FRAGMENT_SHADER_MAIN, stage: Stage::FRAGMENT },
    //                 &[(Target::SPIRV, fragment_spirv)]
    //             ),
    //         ],
    //     }
    // );
    Ok(())
}
