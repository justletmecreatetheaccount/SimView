const PI = 3.141592653589793;

struct VertexIn {
    @location(0) position: vec3f,
    @location(1) uv: vec2f,
}

struct VertexOut {
    @builtin(position) clip_pos: vec4f,
    @location(0) uv: vec2f,
}

struct CameraUniforms {
    view_proj_mat: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> camera: CameraUniforms;
@group(0) @binding(1) var<storage, read> positions: array<vec3<f64>>;
@group(0) @binding(2) var<storage, read> radii: array<f32>;

@vertex
fn vtx_main(vtx_in: VertexIn, @builtin(instance_index) instance: u32) -> VertexOut {
    let vtx_true_pos: vec3<f64> = vec3<f64>(vtx_in.position) * f64(radii[instance]) + positions[instance];
    let vtx_proj: vec4<f64> = mat4x4<f64>(camera.view_proj_mat) * vec4<f64>(vtx_true_pos, 1);
    var out: VertexOut;
    out.clip_pos = vec4f(vtx_proj);
    out.uv = vtx_in.uv;

    return out;
}

@fragment
fn frag_main(frag_in: VertexOut) -> @location(0) vec4f {
    let radius = length(frag_in.uv);
    if radius > 1.0 {
        discard;
    }
    return vec4<f32>(sin(PI * radius / 2), 0, cos(PI * radius / 2), 1);
}
