// Dragging on Arsenal's menu: the HUD editor's element follows the pointer.
fn arsenal_screen_drag(
    game: &crate::game::Game<'_>,
    screen: &crate::game::native_hooks::NativeScreenView<'_>,
    button: i32,
    modifiers: u32,
    canceled: &mut bool,
) {
    use crate::game::arsenal_native as m;
    if screen.id != "mod" || game.native_mod_screen() != Some(m::ID) {
        return;
    }
    *canceled = true;
    let mut menu = m::menu();
    if button != 0 || menu.page != m::Page::HudEdit {
        return;
    }
    let mut settings = m::raw_settings(|key| game.native_setting(m::ID, key));
    let size = [screen.rect[0] + screen.rect[2], screen.rect[1] + screen.rect[3]];
    if m::hud_drag(&mut menu, &mut settings, &m::hud_boxes(), screen.mouse, size, modifiers & 1 != 0) {
        m::set_settings(&settings);
    }
    m::set_menu(menu);
}
