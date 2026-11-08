// Arsenal's settings page: one button per setting, each click moves it on.
fn arsenal_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::arsenal_native as m;
    if mod_id != m::ID {
        return;
    }
    let game = &*game;
    buttons.extend(m::raw_settings(|key| game.native_setting(m::ID, key)).labels());
}
