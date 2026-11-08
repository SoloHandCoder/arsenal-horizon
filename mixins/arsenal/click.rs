// A click on Arsenal's settings page (Options > Mods > Arsenal): change the setting and keep it.
fn arsenal_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::arsenal_native as m;
    if mod_id != m::ID {
        return;
    }
    let mut settings = m::raw_settings(|key| game.native_setting(m::ID, key));
    let before = settings.clone();
    settings.click(button);
    m::set_settings(&settings);
    for (key, value) in m::changed(&before, &settings) {
        game.native_set_setting(m::ID, key, &value);
    }
}
