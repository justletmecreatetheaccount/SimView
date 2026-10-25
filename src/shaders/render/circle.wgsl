struct VertexIn {
    @location(0) position: vec3f,
}

struct CameraUniforms {
    view_proj_mat: mat4x4<f32>,
};
@group(0) @binding(0) var<uniform> camera: CameraUniforms;

@vertex
fn vtx_main(vtx_in: VertexIn) -> @builtin(position) vec4f {
    let vtx_proj = camera.view_proj_mat * vec4f(vtx_in.position, 1);
    return vec4f(vtx_proj);
}

@fragment
fn frag_main() -> @location(0) vec4f {
    return vec4(1, 0, 0, 1);
}
