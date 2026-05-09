struct Screen {
    size: vec2<f32>,
}

@group(0) @binding(0) var<uniform> screen: Screen;
@group(1) @binding(0) var img_texture: texture_2d<f32>;
@group(1) @binding(1) var img_sampler: sampler;

struct VertexIn {
    @location(0) pos:   vec2<f32>,
    @location(1) uv:    vec2<f32>,
    @location(2) color: vec4<f32>,
}

struct VertexOut {
    @builtin(position) clip: vec4<f32>,
    @location(0)       uv:   vec2<f32>,
}

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    let ndc = vec2<f32>(
        in.pos.x / screen.size.x * 2.0 - 1.0,
       -(in.pos.y / screen.size.y * 2.0 - 1.0),
    );
    return VertexOut(vec4<f32>(ndc, 0.0, 1.0), in.uv);
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return textureSample(img_texture, img_sampler, in.uv);
}
