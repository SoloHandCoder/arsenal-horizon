// Bloom: bright parts glow out softly. A disc of samples turned a little for each pixel (no rings), sized to the
// screen so it looks the same at any resolution; only what's brighter than the threshold glows.
fn post(input: PostInput) -> vec4<f32> {
    let strength = frame.mods[0].x;
    if strength <= 0.0 {
        return input.color;
    }
    let aspect = frame.resolution.x / frame.resolution.y;
    let radius = vec2<f32>(0.012 / aspect, 0.012);
    let spin = hash2(input.pixel.xy) * 6.2831853;
    var glow = vec3<f32>(0.0);
    var total = 0.0;
    for (var k = 0; k < 16; k++) {
        // Vogel's spiral: evenly spread over the disc.
        let r = sqrt((f32(k) + 0.5) / 16.0);
        let a = f32(k) * 2.3999632 + spin;
        let offset = vec2<f32>(cos(a), sin(a)) * radius * r;
        let s = textureSampleLevel(previous, linear_sampler, input.uv + offset, 0.0).rgb;
        let bright = max(luminance(s) - 0.65, 0.0);
        let w = 1.0 - r * 0.7;
        glow += s * bright * w;
        total += w;
    }
    let c = input.color.rgb + glow / max(total, 1.0) * strength * 4.0;
    return vec4<f32>(c, input.color.a);
}
