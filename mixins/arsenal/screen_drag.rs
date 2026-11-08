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
    if button == 0 && menu.page == m::Page::Modules {
        // Dragging over the colour picker keeps picking.
        let mut settings = m::raw_settings(|key| game.native_setting(m::ID, key));
        let browse = m::Fetch::Loading;
        if let Some(action @ m::Action::SetColor(..)) = m::menu_click(screen.rect, screen.mouse, &menu, &settings, &browse, &|t| game.native_text_width(t)) {
            let before = settings.clone();
            if m::apply(action, &mut menu, &mut settings, &browse) {
                m::set_settings(&settings);
                for (key, value) in m::changed(&before, &settings) {
                    game.native_set_setting(m::ID, key, &value);
                }
            }
        }
        m::set_menu(menu);
        return;
    }
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
