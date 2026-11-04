// Arsenal's row in Options > Mods, with its zoom key.
fn arsenal_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::arsenal_native as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[m::ZOOM_KEY.into(), "Zoom".into(), m::DEFAULT_ZOOM_KEY.into()]],
    });
}
