// Block Outline: the outline of the block looked at, in the colour and thickness picked.
fn arsenal_visuals(game: &crate::game::Game<'_>, rims: &mut Vec<crate::game::native_hooks::NativeBlockRim>) {
    use crate::game::arsenal_native as m;
    if !game.native_on(m::ID) {
        return;
    }
    let Some(pos) = m::outlined() else {
        return;
    };
    m::set_outlined(None);
    if let Some((argb, width)) = m::settings(|key| game.native_setting(m::ID, key)).outline() {
        rims.push(crate::game::native_hooks::NativeBlockRim { pos, argb, width });
    }
}
