// The held item (and the empty hand) at the size the player picked; a Wardrobe blade at Blade Settings' size, spun by
// its inspect key.
fn arsenal_hand(game: &crate::game::Game<'_>, stack: Option<&horizon_client::items::ItemStack>, yaw: &mut f32, scale: &mut f32) {
    use crate::game::arsenal_native as m;
    if !game.native_on(m::ID) {
        return;
    }
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    *scale *= settings.item_scale();
    let sword = stack.is_some_and(|s| s.item.ends_with("_sword"));
    let own = game.arsenal_uuid();
    let has_blade = own.is_some_and(|u| horizon_client::camera::arsenal::blade_for(u).is_some());
    if let Some((size, turn)) = m::blade_pose(&settings, sword, has_blade) {
        *scale *= size;
        *yaw += turn;
    }
}
