// Bloom: bright parts glow out softly (a ring of samples round each pixel, only what's brighter than the threshold).
fn post(input: PostInput) -> vec4<f32> {
    let strength = frame.mods[0].x;
    if strength <= 0.0 {
        return input.color;
    }
    let px = 1.0 / frame.resolution;
    var glow = vec3<f32>(0.0);
    var total = 0.0;
    for (var ring = 1; ring <= 3; ring++) {
        for (var k = 0; k < 8; k++) {
            let a = f32(k) * 0.7853982 + f32(ring) * 0.4;
            let offset = vec2<f32>(cos(a), sin(a)) * px * f32(ring) * 4.0;
            let s = textureSampleLevel(previous, linear_sampler, input.uv + offset, 0.0).rgb;
            let bright = max(luminance(s) - 0.65, 0.0);
            let w = 1.0 / f32(ring);
            glow += s * bright * w;
            total += w;
        }
    }
    let c = input.color.rgb + glow / max(total, 1.0) * strength * 4.0;
    return vec4<f32>(c, input.color.a);
}
