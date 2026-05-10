struct VertexIn {
    @location(0) pos:    vec2<f32>,
    @location(1) color:  vec4<f32>,
    // rect bounds in pixel space (x, y, x+w, y+h) and corner radius
    @location(2) rect:   vec4<f32>,
    @location(3) radius: f32,
}

struct VertexOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0)       color:    vec4<f32>,
    @location(1)       rect:     vec4<f32>,
    @location(2)       radius:   f32,
}

@group(0) @binding(0) var<uniform> screen: vec2<f32>;

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    let ndc = vec2<f32>(
        (in.pos.x / screen.x) * 2.0 - 1.0,
        1.0 - (in.pos.y / screen.y) * 2.0,
    );
    var out: VertexOut;
    out.clip_pos = vec4<f32>(ndc, 0.0, 1.0);
    out.color    = in.color;
    out.rect     = in.rect;
    out.radius   = in.radius;
    return out;
}

// Signed distance to a rounded rectangle.
fn sdf_rounded_rect(p: vec2<f32>, rect: vec4<f32>, r: f32) -> f32 {
    let center = (rect.xy + rect.zw) * 0.5;
    let half   = (rect.zw - rect.xy) * 0.5 - vec2<f32>(r, r);
    let q      = abs(p - center) - half;
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let d  = sdf_rounded_rect(in.clip_pos.xy, in.rect, in.radius);
    // fwidth gives the screen-space derivative of d — exactly 1 physical pixel.
    // smoothstep over [-fw, fw] produces a perfectly anti-aliased edge at any DPI.
    let fw = fwidth(d);
    let alpha = 1.0 - smoothstep(-fw, fw, d);
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
