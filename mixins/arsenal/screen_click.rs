// A left click on Arsenal's menu: what it lands on, from the same layout as the drawing. Changed settings are kept.
fn arsenal_screen_click(
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
    if button != 0 {
        return;
    }
    let mut settings = m::raw_settings(|key| game.native_setting(m::ID, key));
    let mut menu = m::menu();
    let browse = if menu.page == m::Page::Browse { m::gallery(menu.tab) } else { m::Fetch::Loading };
    let Some(action) = m::menu_click(screen.rect, screen.mouse, &menu, &settings, &browse, &|text| game.native_text_width(text)) else {
        return;
    };
    if m::apply(action, &mut menu, &mut settings, &browse) {
        m::set_settings(&settings);
        for (key, value) in settings.stored() {
            game.native_set_setting(m::ID, key, &value);
        }
    }
    m::set_menu(menu);
}
