struct Model {
    matrix: mat4x4<f32>;
};

struct Camera {
    matrix: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: Camera;
[[group(1), binding(0)]]
var<uniform> model: Model;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};

[[stage(vertex)]]
fn vert_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = camera.matrix * model.matrix * vec4<f32>(input.position, 1.0);
    output.color = input.color;
    return output;
}

[[stage(fragment)]]
fn frag_main(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(input.color, 0.0);
}