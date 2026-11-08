// Block Outline: the vanilla outline is left out and the block is remembered for `visuals.rs` to draw its own.
fn arsenal_highlight(game: &crate::game::Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    use crate::game::arsenal_native as m;
    m::set_aimed(pos);
    let on = game.native_on(m::ID) && m::settings(|key| game.native_setting(m::ID, key)).outline().is_some();
    m::set_outlined(on.then_some(pos));
    if on {
        *canceled = true;
    }
}
