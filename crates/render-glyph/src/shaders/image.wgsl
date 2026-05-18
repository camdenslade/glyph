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
    @location(4) _pad:   f32,
    @location(5) tint:   vec4<f32>,
}

struct VertexOut {
    @builtin(position) clip:   vec4<f32>,
    @location(0)       uv:     vec2<f32>,
    @location(1)       rect:   vec4<f32>,
    @location(2)       radius: f32,
    @location(3)       tint:   vec4<f32>,
}

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    let ndc = vec2<f32>(
        in.pos.x / screen.size.x * 2.0 - 1.0,
       -(in.pos.y / screen.size.y * 2.0 - 1.0),
    );
    return VertexOut(vec4<f32>(ndc, 0.0, 1.0), in.uv, in.rect, in.radius, in.tint);
}

fn sdf_rounded_rect(p: vec2<f32>, rect: vec4<f32>, r: f32) -> f32 {
    let center = (rect.xy + rect.zw) * 0.5;
    let half   = (rect.zw - rect.xy) * 0.5 - vec2<f32>(r, r);
    let q      = abs(p - center) - half;
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let sampled = textureSample(img_texture, img_sampler, in.uv);
    let d  = sdf_rounded_rect(in.clip.xy, in.rect, in.radius);
    let fw = fwidth(d);
    let shape_alpha = 1.0 - smoothstep(-fw, fw, d);

    var out_color: vec4<f32>;
    if in.tint.a > 0.0 {
        // Tint mode: use tint RGB, multiply alpha by sampled alpha (icon mask)
        out_color = vec4<f32>(in.tint.rgb, in.tint.a * sampled.a);
    } else {
        // Normal mode: use texture colour as-is
        out_color = sampled;
    }

    return vec4<f32>(out_color.rgb, out_color.a * shape_alpha);
}
