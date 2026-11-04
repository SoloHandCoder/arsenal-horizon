// The third-person camera's distance the player picked, scaled as vanilla's is.
fn arsenal_camera(game: &crate::game::Game<'_>, distance: &mut f32) {
    use crate::game::arsenal_native as m;
    if game.native_on(m::ID) {
        *distance = m::settings(|key| game.native_setting(m::ID, key)).camera_distance(*distance);
    }
}
