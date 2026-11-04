// Before a HUD layer: Boss Bar, Title and Scoreboard Tweaks hide theirs, and Crosshair Editor draws the crosshair
// from Browse in place of the vanilla one.
fn arsenal_layer(
    game: &mut crate::game::Game<'_>,
    layer: &str,
    batch: &mut horizon_client::hud::HudBatch,
    canceled: &mut bool,
) {
    use crate::game::arsenal_native as m;
    if !game.native_on(m::ID) {
        return;
    }
    let game = &*game;
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    if !settings.hides_layer(layer) {
        return;
    }
    *canceled = true;
    if let Some(crosshair) = settings.crosshair.as_ref().filter(|_| layer.ends_with("crosshair")) {
        let center = [(batch.width / 2.0).floor(), (batch.height / 2.0).floor()];
        let cell = (crosshair.size as f32 * 0.5).max(0.5);
        for shape in m::crosshair_shapes(crosshair, center, cell) {
            if let m::Shape::Fill(rect, argb) = shape {
                game.native_fill(batch, rect, argb);
            }
        }
    }
}
