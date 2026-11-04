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
        let browse = if menu.page == m::Page::Browse { m::gallery(menu.tab) } else { m::Fetch::Loading };
        m::menu_scroll(&mut menu, scroll[1], screen.rect, &browse);
        m::set_menu(menu);
    }
}
