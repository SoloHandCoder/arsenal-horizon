// Keys on Arsenal's menu: the search box's Backspace and Escape, the HUD editor's Escape and Ctrl+Z.
fn arsenal_screen_key(
    game: &crate::game::Game<'_>,
    screen: &crate::game::native_hooks::NativeScreenView<'_>,
    key: &str,
    modifiers: u32,
    canceled: &mut bool,
) {
    use crate::game::arsenal_native as m;
    if screen.id != "mod" || game.native_mod_screen() != Some(m::ID) {
        return;
    }
    let mut menu = m::menu();
    let mut settings = m::raw_settings(|key| game.native_setting(m::ID, key));
    let (taken, changed) = m::menu_key(&mut menu, &mut settings, key, modifiers & 2 != 0);
    if changed {
        m::set_settings(&settings);
        for (key, value) in settings.stored() {
            game.native_set_setting(m::ID, key, &value);
        }
    }
    if taken {
        *canceled = true;
        m::set_menu(menu);
    }
}
