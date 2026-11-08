// Shulker Preview: a shulker box with something in it gets room for its grid under its tooltip.
fn arsenal_tooltip_size(
    game: &crate::game::Game<'_>,
    stack: &horizon_client::items::ItemStack,
    size: &mut Option<[f32; 2]>,
) {
    use crate::game::arsenal_native as m;
    if !game.native_on(m::ID) || size.is_some() {
        return;
    }
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    let full = stack.data().is_some_and(|d| !d.container.is_empty());
    if let Some(s) = m::shulker_size(&settings, &stack.item, full) {
        *size = Some(s);
    }
}
