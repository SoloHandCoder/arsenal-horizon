// What Arsenal hands the pack each frame (frame.mods, set by the mod's native_shader_params hook):
//   mods[0]: x Bloom strength, y Motion Blur strength, z Mist density, w 1 while Arsenal runs
//   mods[1]: World Look: x saturation, y contrast, z exposure, w vignette (all 0 = leave the picture)
//   mods[2]: world tint: rgb colour, w how much
//   mods[3]: sky tint: rgb colour, w how much
//   mods[4]: Sky Effects: x aurora, y stars, z sky glow
//   mods[5]: Mist colour: rgb, w height it lies under (blocks)
//   mods[6]: x Slick Floors (1 on)

fn luminance(c: vec3<f32>) -> f32 {
    return dot(c, vec3<f32>(0.2126, 0.7152, 0.0722));
}

// 1 at night in the Overworld, 0 by day.
fn night() -> f32 {
    return select(0.0, 1.0 - smoothstep(-0.15, 0.05, frame.sun_direction.y), frame.dimension == 0);
}

fn hash2(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}
