// HUD Theme: tooltips in the theme's colours (its panel behind, its accent round the edge).
fn arsenal_tooltip_color(
    game: &crate::game::Game<'_>,
    background: &mut u32,
    border_top: &mut u32,
    border_bottom: &mut u32,
) {
    use crate::game::arsenal_native as m;
    if !game.native_on(m::ID) {
        return;
    }
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    if let Some((bg, top, bottom)) = m::tooltip_colors(&settings) {
        *background = bg;
        *border_top = top;
        *border_bottom = bottom;
    }
}
