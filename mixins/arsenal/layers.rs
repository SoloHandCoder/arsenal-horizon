// Arsenal Cosmetics' models: the Wardrobe's hats, accessories and pets as layers, baked when first worn.
fn arsenal_layers(layers: &mut Vec<(String, horizon_client::mod_api::LayerDefinition)>) {
    layers.extend(crate::game::arsenal_hats::hat_layers());
    layers.extend(crate::game::arsenal_accessories::accessory_layers());
}
