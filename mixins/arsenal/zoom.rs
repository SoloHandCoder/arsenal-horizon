// Arsenal's zoom while its key is held, unless another mod's came first.
fn arsenal_zoom(game: &crate::game::Game<'_>, zoom: &mut Option<crate::game::native_hooks::NativeZoom>) {
    use crate::game::arsenal_native as m;
    if zoom.is_none()
        && game.native_on(m::ID)
        && let Some((factor, smooth, turn)) = m::view()
    {
        *zoom = Some(crate::game::native_hooks::NativeZoom { factor, smooth, turn });
    }
}
