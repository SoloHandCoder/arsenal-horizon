// Shulker Preview: the box's contents as a 9 x 3 grid in the box's colour, with their counts.
fn arsenal_tooltip_draw(
    game: &crate::game::Game<'_>,
    stack: &horizon_client::items::ItemStack,
    batch: &mut horizon_client::hud::HudBatch,
    origin: [f32; 2],
    size: [f32; 2],
) {
    use crate::game::arsenal_native as m;
    if !game.native_on(m::ID) || !stack.item.ends_with("shulker_box") {
        return;
    }
    let Some(data) = stack.data() else { return };
    game.native_fill(batch, [origin[0], origin[1], size[0], size[1]], m::shulker_color(&stack.item));
    for slot in 0..27u8 {
        let [x, y] = m::shulker_slot(origin, slot);
        game.native_fill(batch, [x, y, 16.0, 16.0], 0x5000_0000);
    }
    for (slot, item) in &data.container {
        if *slot >= 27 {
            continue;
        }
        let [x, y] = m::shulker_slot(origin, *slot);
        game.native_item_stack(batch, item, [x, y], 16.0);
        if item.count > 1 {
            let text = item.count.to_string();
            let w = game.native_text_width(&text);
            game.native_text(batch, &text, [x + 17.0 - w, y + 9.0], 0xFFFFFF, true);
        }
    }
}
