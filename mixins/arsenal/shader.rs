// Arsenal's shader pack reads these each frame (frame.mods): which of Bloom, World Look, Sky & World Colors, Motion
// Blur, Mist and Sky Effects are on, and how strong.
fn arsenal_shader(game: &crate::game::Game<'_>, params: &mut [[f32; 4]; 8]) {
    use crate::game::arsenal_native as m;
    if !game.native_on(m::ID) {
        return;
    }
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    *params = m::shader_params(&settings);
}
