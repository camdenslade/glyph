struct Screen {
    size: vec2<f32>,
}

@group(0) @binding(0) var<uniform> screen: Screen;
@group(1) @binding(0) var img_texture: texture_2d<f32>;
@group(1) @binding(1) var img_sampler: sampler;

struct VertexIn {
    @location(0) pos:    vec2<f32>,
    @location(1) uv:     vec2<f32>,
    @location(2) rect:   vec4<f32>,
    @location(3) radius: f32,
}

struct VertexOut {
    @builtin(position) clip: vec4<f32>,
    @location(0)       uv:   vec2<f32>,
    @location(1)       rect: vec4<f32>,
    @location(2)       radius: f32,
}

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    let ndc = vec2<f32>(
        in.pos.x / screen.size.x * 2.0 - 1.0,
       -(in.pos.y / screen.size.y * 2.0 - 1.0),
    );
    return VertexOut(vec4<f32>(ndc, 0.0, 1.0), in.uv, in.rect, in.radius);
}

fn sdf_rounded_rect(p: vec2<f32>, rect: vec4<f32>, r: f32) -> f32 {
    let center = (rect.xy + rect.zw) * 0.5;
    let half   = (rect.zw - rect.xy) * 0.5 - vec2<f32>(r, r);
    let q      = abs(p - center) - half;
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let color = textureSample(img_texture, img_sampler, in.uv);
    let d  = sdf_rounded_rect(in.clip.xy, in.rect, in.radius);
    let fw = fwidth(d);
    let alpha = 1.0 - smoothstep(-fw, fw, d);
    return vec4<f32>(color.rgb, color.a * alpha);
}
