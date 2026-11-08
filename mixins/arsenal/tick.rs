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
    let lines = game.native_take_chat();
    let own = game.native_own_name();
    if !lines.is_empty() || own.is_some() {
        m::chat_lines(&settings, own.as_deref(), &lines);
    }
    let others: Vec<m::Other> = game.native_others().into_iter().map(|o| m::Other {
        id: o.id,
        name: o.name,
        pos: [o.x, o.y, o.z],
        health: o.health,
        absorption: o.absorption,
        hurt_time: o.hurt_time,
        equipment: o.equipment.iter().map(|s| s.as_ref().filter(|s| s.count > 0).map(|s| s.item.to_string())).collect(),
    }).collect();
    let events = game.native_take_entity_events();
    m::players_tick(&settings, &others, &events);
    let me = game.native_player();
    m::own_tick(&settings, me.as_ref().map(|p| [p.x, p.y, p.z]));
    for message in m::take_outbox() {
        game.native_send_chat(&message);
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
