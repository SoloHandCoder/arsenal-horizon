// Letting go on Arsenal's menu: the HUD editor's element stays where it was put (and is kept).
fn arsenal_screen_release(
    game: &crate::game::Game<'_>,
    screen: &crate::game::native_hooks::NativeScreenView<'_>,
    button: i32,
    canceled: &mut bool,
) {
    use crate::game::arsenal_native as m;
    if screen.id != "mod" || game.native_mod_screen() != Some(m::ID) {
        return;
    }
    *canceled = true;
    let mut menu = m::menu();
    if button == 0 && menu.drag.is_some() {
        let moved = menu.drag.as_ref().is_some_and(|d| d.moved);
        m::hud_release(&mut menu);
        if moved {
            let settings = m::raw_settings(|key| game.native_setting(m::ID, key));
            for (key, value) in settings.stored() {
                if key == "hud_layout" || key == "server_huds" {
                    game.native_set_setting(m::ID, key, &value);
                }
            }
        }
        m::set_menu(menu);
    }
}
