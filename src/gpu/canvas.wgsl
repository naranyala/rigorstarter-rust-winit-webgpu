struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    // We use a simple approach: pass the screen size via a uniform or 
    // let the user handle normalization. For a Raylib-like API, 
    // we'll assume the screen size is passed or use a fixed projection.
    
    // Note: In a real Raylib-like impl, we'd use a uniform for screen size.
    // For this demo, we'll use a simple projection that assumes 900x600.
    let width = 900.0;
    let height = 600.0;
    
    let x = (model.position.x / width) * 2.0 - 1.0;
    let y = 1.0 - (model.position.y / height) * 2.0;
    
    var out: VertexOutput;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.color = model.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
