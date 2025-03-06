/*
cd .\IdeaProjects\projective_ga\examples\hello_circle_slang\hello_circle_slang\
Measure-Command { slangc -I "../../../libraries/cga3d/src/integrations/slang-module" -entry "vs_main" -stage "vertex" -o "res/shader.vs.spv" -reflection-json "res/shader.vs.json" -report-perf-benchmark -fvk-use-entrypoint-name -- "src/shader.slang" }
Measure-Command { slangc -I "../../../libraries/cga3d/src/integrations/slang-module" -entry "fs_main" -stage "fragment" -o "res/shader.fs.spv" -reflection-json "res/shader.fs.json" -report-perf-benchmark -fvk-use-entrypoint-name -- "src/shader.slang" }
Measure-Command { slangc -I "../../../libraries/cga3d/src/integrations/slang-module" -entry "fs_main" -stage "fragment" -o "res/shader.fs.wgsl" -reflection-json "res/shader.fs.json" -report-perf-benchmark -fvk-use-entrypoint-name -- "src/shader.slang" }
 */

pub fn main() -> anyhow::Result<()> {
    hello_circle_slang_build::compile_shaders_in("examples/hello_circle_slang/hello_circle_slang")
}