// Right Shift opens Arsenal's menu (again closes it); the zoom key zooms while held. Both keys are the player's
// choice in Options > Mods > Arsenal.
fn arsenal_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::arsenal_native as m;
    if !game.native_on(m::ID) {
        return;
    }
    // Hitmarker: a hit may land after the attack button.
    if pressed && key == "key.mouse.left" && game.native_playing() {
        m::clicked();
        m::attacked();
    }
    // A module's own key (set on its page): switches it, while playing.
    if pressed && game.native_playing() {
        let raw = {
            let game = &*game;
            m::raw_settings(|name| game.native_setting(m::ID, name))
        };
        if let Some((field, name)) = m::module_key(&raw, key) {
            let (new, value, _) = m::toggled(&raw, field);
            m::set_settings(&new);
            game.native_set_setting(m::ID, m::FIELDS[field].key, value);
            let settings = m::settings(|n| game.native_setting(m::ID, n));
            let text = format!("{name}: {}", if new.v[field] == 1 { "ON" } else { "OFF" });
            if let Some(text) = m::route(&settings, text) {
                game.native_chat(&text);
            }
        }
    }
    // F8: Inventory Cursor Speed off at once.
    if pressed && key == "key.keyboard.f8" {
        let raw = {
            let game = &*game;
            m::raw_settings(|name| game.native_setting(m::ID, name))
        };
        if raw.on(m::CURSOR) {
            let (new, value, _) = m::toggled(&raw, m::CURSOR);
            m::set_settings(&new);
            game.native_set_setting(m::ID, m::FIELDS[m::CURSOR].key, value);
            game.native_chat("Inventory Cursor Speed is off (F8).");
        }
    }
    // Free Look and Quick Menu (held) and Performance Mode's key; a feature to switch comes back.
    for (id, _, fallback) in m::MORE_KEYS {
        if key != game.native_mod_key(m::ID, id, fallback) {
            continue;
        }
        let settings = {
            let game = &*game;
            m::settings(|name| game.native_setting(m::ID, name))
        };
        let playing = game.native_playing();
        let quick = id == m::MORE_KEYS[1].0 && settings.on(m::QUICK_MENU);
        let aim = horizon_client::camera::arsenal::quick_aim();
        if quick {
            horizon_client::camera::arsenal::set_quick(pressed && playing);
        }
        if let Some(field) = m::more_key(&settings, id, pressed, playing, aim) {
            let raw = {
                let game = &*game;
                m::raw_settings(|name| game.native_setting(m::ID, name))
            };
            let (new, value, message) = m::toggled(&raw, field);
            m::set_settings(&new);
            game.native_set_setting(m::ID, m::FIELDS[field].key, value);
            if let Some(message) = m::route(&settings, message) {
                game.native_chat(&message);
            }
        }
    }
    // Chat Hotkeys and Friends (the Friend key on the player you look at).
    if pressed && game.native_playing() {
        let settings = {
            let game = &*game;
            m::settings(|name| game.native_setting(m::ID, name))
        };
        let target = {
            let game = &*game;
            let others: Vec<m::Other> = game.arsenal_others().into_iter().map(|o| m::Other {
                id: o.id, uuid: o.uuid, name: o.name, pos: [o.x, o.y, o.z], health: o.health, absorption: o.absorption, hurt_time: o.hurt_time,
                using: o.using, using_off_hand: o.using_off_hand, equipment: Vec::new(),
            }).collect();
            match (game.native_player(), game.native_camera()) {
                (Some(p), Some(c)) => m::target(&others, [c.position[0] as f64, c.position[1] as f64, c.position[2] as f64], p.yaw, p.pitch).map(|o| o.name.clone()),
                _ => None,
            }
        };
        let bound = |id: &str| {
            let fallback = if id == m::FRIEND_KEY { m::DEFAULT_FRIEND_KEY } else { "key.keyboard.unknown" };
            game.native_mod_key(m::ID, id, fallback)
        };
        if let Some(name) = m::hotkey(&settings, key, &bound, target.as_deref()) {
            let list = m::friends_of(game.native_setting(m::ID, "friend_list"));
            let (stored, added) = m::toggle_friend(&list, &name);
            game.native_set_setting(m::ID, "friend_list", &stored);
            game.native_chat(&if added { format!("{name} is your friend now") } else { format!("{name} isn't your friend any more") });
        }
    }
    if pressed && key == game.native_mod_key(m::ID, m::MENU_KEY, m::DEFAULT_MENU_KEY) {
        if game.native_mod_screen() == Some(m::ID) {
            game.native_mod_screen_close(m::ID);
        } else if game.native_playing() {
            game.native_mod_screen_open(m::ID);
        }
        return;
    }
    if key != game.native_mod_key(m::ID, m::ZOOM_KEY, m::DEFAULT_ZOOM_KEY) {
        return;
    }
    let playing = game.native_playing();
    let game = &*game;
    m::zoom_key(playing, pressed, |name| game.native_setting(m::ID, name));
}
