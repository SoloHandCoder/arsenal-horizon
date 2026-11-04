// While the zoom key is held, the wheel zooms instead of moving the hotbar.
fn arsenal_wheel(game: &crate::game::Game<'_>, steps: f32, claimed: &mut bool) {
    use crate::game::arsenal_native as m;
    if !*claimed && game.native_on(m::ID) && m::wheel(steps) {
        *claimed = true;
    }
}
