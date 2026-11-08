// Arsenal Cosmetics: the hats, accessories and pets players wear (the Wardrobe), drawn on them in Java's player-model
// space, each turning with the head or the body as in the Java mod.
fn arsenal_mob_layer(
    game: &crate::game::Game<'_>,
    mob: &crate::game::native_mob_draw::NativeMobRender<'_>,
    who: Option<&crate::game::native_mob_draw::NativePlayerRender<'_>>,
    draw: &mut crate::game::native_block_entity_draw::BlockEntityDraw,
) {
    use crate::game::arsenal_accessories as acc;
    use crate::game::arsenal_hats as hats;
    use crate::game::arsenal_native as m;
    thread_local! {
        static BAKED: std::cell::RefCell<std::collections::HashMap<String, Option<horizon_client::mod_api::ModelPart>>> =
            std::cell::RefCell::new(std::collections::HashMap::new());
    }
    let Some(who) = who else { return };
    if !game.native_on(m::ID) {
        return;
    }
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    if !settings.on(m::COSMETICS) {
        return;
    }
    let uuid = if who.local { game.arsenal_uuid() } else { game.remote_player_uuid(mob.id) };
    let Some(wear) = uuid.and_then(m::worn) else { return };
    if wear.hat.is_none() && wear.accessory.is_none() && wear.pet.is_none() {
        return;
    }
    let crouching = if who.local {
        game.player_model.as_ref().is_some_and(|p| p.extra.crouching)
    } else {
        game.mobs.iter().find(|x| x.id == mob.id).is_some_and(|x| x.extra.crouching)
    };
    let t = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis()) % 1_000_000) as f32 / 1000.0;
    let ctx = acc::Ctx {
        t,
        sneaking: crouching,
        gliding: false,
        swimming_or_flying: false,
        has_chestplate: false,
        walk: mob.walk_position,
        walk_speed: mob.walk_speed,
        cape_lean: 0.0,
        cape_flap: 0.0,
    };
    // Each draw: the layer, its texture, the bone, extra steps, full bright, and what animates it.
    let mut jobs: Vec<(String, String, &'static str, Vec<acc::Step>, bool, &'static str, String)> = Vec::new();
    if let Some(id) = wear.hat.as_deref().filter(|id| hats::HAT_IDS.contains(id)) {
        jobs.push((format!("arsenal:hat_{id}#main"), hats::hat_texture(id), hats::hat_bone(id), Vec::new(), false, "hat", id.to_owned()));
        if hats::hat_has_glow(id) {
            jobs.push((format!("arsenal:hat_{id}#glow"), hats::hat_texture(id), hats::hat_bone(id), Vec::new(), true, "hat", id.to_owned()));
        }
    }
    for (kind, id) in [("accessory", wear.accessory.as_deref()), ("pet", wear.pet.as_deref())] {
        let Some(id) = id else { continue };
        let known = if kind == "pet" { acc::PET_IDS.contains(&id) } else { acc::ACCESSORY_IDS.contains(&id) };
        if !known {
            continue;
        }
        for d in acc::cosmetic_draws(kind, id, &ctx) {
            jobs.push((d.layer, d.texture, d.bone, d.steps, d.glow, kind, id.to_owned()));
        }
    }
    // Java's model space: +y down, front toward -z, feet at y = 24 pixels.
    draw.push();
    draw.scale([1.0, -1.0, -1.0]);
    draw.translate([0.0, -1.501, 0.0]);
    for (layer, texture, bone, steps, glow, kind, id) in jobs {
        draw.push();
        match bone {
            "head" => {
                if crouching {
                    draw.translate([0.0, 4.2 / 16.0, 0.0]);
                }
                draw.rotate_about([0.0, 1.0, 0.0], mob.head_yaw, [0.0; 3]);
                draw.rotate_about([1.0, 0.0, 0.0], mob.pitch, [0.0; 3]);
            }
            "body" => {
                if crouching {
                    draw.translate([0.0, 3.2 / 16.0, 0.0]);
                    draw.rotate_about([1.0, 0.0, 0.0], 0.5f32.to_degrees(), [0.0; 3]);
                }
            }
            "left_arm" => {
                draw.translate([5.0 / 16.0, if crouching { 5.2 } else { 2.0 } / 16.0, 0.0]);
            }
            _ => {}
        }
        for step in steps {
            match step {
                acc::Step::Translate(v) => draw.translate(v),
                acc::Step::RotateX(r) => draw.rotate_about([1.0, 0.0, 0.0], r.to_degrees(), [0.0; 3]),
                acc::Step::RotateY(r) => draw.rotate_about([0.0, 1.0, 0.0], r.to_degrees(), [0.0; 3]),
                acc::Step::RotateZ(r) => draw.rotate_about([0.0, 0.0, 1.0], r.to_degrees(), [0.0; 3]),
                acc::Step::Scale(v) => draw.scale(v),
            }
        }
        BAKED.with_borrow_mut(|baked| {
            let entry = baked.entry(layer.clone()).or_insert_with(|| game.native_bake_layer(&layer));
            if let Some(part) = entry {
                part.reset_all_poses();
                if kind == "hat" {
                    hats::animate_hat(&id, part, t);
                } else {
                    acc::animate_cosmetic(kind, &id, part, &ctx);
                }
                let light = draw.light;
                if glow {
                    draw.light = 0xF0F0;
                }
                game.native_draw_model_part(draw, part, &texture, 0xFFFFFF);
                draw.light = light;
            }
        });
        draw.pop();
    }
    draw.pop();
}
