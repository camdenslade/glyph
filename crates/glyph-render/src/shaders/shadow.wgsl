struct VertexIn {
    @location(0) pos:    vec2<f32>,
    @location(1) rect:   vec4<f32>,
    @location(2) params: vec2<f32>,
    @location(3) color:  vec4<f32>,
}

struct VertexOut {
    @builtin(position) clip:   vec4<f32>,
    @location(0)       rect:   vec4<f32>,
    @location(1)       params: vec2<f32>,
    @location(2)       color:  vec4<f32>,
}

@group(0) @binding(0) var<uniform> screen: vec2<f32>;

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    let ndc = vec2<f32>(
        (in.pos.x / screen.x) * 2.0 - 1.0,
        1.0 - (in.pos.y / screen.y) * 2.0,
    );
    var out: VertexOut;
    out.clip   = vec4<f32>(ndc, 0.0, 1.0);
    out.rect   = in.rect;
    out.params = in.params;
    out.color  = in.color;
    return out;
}

fn erf_approx(x: f32) -> f32 {
    let s = sign(x);
    let a = abs(x);
    let t = 1.0 / (1.0 + 0.3275911 * a);
    let p = t * (0.254829592 + t * (-0.284496736 + t * (1.421413741 + t * (-1.453152027 + t * 1.061405429))));
    return s * (1.0 - p * exp(-a * a));
}

fn gauss_band(a: f32, b: f32, sigma: f32) -> f32 {
    let inv_s2 = 1.0 / (sigma * 1.41421356);
    return 0.5 * (erf_approx(b * inv_s2) - erf_approx(a * inv_s2));
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    // @builtin(position) gives us pixel coords in framebuffer space.
    let p      = in.clip.xy;
    let rect   = in.rect;
    let radius = in.params.x;
    let sigma  = in.params.y;

    let bx = gauss_band(rect.x - p.x, rect.z - p.x, sigma);
    let by = gauss_band(rect.y - p.y, rect.w - p.y, sigma);
    var a  = clamp(bx * by * 4.0, 0.0, 1.0);

    return vec4<f32>(in.color.rgb, in.color.a * a);
}
