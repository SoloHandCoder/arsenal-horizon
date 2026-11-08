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
    let settings = m::raw_settings(|key| game.native_setting(m::ID, key));
    let menu = m::menu();
    let browse = if menu.page == m::Page::Browse { m::gallery(menu.tab) } else { m::Fetch::Loading };
    let shapes = m::menu_frame(screen.rect, screen.mouse, &menu, &settings, &browse, &|text| game.native_text_width(text));
    for shape in shapes {
        match shape {
            m::Shape::Fill(rect, argb) => game.native_fill(batch, rect, argb),
            m::Shape::Text(text, at, rgb, shadow) => game.native_text(batch, &text, at, rgb, shadow),
            m::Shape::Item(id, at) => {
                game.native_item(batch, &id, at, 16.0);
            }
            m::Shape::Quad(points, argb) => game.native_quad(batch, points, argb),
            m::Shape::Styled(text, at, argb, shadow, scale) => game.arsenal_text(batch, &text, at, argb, shadow, scale),
        }
    }
}
