struct VertexIn {
    @location(0) pos:   vec2<f32>,
    @location(1) uv:    vec2<f32>,
    @location(2) color: vec4<f32>,
}

struct VertexOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0)       uv:       vec2<f32>,
    @location(1)       color:    vec4<f32>,
}

@group(0) @binding(0) var<uniform> screen:      vec2<f32>;
@group(1) @binding(0) var          t_atlas:     texture_2d<f32>;
@group(1) @binding(1) var          s_atlas:     sampler;

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    let ndc = vec2<f32>(
        (in.pos.x / screen.x) * 2.0 - 1.0,
        1.0 - (in.pos.y / screen.y) * 2.0,
    );
    var out: VertexOut;
    out.clip_pos = vec4<f32>(ndc, 0.0, 1.0);
    out.uv       = in.uv;
    out.color    = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let alpha = textureSample(t_atlas, s_atlas, in.uv).r;
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
