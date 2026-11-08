// Every client tick: the menu's close button (the click hook can't close screens), and the combat modules: new chat
// lines (Arsenal's chat hook), the other players and their entity events (the players hook), your position (Jump
// Circles), then the messages, sounds and own-chat lines they asked for.
fn arsenal_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::arsenal_native as m;
    if m::take_close() {
        game.native_mod_screen_close(m::ID);
    }
    if !game.native_on(m::ID) {
        return;
    }
    let settings = {
        let game = &*game;
        m::settings(|key| game.native_setting(m::ID, key))
    };
    // Arsenal's visuals (Horizon's own PvP extras, brightness and the renderer's checks).
    let v = m::visuals(&settings, game.arsenal_gamma());
    game.arsenal_visuals(&crate::game::native_hooks::ArsenalVisuals {
        gamma: v.gamma,
        fire_lower: v.fire_lower,
        shield_size: v.shield_size,
        totem_size: v.totem_size,
        glint: v.glint,
        time: v.time,
        weather: v.weather,
        hit: v.hit,
        no_hurt_cam: v.no_hurt_cam,
        no_pumpkin: v.no_pumpkin,
        particles: v.particles,
        chat_hide: v.chat_hide,
        hide_particles: v.hide_particles,
        capture: v.capture,
    });
    let lines = game.arsenal_take_chat();
    let own = game.arsenal_me();
    if !lines.is_empty() || own.is_some() {
        m::chat_lines(&settings, own.as_deref(), &lines);
    }
    let others: Vec<m::Other> = game.arsenal_others().into_iter().map(|o| m::Other {
        id: o.id,
        uuid: o.uuid,
        name: o.name,
        pos: [o.x, o.y, o.z],
        health: o.health,
        absorption: o.absorption,
        hurt_time: o.hurt_time,
        equipment: o.equipment.iter().map(|s| s.as_ref().filter(|s| s.count > 0).map(|s| m::Stack { id: s.item.to_string(), count: s.count, damage: s.damage })).collect(),
    }).collect();
    let events = game.arsenal_entity_events();
    m::players_tick(&settings, &others, &events);
    // Arsenal's own particles, Kill Effects, PvP Tiers / Arsenal Users and Discord.
    let own_id = game.arsenal_status().own_id;
    let own_pop = own_id != 0 && events.iter().any(|(id, e)| *id == own_id && *e == 35);
    m::particles_tick(&settings, &game.arsenal_particles(), own_pop);
    for victim in m::take_kills() {
        m::killed_at(&settings, &others, &victim);
    }
    m::ask_network(&settings, &others);
    m::write_presence(&settings, game.native_player().is_some());
    let status = game.arsenal_status();
    m::status_tick(&settings, &m::Status { attack: status.attack, effects: status.effects, ping: status.ping, latencies: status.latencies },
        m::health().map(|h| h.health + h.absorption));
    let me = game.native_player();
    m::own_tick(&settings, me.as_ref().map(|p| [p.x, p.y, p.z]));
    for message in m::take_outbox() {
        game.arsenal_send_chat(&message);
    }
    let (sounds, notes) = m::take_effects();
    if let Some(p) = me {
        for (sound, volume, pitch) in sounds {
            game.native_play_local_sound(sound, [p.x, p.y, p.z], volume, pitch, 0);
        }
    }
    for note in notes {
        game.native_chat(&note);
    }
}
