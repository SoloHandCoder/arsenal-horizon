// Motion Blur: the last frame fades out over a set time (the same at any frame rate); none after a pause.
fn post(input: PostInput) -> vec4<f32> {
    let amount = clamp(frame.mods[0].y, 0.0, 1.0);
    let dt = frame.delta_time;
    if amount <= 0.0 || dt <= 0.0 || dt > 0.2 {
        return input.color;
    }
    // How much of the last frame is left after dt: a time constant of up to 60 ms.
    let k = min(exp(-dt / (0.06 * amount)), 0.75);
    let last = textureSampleLevel(history, linear_sampler, input.uv, 0.0).rgb;
    return vec4<f32>(mix(input.color.rgb, last, k), input.color.a);
}
