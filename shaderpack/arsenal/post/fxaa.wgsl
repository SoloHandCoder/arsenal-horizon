// Smoothing: a light FXAA, as a pack with post passes turns off the game's own anti-aliasing.
fn post(input: PostInput) -> vec4<f32> {
    let px = 1.0 / frame.resolution;
    let c = input.color.rgb;
    let n = textureSampleLevel(previous, linear_sampler, input.uv + vec2<f32>(0.0, -px.y), 0.0).rgb;
    let s = textureSampleLevel(previous, linear_sampler, input.uv + vec2<f32>(0.0, px.y), 0.0).rgb;
    let e = textureSampleLevel(previous, linear_sampler, input.uv + vec2<f32>(px.x, 0.0), 0.0).rgb;
    let w = textureSampleLevel(previous, linear_sampler, input.uv + vec2<f32>(-px.x, 0.0), 0.0).rgb;
    let lc = luminance(c);
    let ln = luminance(n);
    let ls = luminance(s);
    let le = luminance(e);
    let lw = luminance(w);
    let lo = min(lc, min(min(ln, ls), min(le, lw)));
    let hi = max(lc, max(max(ln, ls), max(le, lw)));
    let range = hi - lo;
    if range < max(0.0312, hi * 0.125) {
        return input.color;
    }
    // Blur along the edge (across the larger change).
    let horizontal = abs(ln + ls - 2.0 * lc) >= abs(le + lw - 2.0 * lc);
    let dir = select(vec2<f32>(px.x, 0.0), vec2<f32>(0.0, px.y), horizontal);
    let a = textureSampleLevel(previous, linear_sampler, input.uv + dir * 0.5, 0.0).rgb;
    let b = textureSampleLevel(previous, linear_sampler, input.uv - dir * 0.5, 0.0).rgb;
    let amount = clamp(range / max(hi, 0.0001), 0.0, 1.0) * 0.6;
    return vec4<f32>(mix(c, (a + b) * 0.5, amount), input.color.a);
}
