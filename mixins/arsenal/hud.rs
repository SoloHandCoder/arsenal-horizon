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
        // Low Health Warning's heartbeat, at the player (only they hear it).
        if m::heartbeat_due(&settings, &health) {
            if let Some(p) = game.native_player() {
                game.native_play_local_sound("minecraft:entity.warden.heartbeat", [p.x, p.y, p.z], 0.6, 1.0, 0);
            }
        }
        let game = &*game;
        let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as u64);
        let mut shapes = m::low_health_shapes(&settings, &health, millis);
        shapes.extend(m::saturation_shapes(&settings, &health));
        shapes.extend(m::health_shapes(&settings, &health, &|text| game.native_text_width(text)));
        for shape in shapes {
            match shape {
                m::Shape::Fill(rect, argb) => game.native_fill(batch, rect, argb),
                m::Shape::Text(text, at, rgb, shadow) => game.native_text(batch, &text, at, rgb, shadow),
                m::Shape::Item(id, at) => {
                    game.native_item(batch, &id, at, 16.0);
                }
                m::Shape::Quad(points, argb) => game.native_quad(batch, points, argb),
            }
        }
    }
    // The inventory modules (Arsenal's inventory hook, patches/inventory.json): Armor HUD, Item Counter, the
    // offhand totem and durability warnings and Ready Check.
    if let Some(view) = game.native_inventory_view() {
        let stack = |s: &Option<horizon_client::items::ItemStack>| {
            s.as_ref().filter(|s| s.count > 0).map(|s| m::Stack { id: s.item.to_string(), count: s.count, damage: s.damage })
        };
        let inv = m::Inventory {
            slots: view.slots.iter().map(stack).collect(),
            armor: [0, 1, 2, 3].map(|i| view.armor.get(i).and_then(stack)),
            offhand: stack(&view.offhand),
            selected: view.selected,
        };
        let worn = m::worn_out(&settings, &inv);
        if m::durability_sound_due(&worn) && !worn.is_empty() {
            if let Some(p) = game.native_player() {
                game.native_play_local_sound("minecraft:block.anvil.land", [p.x, p.y, p.z], 0.5, 1.6, 0);
            }
        }
        let game = &*game;
        let (w, h) = (batch.width, batch.height);
        let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as u64);
        let width = |text: &str| game.native_text_width(text);
        let mut shapes = m::armor_shapes(&settings, &inv, w, h, &width);
        shapes.extend(m::counter_shapes(&settings, &inv, w, h, &width));
        shapes.extend(m::warning_shapes(&settings, &inv, w, h, millis, &width));
        shapes.extend(m::ready_shapes(&settings, &inv, h, &width));
        for shape in shapes {
            match shape {
                m::Shape::Fill(rect, argb) => game.native_fill(batch, rect, argb),
                m::Shape::Text(text, at, rgb, shadow) => game.native_text(batch, &text, at, rgb, shadow),
                m::Shape::Item(id, at) => {
                    game.native_item(batch, &id, at, 16.0);
                }
                m::Shape::Quad(points, argb) => game.native_quad(batch, points, argb),
            }
        }
    }
    // The combat modules (Arsenal's chat and players hooks): in the world through the camera, and on the HUD.
    {
        let game = &*game;
        let others: Vec<m::Other> = game.native_others().into_iter().map(|o| m::Other {
            id: o.id,
            name: o.name,
            pos: [o.x, o.y, o.z],
            health: o.health,
            absorption: o.absorption,
            hurt_time: o.hurt_time,
            equipment: o.equipment.iter().map(|s| s.as_ref().filter(|s| s.count > 0).map(|s| s.item.to_string())).collect(),
        }).collect();
        let cam = game.native_camera().map(|c| m::Cam {
            pos: [c.position[0] as f64, c.position[1] as f64, c.position[2] as f64],
            view: c.view,
            projection: c.projection,
        });
        let target = match (game.native_player(), cam.as_ref()) {
            (Some(p), Some(c)) => m::target(&others, c.pos, p.yaw, p.pitch),
            _ => None,
        };
        let width = |text: &str| game.native_text_width(text);
        for shape in m::pvp_shapes(&settings, &others, target, cam.as_ref(), batch.width, batch.height, &width) {
            match shape {
                m::Shape::Fill(rect, argb) => game.native_fill(batch, rect, argb),
                m::Shape::Text(text, at, rgb, shadow) => game.native_text(batch, &text, at, rgb, shadow),
                m::Shape::Item(id, at) => {
                    game.native_item(batch, &id, at, 16.0);
                }
                m::Shape::Quad(points, argb) => game.native_quad(batch, points, argb),
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
                m::Shape::Item(id, [dx, dy]) => {
                    game.native_item(batch, &id, [x + dx, y + dy], 16.0);
                }
                m::Shape::Quad(points, argb) => game.native_quad(batch, points.map(|[px, py]| [x + px, y + py]), argb),
            }
        }
    }
}
