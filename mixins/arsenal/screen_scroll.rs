// The wheel over Arsenal's menu scrolls its module list or Browse's cards.
fn arsenal_screen_scroll(
    game: &crate::game::Game<'_>,
    screen: &crate::game::native_hooks::NativeScreenView<'_>,
    scroll: [f64; 2],
    canceled: &mut bool,
) {
    use crate::game::arsenal_native as m;
    if screen.id != "mod" || game.native_mod_screen() != Some(m::ID) {
        return;
    }
    *canceled = true;
    if scroll[1] != 0.0 {
        let mut menu = m::menu();
        let mut settings = m::raw_settings(|key| game.native_setting(m::ID, key));
        if menu.page == m::Page::HudEdit {
            if m::hud_scroll(&mut menu, &mut settings, &m::hud_boxes(), screen.mouse, scroll[1]) {
                m::set_settings(&settings);
                for (key, value) in settings.stored() {
                    game.native_set_setting(m::ID, key, &value);
                }
            }
        } else {
            let browse = if menu.page == m::Page::Browse { m::gallery(menu.tab) } else { m::Fetch::Loading };
            m::menu_scroll(&mut menu, &settings, scroll[1], screen.rect, &browse);
        }
        m::set_menu(menu);
    }
}
