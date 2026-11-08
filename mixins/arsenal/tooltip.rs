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
    // Hypixel SkyBlock's prices: by the item's SkyBlock id (its custom data's "id").
    let skyblock_id = stack.custom_data().and_then(|c| match c.get(&horizon_world::nbt::JavaString::from("id")) {
        Some(horizon_world::nbt::Tag::String(id)) => id.to_string().ok(),
        _ => None,
    });
    if let Some(id) = skyblock_id {
        for (text, rgb) in m::skyblock_price_lines(&settings, &id, stack.count) {
            lines.push((horizon_client::text::Text::literal(&text), rgb));
        }
    }
}
