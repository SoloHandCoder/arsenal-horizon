// World Look (saturation, contrast, exposure, vignette), Sky & World Colors' world tint, and Mist low over the ground.
fn post(input: PostInput) -> vec4<f32> {
    var c = input.color.rgb;
    let look = frame.mods[1];
    if look.x > 0.0 {
        c = mix(vec3<f32>(luminance(c)), c, look.x);
    }
    if look.y > 0.0 {
        c = (c - 0.5) * look.y + 0.5;
    }
    if look.z > 0.0 {
        c *= look.z;
    }
    let tint = frame.mods[2];
    c = mix(c, c * tint.rgb * 1.5, clamp(tint.w, 0.0, 1.0));
    let mist = frame.mods[5];
    let density = frame.mods[0].z;
    if density > 0.0 && input.depth > 0.0 {
        let p = view_position(input.uv, input.depth);
        let height = p.y + frame.camera_position.y;
        let low = 1.0 - smoothstep(mist.w - 6.0, mist.w + 4.0, height);
        let far = 1.0 - exp(-length(p) * 0.03 * density);
        c = mix(c, mist.rgb, clamp(far * low, 0.0, 0.85));
    }
    if look.w > 0.0 {
        let aspect = frame.resolution.x / frame.resolution.y;
        let d = length((input.uv - 0.5) * vec2<f32>(aspect, 1.0)) / length(vec2<f32>(aspect, 1.0) * 0.5);
        c *= 1.0 - smoothstep(0.5, 1.15, d) * look.w;
    }
    return vec4<f32>(clamp(c, vec3<f32>(0.0), vec3<f32>(1.0)), input.color.a);
}
