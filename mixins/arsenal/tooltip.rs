// Better Tooltips: durability numbers and what food restores, under the game's own lines.
fn arsenal_tooltip(
    game: &crate::game::Game<'_>,
    stack: &horizon_client::items::ItemStack,
    lines: &mut Vec<(horizon_client::text::Text, u32)>,
) {
    use crate::game::arsenal_native as m;
    if !game.native_on(m::ID) {
        return;
    }
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    for (text, rgb) in m::tooltip_lines(&settings, &stack.item, stack.damage) {
        lines.push((horizon_client::text::Text::literal(&text), rgb));
    }
}
