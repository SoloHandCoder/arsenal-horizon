// Right Shift opens Arsenal's menu (again closes it); the zoom key zooms while held. Both keys are the player's
// choice in Options > Mods > Arsenal.
fn arsenal_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::arsenal_native as m;
    if !game.native_on(m::ID) {
        return;
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
