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

@vertex
fn vtx_main(vtx_in: VertexIn) -> VertexOut {
    let vtx_proj = camera.view_proj_mat * vec4f(vtx_in.position, 1);
    var out: VertexOut;
    out.clip_pos = vec4f(vtx_proj);
    out.uv = vtx_in.uv;
    return out;
}

@fragment
fn frag_main(frag_in: VertexOut) -> @location(0) vec4f {
    let radius = length(frag_in.uv);
    if radius > 1.0 {
        return vec4(0, 0, 0, 0);
    }
    return vec4(1, 0, 0, 1);
}
