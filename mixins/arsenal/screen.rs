// Arsenal's menu, drawn over its own screen.
fn arsenal_screen(
    game: &crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    screen: &crate::game::native_hooks::NativeScreenView<'_>,
) {
    use crate::game::arsenal_native as m;
    if screen.id != "mod" || game.native_mod_screen() != Some(m::ID) {
        return;
    }
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    let (shapes, _) = m::menu_layout(screen.rect, screen.mouse, &m::menu(), &settings, &|text| game.native_text_width(text));
    for shape in shapes {
        match shape {
            m::Shape::Fill(rect, argb) => game.native_fill(batch, rect, argb),
            m::Shape::Text(text, at, rgb, shadow) => game.native_text(batch, &text, at, rgb, shadow),
        }
    }
}
