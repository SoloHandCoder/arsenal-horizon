// Arsenal's panel (and FPS Graph's box) in their corner; the debug screen takes the top corners.
fn arsenal_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::arsenal_native as m;
    // Turned off in Options > Mods: nothing until the next build leaves it out.
    if !game.native_on(m::ID) {
        return;
    }
    use horizon_client::extras::Corner;
    let settings = {
        let game = &*game;
        m::settings(|key| game.native_setting(m::ID, key))
    };
    // Health HUD: under the crosshair, wherever the corner panel is (and also when the corner is hidden).
    if let Some(health) = m::health() {
        let game = &*game;
        for shape in m::health_shapes(&settings, &health, &|text| game.native_text_width(text)) {
            match shape {
                m::Shape::Fill(rect, argb) => game.native_fill(batch, rect, argb),
                m::Shape::Text(text, at, rgb, shadow) => game.native_text(batch, &text, at, rgb, shadow),
            }
        }
    }
    let fps = game.native_frame_rate().0;
    let player = game.native_player().map(|p| ([p.x, p.y, p.z], p.yaw));
    let (speed, session) = m::track(player.map(|(pos, _)| pos), fps, std::time::Instant::now());
    let corner = [Corner::TopLeft, Corner::TopRight, Corner::BottomLeft, Corner::BottomRight][settings.corner()];
    if !game.native_corner_shown(corner) {
        return;
    }
    let readings = m::Readings {
        fps,
        cps: if settings.on(m::INFO) && settings.on(m::CPS) { game.native_clicks() } else { (0, 0) },
        player,
        speed,
        session,
        sneaking: m::sneaking(),
    };
    let lines = m::lines(&settings, &readings);
    let game = &*game;
    let mut boxes = Vec::new();
    if let Some(panel) = m::panel(&lines, &settings, &|text| game.native_text_width(text)) {
        boxes.push(panel);
    }
    if settings.on(m::GRAPH) {
        if let Some(graph) = m::graph(&m::fps_history(), &settings) {
            boxes.push(graph);
        }
    }
    for (shapes, w, h) in boxes {
        let (x, y) = corners.place(corner, w, h);
        for shape in shapes {
            match shape {
                m::Shape::Fill([dx, dy, w, h], argb) => game.native_fill(batch, [x + dx, y + dy, w, h], argb),
                m::Shape::Text(text, [dx, dy], rgb, shadow) => game.native_text(batch, &text, [x + dx, y + dy], rgb, shadow),
            }
        }
    }
}
