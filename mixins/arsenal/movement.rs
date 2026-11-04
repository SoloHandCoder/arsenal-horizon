// Toggle Sneak: one press of sneak keeps the player sneaking until the next.
fn arsenal_movement(game: &crate::game::Game<'_>, sneak: &mut bool) {
    use crate::game::arsenal_native as m;
    if !game.native_on(m::ID) {
        return;
    }
    let on = m::settings(|key| game.native_setting(m::ID, key)).on(m::SNEAK);
    *sneak = m::sneak(on, *sneak);
}
