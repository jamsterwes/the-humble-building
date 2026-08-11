struct CameraUniform {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) texcoord: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) texcoord: vec2<f32>,
};

const WINDOW_X: f32 = 2.0;
const WINDOW_Y: f32 = 2.0;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.texcoord = model.texcoord * vec2<f32>(WINDOW_X, WINDOW_Y);
    out.clip_position = camera.view_proj * camera.model * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tiled = fract(in.texcoord);
    return vec4<f32>(tiled, 1.0, 1.0);
}
