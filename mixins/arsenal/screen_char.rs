// Typing on Arsenal's menu goes into the module search.
fn arsenal_screen_char(
    game: &crate::game::Game<'_>,
    screen: &crate::game::native_hooks::NativeScreenView<'_>,
    c: char,
    canceled: &mut bool,
) {
    use crate::game::arsenal_native as m;
    if screen.id != "mod" || game.native_mod_screen() != Some(m::ID) {
        return;
    }
    let mut menu = m::menu();
    if m::menu_typed(&mut menu, c) {
        *canceled = true;
        m::set_menu(menu);
    }
}
