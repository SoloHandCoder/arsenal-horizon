// Arsenal's panel in its corner; the debug screen takes the top corners.
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
    let corner = [Corner::TopLeft, Corner::TopRight, Corner::BottomLeft, Corner::BottomRight][settings.corner()];
    if !game.native_corner_shown(corner) {
        return;
    }
    let readings = m::Readings {
        fps: game.native_frame_rate().0,
        cps: if settings.on(m::INFO) && settings.on(m::CPS) { game.native_clicks() } else { (0, 0) },
        player: game.native_player().map(|p| ([p.x, p.y, p.z], p.yaw)),
        ..m::Readings::default()
    };
    let (speed, session) = m::track(readings.player.map(|(pos, _)| pos), std::time::Instant::now());
    let readings = m::Readings { speed, session, ..readings };
    let lines = m::lines(&settings, &readings);
    let game = &*game;
    let Some((shapes, w, h)) = m::panel(&lines, settings.accent(), &|text| game.native_text_width(text)) else {
        return;
    };
    let (x, y) = corners.place(corner, w, h);
    for shape in shapes {
        match shape {
            m::Shape::Fill([dx, dy, w, h], argb) => game.native_fill(batch, [x + dx, y + dy, w, h], argb),
            m::Shape::Text(text, [dx, dy], rgb, shadow) => game.native_text(batch, &text, [x + dx, y + dy], rgb, shadow),
        }
    }
}
