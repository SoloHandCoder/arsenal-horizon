// Motion Blur: a little of the last frame stays on, so fast turns smear (none on the first frame or after a pause).
fn post(input: PostInput) -> vec4<f32> {
    let k = clamp(frame.mods[0].y, 0.0, 0.85);
    if k <= 0.0 || frame.delta_time <= 0.0 {
        return input.color;
    }
    let last = textureSampleLevel(history, linear_sampler, input.uv, 0.0).rgb;
    return vec4<f32>(mix(input.color.rgb, last, k), input.color.a);
}
