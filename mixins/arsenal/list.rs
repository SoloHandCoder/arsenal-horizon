// Arsenal's row in Options > Mods, with its keys: the menu and zoom.
fn arsenal_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::arsenal_native as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![
            [m::MENU_KEY.into(), "Arsenal menu".into(), m::DEFAULT_MENU_KEY.into()],
            [m::ZOOM_KEY.into(), "Zoom".into(), m::DEFAULT_ZOOM_KEY.into()],
            [m::FRIEND_KEY.into(), "Friend (the player you look at)".into(), m::DEFAULT_FRIEND_KEY.into()],
            [m::HOTKEY_KEYS[0].0.into(), m::HOTKEY_KEYS[0].1.into(), "key.keyboard.unknown".into()],
            [m::HOTKEY_KEYS[1].0.into(), m::HOTKEY_KEYS[1].1.into(), "key.keyboard.unknown".into()],
            [m::HOTKEY_KEYS[2].0.into(), m::HOTKEY_KEYS[2].1.into(), "key.keyboard.unknown".into()],
            [m::HOTKEY_KEYS[3].0.into(), m::HOTKEY_KEYS[3].1.into(), "key.keyboard.unknown".into()],
            [m::MORE_KEYS[0].0.into(), m::MORE_KEYS[0].1.into(), m::MORE_KEYS[0].2.into()],
            [m::MORE_KEYS[1].0.into(), m::MORE_KEYS[1].1.into(), m::MORE_KEYS[1].2.into()],
            [m::MORE_KEYS[2].0.into(), m::MORE_KEYS[2].1.into(), m::MORE_KEYS[2].2.into()],
        ],
    });
}
