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
    crate::game::native_hooks::ARSENAL_CRYSTAL_OPT.store(v.crystal_optimizer, std::sync::atomic::Ordering::Relaxed);
    // Fullbright's brightness from before, kept in Arsenal's settings.
    match v.keep_gamma {
        Some(g) if g >= 0.0 => game.native_set_setting(m::ID, "saved_gamma", &g.to_string()),
        Some(_) => game.native_set_setting(m::ID, "saved_gamma", ""),
        None => {}
    }
    if v.restore_gamma {
        if let Some(g) = game.native_setting(m::ID, "saved_gamma").and_then(|g| g.parse::<f32>().ok()) {
            game.arsenal_visuals(&crate::game::native_hooks::ArsenalVisuals { gamma: Some(g), particles: 100, ..Default::default() });
            game.native_set_setting(m::ID, "saved_gamma", "");
        }
    }
    game.arsenal_cursor_tick();
    // This version's looks, chat, sounds and input (Arsenal's patches/more.json).
    {
        use horizon_client::camera::arsenal as a;
        use std::sync::atomic::Ordering::Relaxed;
        let own = game.arsenal_me();
        let l = m::looks(&settings, own.as_deref());
        a::HOTBAR.store(l.hotbar, Relaxed);
        a::HOTBAR_ACCENT.store(l.hotbar_accent, Relaxed);
        a::HOTBAR_GLIDE.store(l.glide, Relaxed);
        a::INV_FRAME.store(l.inv_frame, Relaxed);
        a::INV_GLIDE.store(l.inv_glide, Relaxed);
        a::SWING.store(l.swing, Relaxed);
        a::LOWER.store(l.lower, Relaxed);
        a::BLOCKHIT.store(l.blockhit, Relaxed);
        a::HAND_TINT.store(l.hand_tint, Relaxed);
        a::HAND_GLOW.store(l.hand_glow, Relaxed);
        a::CRYSTAL_SIZE.store(l.crystal_size, Relaxed);
        a::CRYSTAL_STILL.store(l.crystal_still, Relaxed);
        a::ARMOR_TINT.store(l.armor_tint, Relaxed);
        a::TAG_SIZE.store(l.tag_size, Relaxed);
        a::TAG_BG.store(l.tag_bg, Relaxed);
        if a::TAB_PING.swap(l.tab_ping, Relaxed) != l.tab_ping {
            a::TAB_DIRTY.store(true, Relaxed);
        }
        a::CHAT_KEEP.store(l.chat_keep, Relaxed);
        a::OBSIDIAN_GUARD.store(l.obsidian_guard, Relaxed);
        a::CRYSTAL_HELP.store(l.crystal_help, Relaxed);
        if let Ok(mut b) = a::BIG_ITEMS.write() {
            if *b != l.big_items {
                *b = l.big_items.clone();
            }
        }
        if let Ok(mut n) = a::NAME_HIDE.write() {
            if *n != l.name_hide {
                *n = l.name_hide.clone();
            }
        }
        if let Ok(mut n) = a::MENTION.write() {
            if *n != l.mention {
                *n = l.mention.clone();
            }
        }
        if let Ok(mut v) = a::VOLUMES.write() {
            if *v != l.volumes {
                *v = l.volumes.clone();
            }
        }
        game.arsenal_more(l.freelook, l.chat, l.reconnect);
        let events = a::BLOCKS.lock().map(|mut b| std::mem::take(&mut *b)).unwrap_or_default();
        m::blocks_tick(&settings, &events);
        m::sprint_tick(game.arsenal_sprinting());
        if let Some(path) = a::SHOT.lock().ok().and_then(|mut s| s.take()) {
            if settings.on(m::SHOT_CLIP) {
                m::copy_image(&path);
            }
        }
        let dir = game.native_data_dir(m::ID);
        m::crash_hook(&dir);
        if let Some(note) = m::crash_note(&settings, &dir) {
            game.native_chat(&note);
        }
        if let Some(note) = m::update_note(&settings) {
            if let Some(note) = m::route(&settings, note) {
                game.native_chat(&note);
            }
        }
        if m::hello_due(&settings) {
            let signed = game.arsenal_signed("arsenal-hello-v1", &[m::VERSION.to_string(), "26.3".to_string()]);
            m::send_hello(&settings, signed);
        }
        m::ask_wardrobe(&settings, game.arsenal_uuid());
        m::lyrics_tick(&settings);
        // Hypixel SkyBlock: the sidebar and tab list (Arsenal's patches/more.json).
        let sidebar = game.arsenal_sidebar();
        let tab = game.arsenal_tab_lines();
        if let Some(note) = m::skyblock_tick(&settings, game.arsenal_server().as_deref(), &sidebar, &tab) {
            if let Some(note) = m::route(&settings, note) {
                game.native_chat(&note);
            }
        }
        // Cloud Backup: requests signed with your Mojang chat key, as the Java mod's.
        let raw = {
            let game = &*game;
            m::raw_settings(|key| game.native_setting(m::ID, key))
        };
        let cloud = {
            let game = &*game;
            m::cloud_tick(&settings, &raw, &|prefix, extra| game.arsenal_signed(prefix, extra))
        };
        if !cloud.restore.is_empty() {
            for (key, value) in &cloud.restore {
                game.native_set_setting(m::ID, key, value);
            }
            let fresh = {
                let game = &*game;
                m::Settings::read(|key| game.native_setting(m::ID, key))
            };
            m::set_settings(&fresh);
        }
        if let Some(note) = cloud.say {
            game.native_chat(&note);
        }
        // Inventory Cursor Speed (its own cursor is drawn by gui_post.rs).
        a::CURSOR_SPEED.store(if settings.on(m::CURSOR) { m::FIELDS[m::CURSOR_SPEED].options[settings.v[m::CURSOR_SPEED]].0.parse().unwrap_or(100) } else { 100 }, Relaxed);
        // Arsenal capes and Wardrobe blades (yours and other Arsenal users'), and Arsenal's cosmetics pack.
        let dressing = m::dressing(&settings, game.arsenal_uuid(), &game.native_data_dir(m::ID), &game.native_instance_dir().join("resourcepacks"));
        if dressing.pack {
            game.arsenal_enable_pack();
        }
        if horizon_client::camera::arsenal::set_blades(dressing.blades) | horizon_client::camera::arsenal::set_capes(dressing.capes) {
            game.arsenal_capes_changed();
        }
        let server = game.arsenal_server();
        m::set_server(server.as_deref());
        // Share codes: the clipboard (copy, or read a code to paste).
        match m::take_clip() {
            Some(m::Clip::Copy(text)) => game.arsenal_clipboard_set(&text),
            Some(m::Clip::Paste) => {
                let text = game.arsenal_clipboard_get();
                let (pairs, say) = m::pasted(&text);
                if let Some(pairs) = pairs {
                    for (key, value) in &pairs {
                        game.native_set_setting(m::ID, key, value);
                    }
                    let fresh = {
                        let game = &*game;
                        m::Settings::read(|key| game.native_setting(m::ID, key))
                    };
                    m::set_settings(&fresh);
                }
                game.native_chat(&say);
            }
            None => {}
        }
        let joined = {
            let game = &*game;
            m::server_tick(&settings, server.as_deref(), &|key| game.native_setting(m::ID, key))
        };
        if let Some(name) = server.as_deref().map(m::server_of).and_then(|k| {
            let raw = {
                let game = &*game;
                m::raw_settings(|key| game.native_setting(m::ID, key))
            };
            m::joined_profile(&raw, &k)
        }) {
            let raw = {
                let game = &*game;
                m::raw_settings(|key| game.native_setting(m::ID, key))
            };
            if let Some(loaded) = m::load_profile(&raw, &name) {
                m::set_settings(&loaded);
                for (key, value) in m::changed(&raw, &loaded) {
                    game.native_set_setting(m::ID, key, &value);
                }
                game.native_chat(&format!("Profile \"{name}\" loaded for this server."));
            }
        }
        for j in joined {
            match j {
                m::Joined::Note(note) => game.native_chat(&note),
                m::Joined::Profile(stored) => {
                    for (key, value) in m::profile_pairs(&stored) {
                        game.native_set_setting(m::ID, &key, &value);
                    }
                    let fresh = {
                        let game = &*game;
                        m::Settings::read(|key| game.native_setting(m::ID, key))
                    };
                    m::set_settings(&fresh);
                    game.native_chat("Loaded your setup for this server.");
                }
            }
        }
    }
    // Arsenal's shader pack goes into shaderpacks/ (once, and again when the mod brings a new version).
    m::install_pack_once(&game.native_instance_dir());
    let lines = game.arsenal_take_chat();
    let own = game.arsenal_me();
    if !lines.is_empty() || own.is_some() {
        m::chat_lines(&settings, own.as_deref(), &lines);
    }
    m::skyblock_chat(&lines);
    let others: Vec<m::Other> = game.arsenal_others().into_iter().map(|o| m::Other {
        id: o.id,
        uuid: o.uuid,
        name: o.name,
        pos: [o.x, o.y, o.z],
        health: o.health,
        absorption: o.absorption,
        hurt_time: o.hurt_time,
        using: o.using,
        using_off_hand: o.using_off_hand,
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
    let offhand_totem = game.arsenal_inventory().is_some_and(|v| v.offhand.as_ref().is_some_and(|s| &*s.item == "minecraft:totem_of_undying" && s.count > 0));
    m::more_tick(&settings, &others, &events, own_id, offhand_totem);
    // The Arsenal website: fights, session parts, presence, friends, quests (signed like the Java mod's).
    {
        let health = m::health().map(|h| h.health);
        let address = game.arsenal_server();
        for up in m::website_tick(&settings, address.as_deref(), &others, health) {
            if let Some(signed) = game.arsenal_signed(up.prefix, &up.extra) {
                m::send_upload(up, signed);
            }
        }
        let (notes, friends) = m::take_web();
        for note in notes {
            if let Some(note) = m::route(&settings, note) {
                game.native_chat(&note);
            }
        }
        if !friends.is_empty() {
            let mut list = m::friends(|key| game.native_setting(m::ID, key));
            let before = list.len();
            for f in friends {
                if !list.iter().any(|x| x.eq_ignore_ascii_case(&f)) {
                    list.push(f);
                }
            }
            if list.len() != before {
                game.native_set_setting(m::ID, "friend_list", &list.join(","));
                m::set_friends(list);
            }
        }
    }
    let mace = game.arsenal_inventory().is_some_and(|v| v.slots.get(v.selected).cloned().flatten().is_some_and(|s| &*s.item == "minecraft:mace"));
    m::mace_hit(&settings, &others, mace);
    // Session Summary: leaving a world ends the session.
    m::world_change(game.native_player().is_some());
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
        if let Some(note) = m::route(&settings, note) {
            game.native_chat(&note);
        }
    }
}
