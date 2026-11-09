// Inventory Cursor Speed: Arsenal's own cursor (the real one is hidden, never moved) over everything.
fn arsenal_gui_post(game: &crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    use std::sync::atomic::Ordering::Relaxed;
    if !horizon_client::camera::arsenal::CURSOR_ON.load(Relaxed) {
        return;
    }
    // Horizon shows the real cursor again on some frames: it's hidden every frame while Arsenal's is in use.
    if let Some(w) = &game.window {
        w.set_cursor_visible(false);
    }
    let [x, y] = game.native_cursor();
    let arrow = |d: f32| [[x - d, y - d * 1.5], [x - d, y + 11.0 + d], [x + 3.0, y + 8.0 + d * 0.5], [x + 8.0 + d * 1.5, y + 8.0 + d * 0.5]];
    game.native_quad(batch, arrow(1.0), 0xFF00_0000);
    game.native_quad(batch, arrow(0.0), 0xFFFF_FFFF);
    game.native_quad(batch, [[x + 2.0, y + 8.0], [x + 4.0, y + 7.5], [x + 6.5, y + 12.5], [x + 4.5, y + 13.0]], 0xFF00_0000);
    game.native_quad(batch, [[x + 2.6, y + 8.4], [x + 3.8, y + 8.1], [x + 6.0, y + 12.2], [x + 4.8, y + 12.5]], 0xFFFF_FFFF);
}
