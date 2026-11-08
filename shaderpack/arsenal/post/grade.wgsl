// World Look (saturation, contrast, exposure, vignette), Sky & World Colors' world tint, Mist low over the ground and
// Slick Floors.
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
        // mist.w: how far under the camera it lies, so caves and every world get it the same.
        let low = 1.0 - smoothstep(mist.w - 6.0, mist.w + 4.0, p.y);
        let far = 1.0 - exp(-length(p) * 0.03 * density);
        c = mix(c, mist.rgb, clamp(far * low, 0.0, 0.85));
    }
    // Slick Floors: up-facing ground near you darker and glossy, with puddles (the normal comes from the depth).
    if frame.mods[6].x > 0.0 && input.depth > 0.0 {
        let px = 2.0 / frame.resolution;
        let right = input.uv + vec2<f32>(px.x, 0.0);
        let down = input.uv + vec2<f32>(0.0, px.y);
        let dr = depth_at(right);
        let dd = depth_at(down);
        if dr > 0.0 && dd > 0.0 {
            let p = view_position(input.uv, input.depth);
            let n = normalize(cross(view_position(right, dr) - p, view_position(down, dd) - p));
            let near = 1.0 - smoothstep(6.0, 16.0, length(p));
            let ground = smoothstep(0.95, 0.995, abs(n.y)) * select(0.0, 1.0, p.y < -0.2) * near * frame.mods[6].x;
            if ground > 0.0 {
                let world = p.xz + frame.camera_position.xz;
                let puddle = smoothstep(0.55, 0.8, hash2(floor(world * 0.5)) * 0.6 + hash2(floor(world * 1.7)) * 0.4);
                let view = normalize(-p);
                let r = reflect(-view, vec3<f32>(0.0, 1.0, 0.0));
                let sun = pow(max(dot(r, normalize(frame.sun_direction)), 0.0), 40.0) * (1.0 - night());
                let fresnel = pow(1.0 - max(view.y, 0.0), 3.0);
                let sky = mix(vec3<f32>(0.45, 0.55, 0.7), vec3<f32>(0.05, 0.07, 0.12), night());
                let wet = mix(c * 0.68, c * 0.5 + sky * 0.35, puddle) + sky * fresnel * 0.18 + vec3<f32>(sun);
                c = mix(c, wet, ground);
            }
        }
    }
    if look.w > 0.0 {
        let aspect = frame.resolution.x / frame.resolution.y;
        let d = length((input.uv - 0.5) * vec2<f32>(aspect, 1.0)) / length(vec2<f32>(aspect, 1.0) * 0.5);
        c *= 1.0 - smoothstep(0.5, 1.15, d) * look.w;
    }
    return vec4<f32>(clamp(c, vec3<f32>(0.0), vec3<f32>(1.0)), input.color.a);
}
