// The zoom key went down or up: the player's choice of key (Options > Mods > Arsenal).
fn arsenal_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::arsenal_native as m;
    if !game.native_on(m::ID) {
        return;
    }
    if key != game.native_mod_key(m::ID, m::ZOOM_KEY, m::DEFAULT_ZOOM_KEY) {
        return;
    }
    let playing = game.native_playing();
    let game = &*game;
    m::zoom_key(playing, pressed, |name| game.native_setting(m::ID, name));
}
