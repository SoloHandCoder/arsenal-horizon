// Health HUD: the health rows hand over the player's health each frame; Horizon still draws its own rows (draws is
// left as it is), and Arsenal draws its number and bar under the crosshair from the HUD hook.
fn arsenal_health(
    game: &mut crate::game::Game<'_>,
    inputs: &horizon_client::hud_layout::HealthInputs,
    wi: i32,
    hi: i32,
) {
    use crate::game::arsenal_native as m;
    if !game.native_on(m::ID) {
        return;
    }
    m::set_health(m::Health {
        health: inputs.health,
        max: inputs.max_health,
        absorption: inputs.absorption,
        food: inputs.food,
        saturation: inputs.saturation,
        width: wi as f32,
        height: hi as f32,
    });
}
