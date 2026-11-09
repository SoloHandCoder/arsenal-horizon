// Less or no distance fog in the open air (the render distance's fog and the air's); water, lava, powder snow,
// blindness and darkness keep theirs.
fn arsenal_fog(game: &crate::game::Game<'_>, fog_type: horizon_client::fog::FogType, fog: &mut horizon_client::fog::FogData) {
    use crate::game::arsenal_native as m;
    if fog_type != horizon_client::fog::FogType::Atmospheric || !game.native_on(m::ID) {
        return;
    }
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    // Blindness and Darkness pull the fog in to a few blocks: those stay.
    if fog.environmental_end < 32.0 {
        return;
    }
    (fog.environmental_start, fog.environmental_end) = settings.fog_range(fog.environmental_start, fog.environmental_end);
    (fog.render_distance_start, fog.render_distance_end) = settings.fog_range(fog.render_distance_start, fog.render_distance_end);
}
