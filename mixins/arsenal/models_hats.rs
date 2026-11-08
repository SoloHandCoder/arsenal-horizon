// Arsenal's Wardrobe hats, ported from HatLayer.java. Every hat sits under one root
// child "hat" (posed at ZERO) so the whole hat can be moved; the parts under it keep
// Java's names and numbers. Units are texels and radians, +y down, as in Java.

use crate::mod_api::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartId, PartPose};

/// Every hat id, in the order HatLayer lists them.
pub const HAT_IDS: &[&str] = &[
    "og_top_hat",
    "crown",
    "wizard_hat",
    "viking_helmet",
    "cat_ears",
    "devil_horns",
    "headphones",
    "laurel_wreath",
    "witch_hat",
    "bunny_ears",
    "pumpkin_head",
    "santa_hat",
    "bandana",
    "antlers",
    "floating_crown",
    "first_blood",
    "golden_reaper",
    "combo_king",
    "creator_beret",
    "winter_scarf",
    "birthday_hat",
    "chef_hat",
    "pirate_hat",
    "halo_crown",
    "focus_headband",
    "party_hat_deluxe",
    "jester_hat",
    "bandage",
    "scarecrow_hat",
    "elf_hat",
    "sun_hat",
    "heart_crown",
    "beanie",
    "samurai_helmet",
    "war_helm",
    "vip_hat",
    "supporter_hat",
];

/// Builds a hat's parts under its "hat" part and gives the texture size.
type Build = fn(&mut MeshDefinition, PartId) -> (u32, u32);

/// One layer per hat (`#main`), plus a `#glow` layer for the hats Java draws a
/// second, full-bright model for (map GLOW).
pub fn hat_layers() -> Vec<(String, LayerDefinition)> {
    let mut out = Vec::new();
    for id in HAT_IDS {
        if let Some(build) = model_of(id) {
            out.push((format!("arsenal:hat_{id}#main"), layer(build)));
        }
        if let Some(build) = glow_of(id) {
            out.push((format!("arsenal:hat_{id}#glow"), layer(build)));
        }
    }
    out
}

/// Whether this hat has a `#glow` layer. Java draws it with the same texture at
/// full brightness (light 0xF000F0); the renderer has to do that part.
pub fn hat_has_glow(id: &str) -> bool {
    glow_of(id).is_some()
}

/// The bone the hat follows: the scarf is worn on the neck, so it follows the body.
pub fn hat_bone(id: &str) -> &'static str {
    if id == "winter_scarf" {
        "body"
    } else {
        "head"
    }
}

/// Each hat's own texture (VIP and Supporter too, though they share a model).
pub fn hat_texture(id: &str) -> String {
    format!("arsenal:item/hat_{id}")
}

/// Moves a hat by time, as HatLayer.draw/animate do. `model` is the baked root of
/// either of the hat's layers, after reset_all_poses(); parts a layer lacks are skipped.
pub fn animate_hat(id: &str, model: &mut ModelPart, t: f32) {
    let t = t as f64;
    let sin = f64::sin;
    match id {
        "floating_crown" => {
            // bobs above the head and turns (applies to the gems too, as in Java)
            set(model, &[], |p| {
                p.y = (-12.0 + sin(t * 1.8) * 0.75) as f32;
                p.y_rot = (t * 0.9) as f32;
            });
        }
        "halo_crown" | "vip_hat" => {
            // only the halo (glow layer) bobs; the crown stays put
            set(model, &["ring"], |p| p.y = (-14.5 + sin(t * 2.2) * 0.4) as f32);
        }
        "scarecrow_hat" => {
            // the crow on the brim looks around now and then
            set(model, &["crow", "head"], |p| p.y_rot = (sin(t * 0.7) * 0.6 + sin(t * 1.9) * 0.2) as f32);
        }
        "elf_hat" => {
            // the bell on the tip swings
            set(model, &["mid", "upper", "tip", "end"], |p| p.z_rot = (sin(t * 2.2) * 0.25) as f32);
            set(model, &["mid", "upper", "tip", "end", "bell"], |p| p.z_rot = (sin(t * 2.2 - 0.8) * 0.4) as f32);
        }
        "sun_hat" => {
            // the ribbon's tails flutter
            set(model, &["bow", "right_tail"], |p| p.x_rot = (0.25 + sin(t * 2.6) * 0.15) as f32);
            set(model, &["bow", "left_tail"], |p| p.x_rot = (0.25 + sin(t * 2.6 + 1.1) * 0.15) as f32);
        }
        "wizard_hat" | "witch_hat" => {
            // the bent tip nods and sways
            let base = if id == "wizard_hat" { -0.45 } else { -0.6 };
            set(model, &["tip"], |p| {
                p.x_rot = (base + sin(t * 1.6) * 0.08) as f32;
                p.z_rot = (sin(t * 1.1) * 0.06) as f32;
            });
        }
        "bunny_ears" => {
            // sway gently, and now and then one flicks
            let flick = sin(t * 0.9).max(0.0).powi(30) * sin(t * 22.0) * 0.16;
            set(model, &["left"], |p| p.z_rot = (-0.14 + sin(t * 1.7) * 0.05) as f32);
            set(model, &["right"], |p| p.z_rot = (0.14 - sin(t * 1.7 + 0.5) * 0.05 + flick) as f32);
        }
        "santa_hat" => {
            // the floppy end swings, each part a little after the one before it
            set(model, &["mid"], |p| p.z_rot = (0.3 + sin(t * 1.5) * 0.04) as f32);
            set(model, &["mid", "tip"], |p| p.z_rot = (0.55 + sin(t * 1.5 - 0.6) * 0.07) as f32);
            set(model, &["mid", "tip", "end"], |p| p.z_rot = (0.7 + sin(t * 1.5 - 1.2) * 0.12) as f32);
        }
        "bandana" | "first_blood" => {
            set(model, &["tailL"], |p| p.x_rot = (0.4 + sin(t * 2.4) * 0.14) as f32);
            set(model, &["tailR"], |p| p.x_rot = (0.4 + sin(t * 2.4 + 1.2) * 0.14) as f32);
        }
        "focus_headband" => {
            set(model, &["tailL"], |p| p.x_rot = (0.5 + sin(t * 2.4) * 0.14) as f32);
            set(model, &["tailR"], |p| p.x_rot = (0.35 + sin(t * 2.4 + 1.2) * 0.14) as f32);
        }
        "winter_scarf" => {
            set(model, &["long"], |p| p.x_rot = (sin(t * 1.4) * 0.05) as f32);
            set(model, &["short"], |p| p.x_rot = (sin(t * 1.4 + 0.9) * 0.06) as f32);
        }
        "jester_hat" => {
            // the bells swing
            set(model, &["right", "mid", "tip", "bell"], |p| p.x_rot = (sin(t * 3.1) * 0.4) as f32);
            set(model, &["left", "mid", "tip", "bell"], |p| p.x_rot = (sin(t * 3.1 + 1.4) * 0.4) as f32);
        }
        "bandage" => set(model, &["end"], |p| p.x_rot = (0.3 + sin(t * 2.2) * 0.12) as f32),
        "beanie" => {
            // the pompom wobbles on the top
            set(model, &["pompom"], |p| {
                p.z_rot = (sin(t * 2.6) * 0.18) as f32;
                p.x_rot = (sin(t * 2.1 + 0.7) * 0.12) as f32;
            });
        }
        "war_helm" => {
            set(model, &["plume"], |p| {
                p.x_rot = (sin(t * 1.4) * 0.04) as f32;
                p.z_rot = (sin(t * 1.9) * 0.05) as f32;
            });
        }
        "samurai_helmet" => set(model, &["back_guard"], |p| p.x_rot = (0.35 + sin(t * 1.3) * 0.04) as f32),
        _ => {}
    }
}

/// Runs `f` on the part at `path` under the "hat" part, if it is there.
fn set(model: &mut ModelPart, path: &[&str], f: impl FnOnce(&mut ModelPart)) {
    let Some(mut p) = model.get_child_mut("hat") else { return };
    for name in path {
        match p.get_child_mut(name) {
            Some(child) => p = child,
            None => return,
        }
    }
    f(p);
}

fn layer(build: Build) -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    let hat = mesh.add_or_replace_child(root, "hat", CubeListBuilder::create(), PartPose::ZERO);
    let (w, h) = build(&mut mesh, hat);
    LayerDefinition::create(mesh, w, h)
}

fn model_of(id: &str) -> Option<Build> {
    let build: Build = match id {
        "og_top_hat" => top_hat,
        "crown" => crown,
        "wizard_hat" => wizard_hat,
        "viking_helmet" => viking_helmet,
        "cat_ears" => cat_ears,
        "devil_horns" => devil_horns,
        "headphones" => headphones,
        "laurel_wreath" => laurel_wreath,
        "witch_hat" => witch_hat,
        "bunny_ears" => bunny_ears,
        "pumpkin_head" => pumpkin_head,
        "santa_hat" => santa_hat,
        "bandana" => bandana,
        "antlers" => antlers,
        "floating_crown" => floating_crown,
        "first_blood" => first_blood,
        "golden_reaper" => golden_reaper,
        "combo_king" => combo_king,
        "creator_beret" => creator_beret,
        "winter_scarf" => winter_scarf,
        "birthday_hat" => birthday_hat,
        "chef_hat" => chef_hat,
        "pirate_hat" => pirate_hat,
        "halo_crown" | "vip_hat" => halo_crown,
        "focus_headband" => focus_headband,
        "party_hat_deluxe" => party_hat_deluxe,
        "jester_hat" => jester_hat,
        "bandage" => bandage,
        "scarecrow_hat" => scarecrow_hat,
        "elf_hat" => elf_hat,
        "sun_hat" => sun_hat,
        "heart_crown" | "supporter_hat" => heart_crown,
        "beanie" => beanie,
        "samurai_helmet" => samurai_helmet,
        "war_helm" => war_helm,
        _ => return None,
    };
    Some(build)
}

fn glow_of(id: &str) -> Option<Build> {
    let build: Build = match id {
        "pumpkin_head" => pumpkin_face,
        "floating_crown" => floating_crown_gems,
        "combo_king" => combo_king_gems,
        "halo_crown" | "vip_hat" => halo_ring,
        "party_hat_deluxe" => party_hat_glitter,
        "heart_crown" | "supporter_hat" => heart_crown_gems,
        _ => return None,
    };
    Some(build)
}

fn c() -> CubeListBuilder {
    CubeListBuilder::create()
}

fn grow(x: f32, y: f32, z: f32) -> CubeDeformation {
    CubeDeformation::xyz(x, y, z)
}

fn pr(x: f32, y: f32, z: f32, xr: f32, yr: f32, zr: f32) -> PartPose {
    PartPose::offset_and_rotation(x, y, z, xr, yr, zr)
}

const Z: PartPose = PartPose::ZERO;

// An 11x11 brim resting on the head and a 7x7x7 crown on top.
fn top_hat(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "brim", c().tex_offs(0, 0).add_box([-5.5, -9.5, -5.5], [11.0, 1.0, 11.0]), Z);
    m.add_or_replace_child(r, "crown", c().tex_offs(0, 12).add_box([-3.5, -16.5, -3.5], [7.0, 7.0, 7.0]), Z);
    (64, 32)
}

// A gold band with spikes and a taller front gem.
fn crown(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "front", c().tex_offs(0, 0).add_box([-4.5, -10.5, -4.5], [9.0, 2.0, 1.0]), Z);
    m.add_or_replace_child(r, "back", c().tex_offs(0, 4).add_box([-4.5, -10.5, 3.5], [9.0, 2.0, 1.0]), Z);
    m.add_or_replace_child(r, "left", c().tex_offs(0, 8).add_box([-4.5, -10.5, -3.5], [1.0, 2.0, 7.0]), Z);
    m.add_or_replace_child(r, "right", c().tex_offs(20, 8).add_box([3.5, -10.5, -3.5], [1.0, 2.0, 7.0]), Z);
    let mut spikes = c().tex_offs(40, 0);
    let at = [[-4.5, -4.5], [3.5, -4.5], [-4.5, 3.5], [-0.5, 3.5], [3.5, 3.5], [-4.5, -0.5], [3.5, -0.5]];
    for s in at {
        spikes = spikes.add_box([s[0], -12.5, s[1]], [1.0, 2.0, 1.0]);
    }
    m.add_or_replace_child(r, "spikes", spikes, Z);
    m.add_or_replace_child(r, "gem", c().tex_offs(48, 0).add_box([-0.5, -13.5, -4.5], [1.0, 3.0, 1.0]), Z);
    (64, 32)
}

// A wide brim, two cone steps and a tip that bends back.
fn wizard_hat(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "brim", c().tex_offs(0, 0).add_box([-6.0, -9.5, -6.0], [12.0, 1.0, 12.0]), Z);
    m.add_or_replace_child(r, "low", c().tex_offs(0, 13).add_box([-4.0, -13.5, -4.0], [8.0, 4.0, 8.0]), Z);
    m.add_or_replace_child(r, "mid", c().tex_offs(0, 25).add_box([-3.0, -17.5, -3.0], [6.0, 4.0, 6.0]), Z);
    m.add_or_replace_child(
        r,
        "tip",
        c().tex_offs(32, 13).add_box([-2.0, -4.0, -2.0], [4.0, 4.0, 4.0]).tex_offs(32, 25).add_box([-1.0, -7.0, 0.0], [2.0, 3.0, 2.0]),
        pr(0.0, -17.5, 0.0, -0.45, 0.0, 0.0),
    );
    (64, 64)
}

// An iron dome with a nose guard and two horns.
fn viking_helmet(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "dome", c().tex_offs(0, 0).add_box([-5.0, -10.0, -5.0], [10.0, 5.0, 10.0]), Z);
    m.add_or_replace_child(r, "nose", c().tex_offs(40, 0).add_box([-0.5, -5.0, -5.5], [1.0, 3.0, 1.0]), Z);
    m.add_or_replace_child(
        r,
        "hornL",
        c().tex_offs(0, 16).add_box([-8.0, -9.0, -1.0], [3.0, 2.0, 2.0]).tex_offs(0, 20).add_box([-9.0, -12.0, -1.0], [2.0, 3.0, 2.0]),
        Z,
    );
    m.add_or_replace_child(
        r,
        "hornR",
        c().tex_offs(12, 16).add_box([5.0, -9.0, -1.0], [3.0, 2.0, 2.0]).tex_offs(12, 20).add_box([7.0, -12.0, -1.0], [2.0, 3.0, 2.0]),
        Z,
    );
    (64, 32)
}

// Two small pointed ears.
fn cat_ears(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "left", c().tex_offs(0, 0).add_box([-4.0, -11.0, -1.0], [3.0, 3.0, 1.0]), Z);
    m.add_or_replace_child(r, "right", c().tex_offs(10, 0).add_box([1.0, -11.0, -1.0], [3.0, 3.0, 1.0]), Z);
    (64, 32)
}

// A band of laurel leaves with leaf tips standing out.
fn laurel_wreath(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "front", c().tex_offs(0, 0).add_box([-4.5, -9.5, -4.5], [9.0, 2.0, 1.0]), Z);
    m.add_or_replace_child(r, "back", c().tex_offs(0, 4).add_box([-4.5, -9.5, 3.5], [9.0, 2.0, 1.0]), Z);
    m.add_or_replace_child(r, "left", c().tex_offs(0, 8).add_box([-4.5, -9.5, -3.5], [1.0, 2.0, 7.0]), Z);
    m.add_or_replace_child(r, "right", c().tex_offs(20, 8).add_box([3.5, -9.5, -3.5], [1.0, 2.0, 7.0]), Z);
    let mut tips = c().tex_offs(40, 0);
    let at = [[-5.0, -3.0], [-5.0, 1.0], [4.0, -3.0], [4.0, 1.0], [-2.5, -5.0], [1.5, -5.0], [-2.5, 4.0], [1.5, 4.0]];
    for s in at {
        tips = tips.add_box([s[0], -10.5, s[1]], [1.0, 1.0, 1.0]);
    }
    m.add_or_replace_child(r, "tips", tips, Z);
    (64, 32)
}

// A wide brim, a band, two cone steps and a crooked tip.
fn witch_hat(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "brim", c().tex_offs(0, 0).add_box([-6.0, -9.5, -6.0], [12.0, 1.0, 12.0]), Z);
    m.add_or_replace_child(r, "low", c().tex_offs(0, 13).add_box([-3.5, -13.5, -3.5], [7.0, 4.0, 7.0]), Z);
    m.add_or_replace_child(r, "band", c().tex_offs(0, 36).add_box([-4.0, -10.5, -4.0], [8.0, 1.0, 8.0]), Z);
    m.add_or_replace_child(r, "mid", c().tex_offs(0, 25).add_box([-2.5, -17.5, -2.5], [5.0, 4.0, 5.0]), Z);
    m.add_or_replace_child(
        r,
        "tip",
        c().tex_offs(32, 13).add_box([-1.5, -4.0, -1.5], [3.0, 4.0, 3.0]).tex_offs(32, 25).add_box([-0.5, -7.0, 0.0], [1.0, 3.0, 1.0]),
        pr(0.0, -17.5, 0.0, -0.6, 0.0, 0.0),
    );
    (64, 64)
}

// Two tall ears leaning a little outwards.
fn bunny_ears(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "left", c().tex_offs(0, 0).add_box([-1.0, -6.0, -0.5], [2.0, 6.0, 1.0]), pr(-2.0, -8.0, 0.0, 0.0, 0.0, -0.14));
    m.add_or_replace_child(r, "right", c().tex_offs(10, 0).add_box([-1.0, -6.0, -0.5], [2.0, 6.0, 1.0]), pr(2.0, -8.0, 0.0, 0.0, 0.0, 0.14));
    (64, 32)
}

// Two small red horns.
fn devil_horns(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(
        r,
        "left",
        c().tex_offs(0, 0).add_box([-4.0, -10.0, -2.0], [2.0, 2.0, 2.0]).tex_offs(10, 0).add_box([-4.5, -12.0, -1.5], [1.0, 2.0, 1.0]),
        Z,
    );
    m.add_or_replace_child(
        r,
        "right",
        c().tex_offs(16, 0).add_box([2.0, -10.0, -2.0], [2.0, 2.0, 2.0]).tex_offs(26, 0).add_box([3.5, -12.0, -1.5], [1.0, 2.0, 1.0]),
        Z,
    );
    (64, 32)
}

// A band over the head, bars down the sides, cups on the ears.
fn headphones(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "cupL", c().tex_offs(0, 0).add_box([-6.0, -6.0, -2.0], [2.0, 4.0, 4.0]), Z);
    m.add_or_replace_child(r, "cupR", c().tex_offs(14, 0).add_box([4.0, -6.0, -2.0], [2.0, 4.0, 4.0]), Z);
    m.add_or_replace_child(r, "barL", c().tex_offs(0, 10).add_box([-5.5, -9.5, -0.5], [1.0, 4.0, 1.0]), Z);
    m.add_or_replace_child(r, "barR", c().tex_offs(6, 10).add_box([4.5, -9.5, -0.5], [1.0, 4.0, 1.0]), Z);
    m.add_or_replace_child(r, "band", c().tex_offs(0, 16).add_box([-5.0, -9.5, -0.5], [10.0, 1.0, 1.0]), Z);
    (64, 32)
}

// A carved pumpkin a little bigger than the head, with a stem and a leaf.
fn pumpkin_head(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "shell", c().tex_offs(0, 0).add_box([-5.0, -9.5, -5.0], [10.0, 10.0, 10.0]), Z);
    m.add_or_replace_child(r, "stem", c().tex_offs(40, 0).add_box([-1.0, -11.5, -1.0], [2.0, 2.0, 2.0]), Z);
    m.add_or_replace_child(r, "leaf", c().tex_offs(40, 6).add_box([0.0, -10.5, -1.0], [3.0, 1.0, 2.0]), Z);
    (64, 64)
}

// Glow: the carved face, a flat sheet just in front of the shell.
fn pumpkin_face(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "face", c().tex_offs(0, 24).add_box([-5.0, -9.5, -5.1], [10.0, 10.0, 0.0]), Z);
    (64, 64)
}

// A fur band and a red cone that flops over, with a pom-pom on the end.
fn santa_hat(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "band", c().tex_offs(0, 0).add_box([-5.0, -9.5, -5.0], [10.0, 2.0, 10.0]), Z);
    m.add_or_replace_child(r, "low", c().tex_offs(0, 12).add_box([-4.0, -12.5, -4.0], [8.0, 3.0, 8.0]), Z);
    let mid = m.add_or_replace_child(r, "mid", c().tex_offs(0, 24).add_box([-3.0, -3.0, -3.0], [6.0, 3.0, 6.0]), pr(0.5, -12.5, 0.0, 0.0, 0.0, 0.3));
    let tip = m.add_or_replace_child(mid, "tip", c().tex_offs(32, 12).add_box([-2.0, -3.0, -2.0], [4.0, 3.0, 4.0]), pr(0.5, -2.5, 0.0, 0.0, 0.0, 0.55));
    let end = m.add_or_replace_child(tip, "end", c().tex_offs(32, 24).add_box([-1.0, -3.0, -1.0], [2.0, 3.0, 2.0]), pr(0.5, -2.5, 0.0, 0.0, 0.0, 0.7));
    m.add_or_replace_child(end, "pom", c().tex_offs(44, 24).add_box([-1.5, -4.5, -1.5], [3.0, 3.0, 3.0]), Z);
    (64, 64)
}

// A cloth band round the forehead, a knot at the back and two tails.
fn bandana_parts(m: &mut MeshDefinition, r: PartId) {
    m.add_or_replace_child(r, "band", c().tex_offs(0, 0).add_box_with([-4.0, -7.5, -4.0], [8.0, 2.0, 8.0], CubeDeformation::new(0.7)), Z);
    m.add_or_replace_child(r, "knot", c().tex_offs(0, 12).add_box([-1.0, -7.5, 4.7], [2.0, 2.0, 1.0]), Z);
    m.add_or_replace_child(r, "tailL", c().tex_offs(8, 12).add_box([-0.5, 0.0, -0.5], [1.0, 3.0, 1.0]), pr(-0.6, -6.0, 5.2, 0.4, 0.0, 0.35));
    m.add_or_replace_child(r, "tailR", c().tex_offs(14, 12).add_box([-0.5, 0.0, -0.5], [1.0, 3.0, 1.0]), pr(0.6, -6.0, 5.2, 0.4, 0.0, -0.35));
}

fn bandana(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    bandana_parts(m, r);
    (64, 32)
}

// Two branching antlers leaning outwards.
fn antlers(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    for side in [-1, 1] {
        let s = side as f32;
        let left = side < 0;
        let a = c()
            .tex_offs(16, 4)
            .add_box([-1.0, -1.0, -1.0], [2.0, 1.0, 2.0])
            .tex_offs(0, 0)
            .add_box([-0.5, -6.0, -0.5], [1.0, 6.0, 1.0])
            .tex_offs(8, 0)
            .add_box([if left { -3.5 } else { 0.5 }, -4.0, -0.5], [3.0, 1.0, 1.0])
            .tex_offs(8, 4)
            .add_box([if left { -3.5 } else { 2.5 }, -6.0, -0.5], [1.0, 2.0, 1.0])
            .tex_offs(8, 4)
            .add_box([if left { 0.5 } else { -1.5 }, -7.5, -0.5], [1.0, 2.0, 1.0])
            .tex_offs(16, 0)
            .add_box([-0.5, -2.5, -2.5], [1.0, 1.0, 2.0]);
        m.add_or_replace_child(r, if left { "left" } else { "right" }, a, pr(s * 2.5, -8.0, 0.0, 0.0, 0.0, s * 0.3));
    }
    (64, 32)
}

// The crown's eight points (shared by the floating crown and the halo crown).
fn crown_points() -> CubeListBuilder {
    let mut points = c().tex_offs(32, 0);
    for p in [[-3.5, -3.5], [2.5, -3.5], [-3.5, 2.5], [2.5, 2.5]] {
        points = points.add_box([p[0], -3.0, p[1]], [1.0, 1.0, 1.0]);
    }
    points = points.tex_offs(36, 0);
    for p in [[-0.5, -3.5], [-0.5, 2.5], [-3.5, -0.5], [2.5, -0.5]] {
        points = points.add_box([p[0], -4.0, p[1]], [1.0, 2.0, 1.0]);
    }
    points
}

// A small gold crown built round its own centre (it floats, see animate_hat).
fn floating_crown(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "front", c().tex_offs(0, 0).add_box([-3.5, -2.0, -3.5], [7.0, 2.0, 1.0]), Z);
    m.add_or_replace_child(r, "back", c().tex_offs(0, 4).add_box([-3.5, -2.0, 2.5], [7.0, 2.0, 1.0]), Z);
    m.add_or_replace_child(r, "left", c().tex_offs(0, 8).add_box([-3.5, -2.0, -2.5], [1.0, 2.0, 5.0]), Z);
    m.add_or_replace_child(r, "right", c().tex_offs(16, 8).add_box([2.5, -2.0, -2.5], [1.0, 2.0, 5.0]), Z);
    m.add_or_replace_child(r, "points", crown_points(), Z);
    m.add_or_replace_child(r, "velvet", c().tex_offs(0, 16).add_box([-2.5, -3.0, -2.5], [5.0, 2.0, 5.0]), Z);
    m.add_or_replace_child(
        r,
        "cross",
        c().tex_offs(24, 16).add_box([-0.5, -5.0, -0.5], [1.0, 2.0, 1.0]).tex_offs(28, 16).add_box([-1.0, -4.5, -0.5], [2.0, 1.0, 1.0]),
        Z,
    );
    (64, 32)
}

// Glow: the floating crown's gems and the orb on the cross.
fn floating_crown_gems(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "front", c().tex_offs(40, 0).add_box([-0.5, -1.5, -3.8], [1.0, 1.0, 1.0]), Z);
    m.add_or_replace_child(r, "back", c().tex_offs(44, 0).add_box([-0.5, -1.5, 2.8], [1.0, 1.0, 1.0]), Z);
    m.add_or_replace_child(r, "left", c().tex_offs(48, 0).add_box([-3.8, -1.5, -0.5], [1.0, 1.0, 1.0]), Z);
    m.add_or_replace_child(r, "right", c().tex_offs(52, 0).add_box([2.8, -1.5, -0.5], [1.0, 1.0, 1.0]), Z);
    m.add_or_replace_child(r, "orb", c().tex_offs(40, 4).add_box([-0.5, -6.0, -0.5], [1.0, 1.0, 1.0]), Z);
    (64, 32)
}

// The bandana with a skull patch on the front (9x9 texels squeezed onto 3x3).
fn first_blood(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    bandana_parts(m, r);
    m.add_or_replace_child(r, "skull", c().tex_offs(24, 12).add_box_with([-4.5, -11.0, -4.8], [9.0, 9.0, 0.0], grow(-3.0, -3.0, 0.0)), Z);
    (64, 32)
}

// A black hood a size bigger than the head, a lining and a peak at the back.
fn golden_reaper(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "shell", c().tex_offs(0, 0).add_box([-5.0, -9.5, -5.0], [10.0, 10.0, 10.0]), Z);
    m.add_or_replace_child(r, "lining", c().tex_offs(0, 20).add_box_with([-4.0, -8.0, -4.0], [8.0, 8.0, 8.0], grow(0.6, 0.6, 0.6)), Z);
    m.add_or_replace_child(
        r,
        "peak",
        c().tex_offs(40, 0).add_box([-2.5, -10.5, -0.5], [5.0, 1.0, 5.0]).tex_offs(40, 8).add_box([-1.5, -11.5, 1.5], [3.0, 1.0, 3.0]),
        Z,
    );
    (64, 64)
}

// The Combo King crown sits on the head, tipped a little to one side.
fn combo_king_pose() -> PartPose {
    pr(0.0, -8.5, 0.0, 0.0, 0.0, 0.12)
}

// The band shared by the Combo King and the Halo Crown.
fn small_band() -> CubeListBuilder {
    c().tex_offs(0, 0)
        .add_box([-3.5, -2.0, -3.5], [7.0, 2.0, 1.0])
        .tex_offs(0, 4)
        .add_box([-3.5, -2.0, 2.5], [7.0, 2.0, 1.0])
        .tex_offs(0, 8)
        .add_box([-3.5, -2.0, -2.5], [1.0, 2.0, 5.0])
        .tex_offs(16, 8)
        .add_box([2.5, -2.0, -2.5], [1.0, 2.0, 5.0])
}

// A small, mean crown: a narrow band and eight tapering spikes leaning out.
fn combo_king(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    let crown = m.add_or_replace_child(r, "crown", small_band(), combo_king_pose());
    // x, z, base height (the tip is 2 more)
    let spikes: [[i32; 3]; 8] = [[-3, -3, 1], [3, -3, 1], [-3, 3, 1], [3, 3, 1], [0, -3, 3], [0, 3, 2], [-3, 0, 2], [3, 0, 2]];
    for (i, [x, z, h]) in spikes.into_iter().enumerate() {
        let base_u = if h == 1 { 32 } else if h == 2 { 36 } else { 40 };
        let hf = h as f32;
        m.add_or_replace_child(
            crown,
            &format!("spike{i}"),
            c().tex_offs(base_u, 0)
                .add_box([-0.5, -hf, -0.5], [1.0, hf, 1.0])
                .tex_offs(44, 0)
                .add_box_with([-0.5, -hf - 2.0, -0.5], [1.0, 2.0, 1.0], grow(-0.25, 0.0, -0.25)),
            pr(x as f32, -2.0, z as f32, -0.28 * z.signum() as f32, 0.0, 0.28 * x.signum() as f32),
        );
    }
    (64, 32)
}

// Glow: the Combo King's red gems.
fn combo_king_gems(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(
        r,
        "gems",
        c().tex_offs(48, 0)
            .add_box([-0.5, -1.5, -3.8], [1.0, 1.0, 1.0])
            .add_box([-0.5, -1.5, 2.8], [1.0, 1.0, 1.0])
            .add_box([-3.8, -1.5, -0.5], [1.0, 1.0, 1.0])
            .add_box([2.8, -1.5, -0.5], [1.0, 1.0, 1.0]),
        combo_king_pose(),
    );
    (64, 32)
}

// A painter's beret slouched to one side.
fn creator_beret(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(
        r,
        "beret",
        c().tex_offs(0, 13)
            .add_box([-4.5, -1.0, -4.5], [9.0, 1.0, 9.0])
            .tex_offs(0, 0)
            .add_box([-5.5, -3.0, -5.5], [11.0, 2.0, 11.0])
            .tex_offs(0, 23)
            .add_box([-4.5, -4.0, -4.5], [9.0, 1.0, 9.0])
            .tex_offs(40, 13)
            .add_box_with([-0.5, -6.0, -0.5], [1.0, 2.0, 1.0], grow(-0.2, 0.0, -0.2)),
        pr(0.8, -8.6, 0.0, 0.0, 0.0, 0.2),
    );
    (64, 64)
}

// A knitted scarf in body space: a ring round the neck, a knot and two ends.
fn winter_scarf(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "ring", c().tex_offs(0, 0).add_box_with([-5.0, -1.5, -3.0], [10.0, 3.0, 6.0], grow(0.3, 0.3, 0.3)), Z);
    m.add_or_replace_child(r, "knot", c().tex_offs(0, 10).add_box([1.0, -0.5, -4.3], [3.0, 3.0, 1.0]), Z);
    m.add_or_replace_child(r, "long", c().tex_offs(10, 10).add_box([-1.5, 0.0, -0.5], [3.0, 7.0, 1.0]), pr(2.5, 2.2, -3.9, 0.0, 0.0, 0.06));
    m.add_or_replace_child(r, "short", c().tex_offs(20, 10).add_box([-1.5, 0.0, -0.5], [3.0, 5.0, 1.0]), pr(3.4, 2.0, -3.5, 0.0, 0.0, -0.22));
    (64, 32)
}

// A striped party cone with a pom-pom, and an elastic under the chin.
fn birthday_hat(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(
        r,
        "cone",
        c().tex_offs(0, 0)
            .add_box([-3.0, -2.0, -3.0], [6.0, 2.0, 6.0])
            .tex_offs(0, 8)
            .add_box([-2.5, -4.0, -2.5], [5.0, 2.0, 5.0])
            .tex_offs(0, 16)
            .add_box([-2.0, -6.0, -2.0], [4.0, 2.0, 4.0])
            .tex_offs(0, 24)
            .add_box([-1.5, -8.0, -1.5], [3.0, 2.0, 3.0])
            .tex_offs(24, 16)
            .add_box([-1.0, -10.0, -1.0], [2.0, 2.0, 2.0])
            .tex_offs(24, 24)
            .add_box_with([-1.5, -12.5, -1.5], [3.0, 3.0, 3.0], grow(-0.2, -0.2, -0.2)),
        pr(1.2, -8.6, 0.0, 0.0, 0.0, 0.18),
    );
    m.add_or_replace_child(
        r,
        "elastic",
        c().tex_offs(40, 0)
            .add_box_with([-4.6, -8.5, -1.5], [0.0, 9.0, 1.0], grow(0.0, 0.0, -0.35))
            .add_box_with([4.6, -8.5, -1.5], [0.0, 9.0, 1.0], grow(0.0, 0.0, -0.35))
            .tex_offs(44, 0)
            .add_box_with([-4.6, 0.5, -1.5], [9.2, 0.0, 1.0], grow(0.0, 0.0, -0.35)),
        Z,
    );
    (64, 32)
}

// A chef's toque: a band, a puffy crown and two smaller layers on top.
fn chef_hat(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "band", c().tex_offs(0, 0).add_box([-5.0, -9.5, -5.0], [10.0, 3.0, 10.0]), Z);
    m.add_or_replace_child(r, "puff", c().tex_offs(0, 13).add_box([-5.5, -15.5, -5.5], [11.0, 6.0, 11.0]), Z);
    m.add_or_replace_child(r, "cap", c().tex_offs(0, 30).add_box([-5.0, -16.5, -5.0], [10.0, 1.0, 10.0]), Z);
    m.add_or_replace_child(r, "top", c().tex_offs(0, 41).add_box([-4.0, -17.5, -4.0], [8.0, 1.0, 8.0]), Z);
    (64, 64)
}

// A black tricorn: a crown, three turned-up brim walls and a skull badge.
fn pirate_hat(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(
        r,
        "crown",
        c().tex_offs(0, 0).add_box([-4.0, -12.5, -1.0], [8.0, 4.0, 5.0]).tex_offs(28, 0).add_box([-2.5, -12.5, -4.0], [5.0, 4.0, 3.0]),
        Z,
    );
    m.add_or_replace_child(
        r,
        "back",
        c().tex_offs(0, 10).add_box([-6.5, -3.0, -0.5], [13.0, 3.0, 1.0]).tex_offs(0, 15).add_box([-5.0, -1.0, -3.5], [10.0, 1.0, 3.0]),
        pr(0.0, -8.5, 5.5, -0.3, 0.0, 0.0),
    );
    m.add_or_replace_child(
        r,
        "right",
        c().tex_offs(0, 20).add_box([-7.0, -3.0, -0.5], [14.0, 3.0, 1.0]).tex_offs(0, 25).add_box([-5.0, -1.0, -3.5], [10.0, 1.0, 3.0]),
        pr(4.0, -8.5, -1.25, -0.3, 1.951, 0.0),
    );
    m.add_or_replace_child(
        r,
        "left",
        c().tex_offs(32, 20).add_box([-7.0, -3.0, -0.5], [14.0, 3.0, 1.0]).tex_offs(32, 25).add_box([-5.0, -1.0, -3.5], [10.0, 1.0, 3.0]),
        pr(-4.0, -8.5, -1.25, -0.3, -1.951, 0.0),
    );
    m.add_or_replace_child(r, "skull", c().tex_offs(0, 32).add_box_with([-3.5, -14.0, -8.3], [7.0, 7.0, 0.0], grow(-1.5, -1.5, 0.0)), Z);
    (64, 64)
}

// A small gold crown with a velvet cap (the halo is halo_ring). Also vip_hat.
fn halo_crown(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    let crown = m.add_or_replace_child(
        r,
        "crown",
        small_band().tex_offs(0, 16).add_box([-2.5, -3.0, -2.5], [5.0, 2.0, 5.0]),
        PartPose::offset(0.0, -8.5, 0.0),
    );
    m.add_or_replace_child(crown, "points", crown_points(), Z);
    (64, 32)
}

// Glow: the halo, an octagon of small boxes (it bobs, see animate_hat).
fn halo_ring(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    let mut ring = c()
        .tex_offs(32, 8)
        .add_box([-2.0, -0.5, -4.0], [4.0, 1.0, 1.0])
        .tex_offs(32, 12)
        .add_box([-2.0, -0.5, 3.0], [4.0, 1.0, 1.0])
        .tex_offs(44, 8)
        .add_box([-4.0, -0.5, -2.0], [1.0, 1.0, 4.0])
        .tex_offs(44, 14)
        .add_box([3.0, -0.5, -2.0], [1.0, 1.0, 4.0])
        .tex_offs(56, 8);
    for p in [[-3.0, -3.0], [2.0, -3.0], [-3.0, 2.0], [2.0, 2.0]] {
        ring = ring.add_box([p[0], -0.5, p[1]], [1.0, 1.0, 1.0]);
    }
    m.add_or_replace_child(r, "ring", ring, PartPose::offset(0.0, -14.5, 0.0));
    (64, 32)
}

// A sweatband with a knot at the back and two fluttering tails.
fn focus_headband(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "band", c().tex_offs(0, 0).add_box_with([-4.0, -8.0, -4.0], [8.0, 3.0, 8.0], grow(0.6, 0.15, 0.6)), Z);
    m.add_or_replace_child(r, "knot", c().tex_offs(0, 12).add_box([-1.5, -7.5, 4.6], [3.0, 2.0, 1.0]), Z);
    m.add_or_replace_child(
        r,
        "tailL",
        c().tex_offs(10, 12).add_box_with([-1.0, 0.0, -0.5], [2.0, 5.0, 1.0], grow(0.0, 0.0, -0.3)),
        pr(-0.8, -6.5, 5.4, 0.5, 0.0, 0.25),
    );
    m.add_or_replace_child(
        r,
        "tailR",
        c().tex_offs(18, 12).add_box_with([-1.0, 0.0, -0.5], [2.0, 5.0, 1.0], grow(0.0, 0.0, -0.3)),
        pr(0.8, -6.5, 5.4, 0.35, 0.0, -0.3),
    );
    (64, 32)
}

// The Party Hat Deluxe tips the other way from the Birthday Hat.
fn party_hat_pose() -> PartPose {
    pr(-1.2, -8.6, 0.0, 0.0, 0.0, -0.15)
}

// The party cone's steps; v is the first row (the glitter is 32 rows lower).
fn party_cone(v: i32, g: f32) -> CubeListBuilder {
    let d = grow(g, g, g);
    c().tex_offs(0, v)
        .add_box_with([-3.0, -2.0, -3.0], [6.0, 2.0, 6.0], d)
        .tex_offs(0, v + 8)
        .add_box_with([-2.5, -4.0, -2.5], [5.0, 2.0, 5.0], d)
        .tex_offs(0, v + 16)
        .add_box_with([-2.0, -6.0, -2.0], [4.0, 2.0, 4.0], d)
        .tex_offs(0, v + 24)
        .add_box_with([-1.5, -8.0, -1.5], [3.0, 2.0, 3.0], d)
        .tex_offs(24, v + 16)
        .add_box_with([-1.0, -10.0, -1.0], [2.0, 2.0, 2.0], d)
        .tex_offs(24, v + 24)
        .add_box_with([-0.5, -12.0, -0.5], [1.0, 2.0, 1.0], d)
}

// A tall glitter cone with a gold knob and a tassel.
fn party_hat_deluxe(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    let cone = m.add_or_replace_child(r, "cone", party_cone(0, 0.0), party_hat_pose());
    let knob = m.add_or_replace_child(cone, "knob", c().tex_offs(32, 0).add_box([-1.0, -1.5, -1.0], [2.0, 2.0, 2.0]), PartPose::offset(0.0, -12.0, 0.0));
    m.add_or_replace_child(
        knob,
        "tassel",
        c().tex_offs(32, 8).add_box_with([-1.0, 0.0, -1.0], [2.0, 4.0, 2.0], grow(-0.15, 0.0, -0.15)),
        pr(-0.3, -0.5, 0.0, 0.0, 0.0, 0.6),
    );
    (64, 64)
}

// Glow: the cone's boxes a hair bigger, see-through except the sparkles.
fn party_hat_glitter(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "cone", party_cone(32, 0.02), party_hat_pose());
    (64, 64)
}

// One jester point: three segments curling out and down, and a bell.
fn jester_point(m: &mut MeshDefinition, r: PartId, name: &str, v: i32, side: f32) {
    let a = m.add_or_replace_child(r, name, c().tex_offs(0, v).add_box([-1.5, -4.0, -1.5], [3.0, 4.0, 3.0]), pr(side * 2.2, -10.5, 0.0, 0.0, 0.0, side * 0.6));
    let b = m.add_or_replace_child(a, "mid", c().tex_offs(16, v).add_box([-1.0, -4.0, -1.0], [2.0, 4.0, 2.0]), pr(0.0, -3.7, 0.0, 0.0, 0.0, side * 0.6));
    let t = m.add_or_replace_child(b, "tip", c().tex_offs(28, v).add_box([-0.5, -3.0, -0.5], [1.0, 3.0, 1.0]), pr(0.0, -3.7, 0.0, 0.0, 0.0, side * 0.75));
    m.add_or_replace_child(t, "bell", c().tex_offs(36, v).add_box([-1.0, -0.5, -1.0], [2.0, 2.0, 2.0]), pr(0.0, -3.2, 0.0, 0.0, 0.0, -side * 1.95));
}

// A two-pointed jester cap with bells.
fn jester_hat(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "cap", c().tex_offs(0, 0).add_box_with([-4.5, -10.5, -4.5], [9.0, 3.0, 9.0], grow(0.3, 0.3, 0.3)), Z);
    jester_point(m, r, "right", 16, 1.0);
    jester_point(m, r, "left", 28, -1.0);
    (64, 64)
}

// Gauze round the forehead, a strip over the top, a loose end and a plaster.
fn bandage(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "wrap", c().tex_offs(0, 0).add_box_with([-4.0, -8.0, -4.0], [8.0, 3.0, 8.0], grow(0.6, 0.2, 0.6)), Z);
    m.add_or_replace_child(
        r,
        "strip",
        c().tex_offs(32, 0).add_box_with([-4.0, -8.0, -1.0], [8.0, 5.0, 2.0], grow(1.2, 0.7, 0.0)),
        pr(0.0, 0.0, 0.0, 0.0, 0.3, 0.0),
    );
    m.add_or_replace_child(
        r,
        "end",
        c().tex_offs(0, 12).add_box_with([-1.0, 0.0, -0.5], [2.0, 4.0, 1.0], grow(0.0, 0.0, -0.3)),
        pr(1.5, -6.0, 4.7, 0.3, 0.0, -0.2),
    );
    m.add_or_replace_child(r, "plasterA", c().tex_offs(0, 20).add_box([-1.5, -0.5, 0.0], [3.0, 1.0, 0.0]), pr(2.3, -2.8, -4.6, 0.0, 0.0, 0.7));
    m.add_or_replace_child(r, "plasterB", c().tex_offs(0, 22).add_box([-1.5, -0.5, 0.0], [3.0, 1.0, 0.0]), pr(2.3, -2.8, -4.65, 0.0, 0.0, -0.7));
    (64, 32)
}

// A wide straw hat with straw poking out and a crow on the brim.
fn scarecrow_hat(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "brim", c().tex_offs(0, 0).add_box([-7.0, -9.0, -7.0], [14.0, 1.0, 14.0]), Z);
    m.add_or_replace_child(
        r,
        "crown",
        c().tex_offs(0, 16).add_box([-4.5, -13.0, -4.5], [9.0, 4.0, 9.0]).tex_offs(0, 30).add_box([-4.0, -14.0, -4.0], [8.0, 1.0, 8.0]),
        Z,
    );
    m.add_or_replace_child(
        r,
        "straw",
        c().tex_offs(0, 42)
            .add_box([-4.75, -9.0, -4.0], [0.0, 3.0, 8.0])
            .tex_offs(16, 42)
            .add_box([4.75, -9.0, -4.0], [0.0, 3.0, 8.0])
            .tex_offs(32, 42)
            .add_box([-4.0, -9.0, 4.75], [8.0, 3.0, 0.0]),
        Z,
    );
    let crow = m.add_or_replace_child(r, "crow", c().tex_offs(36, 30).add_box([-1.0, -2.0, -1.5], [2.0, 2.0, 3.0]), pr(-5.3, -9.0, -3.2, 0.0, 0.6, 0.0));
    m.add_or_replace_child(crow, "tail", c().tex_offs(48, 36).add_box([-0.5, -1.6, 1.3], [1.0, 1.0, 2.0]), pr(0.0, 0.0, 0.0, -0.35, 0.0, 0.0));
    m.add_or_replace_child(
        crow,
        "head",
        c().tex_offs(48, 30).add_box([-1.0, -2.0, -1.0], [2.0, 2.0, 2.0]).tex_offs(56, 30).add_box([-0.5, -1.2, -2.0], [1.0, 1.0, 1.0]),
        PartPose::offset(0.0, -1.6, -1.3),
    );
    (64, 64)
}

// A tall felt elf hat that curls over at the end, with a bell and holly.
fn elf_hat(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "cuff", c().tex_offs(0, 0).add_box([-5.0, -10.0, -5.0], [10.0, 2.0, 10.0]), Z);
    m.add_or_replace_child(r, "low", c().tex_offs(0, 12).add_box([-4.0, -13.0, -4.0], [8.0, 3.0, 8.0]), Z);
    let mid = m.add_or_replace_child(r, "mid", c().tex_offs(0, 24).add_box([-3.0, -3.0, -3.0], [6.0, 3.0, 6.0]), pr(0.0, -12.5, 0.3, -0.12, 0.0, 0.0));
    let upper = m.add_or_replace_child(mid, "upper", c().tex_offs(32, 12).add_box([-2.0, -3.0, -2.0], [4.0, 3.0, 4.0]), pr(0.0, -2.5, 0.2, -0.2, 0.0, 0.0));
    let tip = m.add_or_replace_child(upper, "tip", c().tex_offs(32, 24).add_box([-1.0, -3.0, -1.0], [2.0, 3.0, 2.0]), pr(0.0, -2.5, 0.1, -0.35, 0.0, 0.0));
    let end = m.add_or_replace_child(tip, "end", c().tex_offs(44, 24).add_box([-0.5, -2.0, -0.5], [1.0, 2.0, 1.0]), pr(0.0, -2.6, 0.0, -1.3, 0.0, 0.0));
    m.add_or_replace_child(end, "bell", c().tex_offs(48, 24).add_box([-1.0, 0.0, -1.0], [2.0, 2.0, 2.0]), PartPose::offset(0.0, -1.8, 0.0));
    let holly = m.add_or_replace_child(r, "holly", c().tex_offs(0, 36).add_box([-3.0, -1.5, 0.0], [6.0, 3.0, 0.0]), pr(-2.2, -9.4, -5.15, 0.0, 0.0, -0.15));
    m.add_or_replace_child(
        holly,
        "berries",
        c().tex_offs(16, 36)
            .add_box([-1.0, -1.0, -0.8], [1.0, 1.0, 1.0])
            .add_box([0.0, -0.4, -0.9], [1.0, 1.0, 1.0])
            .add_box([-0.6, 0.2, -0.7], [1.0, 1.0, 1.0]),
        Z,
    );
    (64, 64)
}

// A floppy sun hat with drooping flaps, a bow, a flower and sunglasses.
fn sun_hat(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "brim", c().tex_offs(0, 0).add_box([-8.0, -9.0, -8.0], [16.0, 1.0, 16.0]), Z);
    m.add_or_replace_child(r, "front_flap", c().tex_offs(0, 38).add_box([-8.0, 0.0, -2.0], [16.0, 1.0, 2.0]), pr(0.0, -9.0, -8.0, 0.35, 0.0, 0.0));
    m.add_or_replace_child(r, "back_flap", c().tex_offs(0, 41).add_box([-8.0, 0.0, 0.0], [16.0, 1.0, 2.0]), pr(0.0, -9.0, 8.0, -0.35, 0.0, 0.0));
    m.add_or_replace_child(r, "right_flap", c().tex_offs(36, 29).add_box([-2.0, 0.0, -6.0], [2.0, 1.0, 12.0]), pr(-8.0, -9.0, 0.0, 0.0, 0.0, -0.35));
    m.add_or_replace_child(r, "left_flap", c().tex_offs(36, 42).add_box([0.0, 0.0, -6.0], [2.0, 1.0, 12.0]), pr(8.0, -9.0, 0.0, 0.0, 0.0, 0.35));
    m.add_or_replace_child(
        r,
        "crown",
        c().tex_offs(0, 17).add_box([-4.5, -12.0, -4.5], [9.0, 3.0, 9.0]).tex_offs(0, 29).add_box([-4.0, -13.0, -4.0], [8.0, 1.0, 8.0]),
        Z,
    );
    let bow = m.add_or_replace_child(
        r,
        "bow",
        c().tex_offs(16, 44)
            .add_box([-3.0, -1.0, -0.5], [2.0, 2.0, 1.0])
            .tex_offs(24, 44)
            .add_box([1.0, -1.0, -0.5], [2.0, 2.0, 1.0])
            .tex_offs(32, 44)
            .add_box([-0.5, -0.5, -0.6], [1.0, 1.0, 1.0]),
        PartPose::offset(0.0, -10.0, 5.0),
    );
    m.add_or_replace_child(bow, "right_tail", c().tex_offs(0, 44).add_box([-1.0, 0.0, 0.0], [2.0, 5.0, 0.0]), pr(-0.8, 0.3, 0.1, 0.25, 0.0, 0.15));
    m.add_or_replace_child(bow, "left_tail", c().tex_offs(8, 44).add_box([-1.0, 0.0, 0.0], [2.0, 5.0, 0.0]), pr(0.8, 0.3, 0.1, 0.25, 0.0, -0.15));
    m.add_or_replace_child(
        r,
        "flower",
        c().tex_offs(0, 50).add_box([-2.5, -2.5, 0.0], [5.0, 5.0, 0.0]).tex_offs(12, 50).add_box([-0.5, -0.5, -0.6], [1.0, 1.0, 1.0]),
        pr(-4.65, -10.5, -1.5, 0.0, std::f32::consts::FRAC_PI_2, 0.0),
    );
    m.add_or_replace_child(r, "glasses", c().tex_offs(0, 56).add_box([-4.0, -1.5, 0.0], [8.0, 3.0, 0.0]), pr(0.0, -10.6, -4.65, -0.12, 0.0, 0.0));
    (64, 64)
}

// A circlet of hearts: a band, a big heart at the front and six little ones. Also supporter_hat.
fn heart_crown(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "front", c().tex_offs(0, 0).add_box([-4.5, -10.5, -4.5], [9.0, 2.0, 1.0]), Z);
    m.add_or_replace_child(r, "back", c().tex_offs(0, 4).add_box([-4.5, -10.5, 3.5], [9.0, 2.0, 1.0]), Z);
    m.add_or_replace_child(r, "right", c().tex_offs(0, 8).add_box([-4.5, -10.5, -3.5], [1.0, 2.0, 7.0]), Z);
    m.add_or_replace_child(r, "left", c().tex_offs(16, 8).add_box([3.5, -10.5, -3.5], [1.0, 2.0, 7.0]), Z);
    // the big heart stands on its point on the front of the band
    m.add_or_replace_child(
        r,
        "big_heart",
        c().tex_offs(32, 0)
            .add_box([-2.5, -4.0, -0.5], [2.0, 1.0, 1.0])
            .tex_offs(38, 0)
            .add_box([0.5, -4.0, -0.5], [2.0, 1.0, 1.0])
            .tex_offs(44, 0)
            .add_box([-2.5, -3.0, -0.5], [5.0, 1.0, 1.0])
            .tex_offs(32, 4)
            .add_box([-1.5, -2.0, -0.5], [3.0, 1.0, 1.0])
            .tex_offs(40, 4)
            .add_box([-0.5, -1.0, -0.5], [1.0, 1.0, 1.0]),
        PartPose::offset(0.0, -10.5, -4.0),
    );
    // little hearts round the band, each turned to face out: x, z, turn
    let at: [[f32; 3]; 6] = [[-4.0, -4.0, 0.785], [4.0, -4.0, -0.785], [-4.3, 0.5, 1.687], [4.3, 0.5, -1.687], [-2.5, 4.2, 2.605], [2.5, 4.2, -2.605]];
    for (k, [x, z, turn]) in at.into_iter().enumerate() {
        m.add_or_replace_child(
            r,
            &format!("heart{k}"),
            c().tex_offs(48, 4)
                .add_box([-1.5, -3.0, -0.5], [1.0, 1.0, 1.0])
                .tex_offs(52, 4)
                .add_box([0.5, -3.0, -0.5], [1.0, 1.0, 1.0])
                .tex_offs(32, 8)
                .add_box([-1.5, -2.0, -0.5], [3.0, 1.0, 1.0])
                .tex_offs(56, 4)
                .add_box([-0.5, -1.0, -0.5], [1.0, 1.0, 1.0]),
            pr(x, -10.5, z, 0.0, turn, 0.0),
        );
    }
    (64, 32)
}

// Glow: the Heart Crown's rubies.
fn heart_crown_gems(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "gem", c().tex_offs(40, 8).add_box([-1.5, -3.2, -0.6], [3.0, 2.0, 0.0]), PartPose::offset(0.0, -10.5, -4.0));
    m.add_or_replace_child(
        r,
        "band",
        c().tex_offs(48, 8)
            .add_box([-3.5, -10.0, -4.6], [1.0, 1.0, 0.0])
            .add_box([2.5, -10.0, -4.6], [1.0, 1.0, 0.0])
            .add_box([-4.6, -10.0, -0.5], [0.0, 1.0, 1.0])
            .add_box([4.6, -10.0, -0.5], [0.0, 1.0, 1.0]),
        Z,
    );
    (64, 32)
}

// A knitted hat: a ribbed cuff, the dome and a pompom.
fn beanie(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "cuff", c().tex_offs(0, 0).add_box([-5.0, -7.0, -5.0], [10.0, 3.0, 10.0]), Z);
    m.add_or_replace_child(r, "dome", c().tex_offs(0, 13).add_box([-4.5, -9.5, -4.5], [9.0, 3.0, 9.0]), Z);
    m.add_or_replace_child(r, "pompom", c().tex_offs(0, 25).add_box([-1.5, -2.0, -1.5], [3.0, 2.0, 3.0]), PartPose::offset(0.0, -9.5, 0.0));
    (64, 64)
}

// A lacquered bowl with flaring guards and a crescent on the front.
fn samurai_helmet(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "bowl", c().tex_offs(0, 0).add_box([-5.0, -10.0, -5.0], [10.0, 5.0, 10.0]), Z);
    m.add_or_replace_child(r, "top", c().tex_offs(0, 15).add_box([-3.5, -11.5, -3.5], [7.0, 2.0, 7.0]), Z);
    m.add_or_replace_child(r, "back_guard", c().tex_offs(0, 24).add_box([-6.0, 0.0, 0.0], [12.0, 4.0, 1.0]), pr(0.0, -5.5, 5.0, 0.35, 0.0, 0.0));
    m.add_or_replace_child(r, "right_guard", c().tex_offs(0, 29).add_box([-1.0, 0.0, -5.0], [1.0, 4.0, 10.0]), pr(-5.0, -5.5, 0.0, 0.0, 0.0, 0.3));
    m.add_or_replace_child(r, "left_guard", c().tex_offs(24, 29).add_box([0.0, 0.0, -5.0], [1.0, 4.0, 10.0]), pr(5.0, -5.5, 0.0, 0.0, 0.0, -0.3));
    m.add_or_replace_child(r, "crest", c().tex_offs(0, 44).add_box([-6.0, -7.0, 0.0], [12.0, 7.0, 0.0]), PartPose::offset(0.0, -8.5, -5.1));
    (64, 64)
}

// A steel helmet with cheek pieces, a nose guard and a plume.
fn war_helm(m: &mut MeshDefinition, r: PartId) -> (u32, u32) {
    m.add_or_replace_child(r, "dome", c().tex_offs(0, 0).add_box([-5.0, -9.5, -5.0], [10.0, 5.0, 10.0]), Z);
    m.add_or_replace_child(r, "right_cheek", c().tex_offs(0, 15).add_box([-5.0, -4.5, -5.0], [2.0, 3.0, 3.0]), Z);
    m.add_or_replace_child(r, "left_cheek", c().tex_offs(12, 15).add_box([3.0, -4.5, -5.0], [2.0, 3.0, 3.0]), Z);
    m.add_or_replace_child(r, "nose", c().tex_offs(24, 15).add_box([-1.0, -4.5, -5.4], [2.0, 3.0, 1.0]), Z);
    m.add_or_replace_child(r, "neck", c().tex_offs(0, 22).add_box([-5.0, -4.5, 2.0], [10.0, 3.0, 3.0]), Z);
    m.add_or_replace_child(r, "plume", c().tex_offs(0, 30).add_box([-0.5, -4.0, -5.0], [1.0, 4.0, 10.0]), PartPose::offset(0.0, -9.0, 0.0));
    (64, 64)
}
