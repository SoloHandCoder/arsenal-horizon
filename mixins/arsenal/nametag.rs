// Hide Nametags: no name tag above players (or mobs too).
fn arsenal_nametag(game: &crate::game::Game<'_>, kind: &str, visible: &mut bool) {
    use crate::game::arsenal_native as m;
    if *visible && game.native_on(m::ID) && m::settings(|key| game.native_setting(m::ID, key)).hides_tag(kind) {
        *visible = false;
    }
}
