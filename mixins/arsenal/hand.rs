// The held item (and the empty hand) at the size the player picked.
fn arsenal_hand(game: &crate::game::Game<'_>, scale: &mut f32) {
    use crate::game::arsenal_native as m;
    if game.native_on(m::ID) {
        *scale *= m::settings(|key| game.native_setting(m::ID, key)).item_scale();
    }
}
