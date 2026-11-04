// Less or no distance fog in the open air; water, lava and powder snow keep theirs.
fn arsenal_fog(game: &crate::game::Game<'_>, fog_type: horizon_client::fog::FogType, fog: &mut horizon_client::fog::FogData) {
    use crate::game::arsenal_native as m;
    if fog_type != horizon_client::fog::FogType::Atmospheric || !game.native_on(m::ID) {
        return;
    }
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    (fog.environmental_start, fog.environmental_end) = settings.fog_range(fog.environmental_start, fog.environmental_end);
}
