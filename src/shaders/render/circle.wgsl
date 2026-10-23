struct VertexIn {
    @location(0) position: vec3f,
}

@vertex
fn vtx_main(vtx_in: VertexIn) -> @builtin(position) vec4f {
  return vec4f(vtx_in.position, 1);
}

@fragment
fn frag_main() -> @location(0) vec4f {
  return vec4(1, 0, 0, 1);
}
