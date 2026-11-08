// The sky: Sky & World Colors' sky tint, and Sky Effects' aurora, extra stars and glow at night.
fn sky(input: SkyInput) -> vec4<f32> {
    var c = input.color.rgb;
    let tint = frame.mods[3];
    c = mix(c, c * tint.rgb * 1.6, clamp(tint.w, 0.0, 1.0));
    let fx = frame.mods[4];
    let up = input.direction.y;
    let n = night();
    if fx.x > 0.0 && up > 0.0 {
        // Aurora: soft green and violet bands drifting across the northern sky.
        let band = sin(input.direction.x * 6.0 + frame.time * 0.15) * 0.5 + 0.5;
        let wave = sin(input.direction.z * 9.0 + band * 3.0 + frame.time * 0.3) * 0.5 + 0.5;
        let height = smoothstep(0.05, 0.35, up) * (1.0 - smoothstep(0.55, 0.95, up));
        let colour = mix(vec3<f32>(0.15, 0.95, 0.55), vec3<f32>(0.55, 0.3, 0.95), band);
        c += colour * wave * height * fx.x * n * 0.6;
    }
    if fx.y > 0.0 && up > 0.0 {
        // More stars: tiny points from a hash of the direction.
        let cell = floor(input.direction.xz / max(up, 0.05) * 220.0);
        let star = step(0.997, hash2(cell)) * smoothstep(0.0, 0.2, up);
        c += vec3<f32>(star * fx.y * n);
    }
    if fx.z > 0.0 {
        c += vec3<f32>(0.25, 0.2, 0.45) * fx.z * n * (1.0 - smoothstep(0.0, 0.5, abs(up))) * 0.4;
    }
    return vec4<f32>(c, input.color.a);
}
