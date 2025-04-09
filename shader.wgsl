struct VertexOutput{
    @builtin(position) clip_position: vec4<f32>,
    @location(0) position: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32,) -> VertexOutput
{
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index));
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    // x = 0 to 2
    out.position = vec2<f32>(x,y);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3,0.7, 0.5, 1.0);
}

@fragment
fn fs_main2(in: VertexOutput) -> @location(0) vec4<f32> {
    var col = vec4<f32>(in.position.x,in.position.y,1.0,1.0);
    // col x (-2 to 2) y (1 to -1)
    col.r = (col.r + 1.0) / 2.0;
    col.g = (col.g + 1.0) / 2.0;


    return col;
}
