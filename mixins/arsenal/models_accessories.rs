// Arsenal's accessory and pet models, ported from the Java mod's AccessoryLayer:
// the meshes (accessory_layers), where each one is drawn this frame
// (cosmetic_draws) and the part poses set on it before drawing (animate_cosmetic).
// Particle auras are not here.
// 
// Not ported: the see-through render type (slime, allay, bee, golden_bee,
// fox_spirit, ghost_aura, snowfall draw translucent in Java).

#[allow(unused_imports)]
use crate::mod_api::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartId, PartPose};
use std::f64::consts::PI;

/// The accessories that have a model.
pub const ACCESSORY_IDS: &[&str] = &[
    "halo",
    "orbiting_crystals",
    "wings",
    "rune_circle",
    "ender_wings",
    "totem_orbit",
    "cloud_trail",
    "bronze_sword",
    "untouchable",
    "daily_player",
    "friendship",
    "linked_badge",
    "popularity_stars",
    "ember_wings",
    "completionist_halo",
    "ambassador_badge",
    "first_week_badge",
    "lucky_clover",
    "ghost_aura",
    "snowfall",
    "beach_ball",
    "vip_accessory",
    "supporter_accessory",
];

/// The pets.
pub const PET_IDS: &[&str] = &[
    "slime",
    "owl",
    "allay",
    "baby_dragon",
    "bee",
    "night_owl",
    "pumpkin_lantern",
    "baby_wither",
    "phantom",
    "ghast",
    "golden_bee",
    "creeper_buddy",
    "fox_spirit",
    "bat",
    "reindeer",
    "crab",
    "love_bird",
    "axolotl",
    "mini_golem",
    "vip_pet",
];

/// One pose-stack call after the bone (blocks and radians, in order).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Step {
    Translate([f32; 3]),
    RotateX(f32),
    RotateY(f32),
    RotateZ(f32),
    Scale([f32; 3]),
}

/// One model drawn this frame.
#[derive(Clone, Debug)]
pub struct Draw {
    pub layer: String,
    pub texture: String,
    /// "head", "body", "root" (player-model space), or "left_arm" (Daily Player's patch only).
    pub bone: &'static str,
    pub steps: Vec<Step>,
    /// Full-bright (Java light 0xF000F0).
    pub glow: bool,
}

/// What the draw code needs to know about the player this frame.
#[derive(Clone, Copy, Debug, Default)]
pub struct Ctx {
    /// Wall-clock seconds, Java's (ms % 1e6) / 1000.
    pub t: f32,
    pub sneaking: bool,
    pub gliding: bool,
    pub swimming_or_flying: bool,
    pub has_chestplate: bool,
    pub walk: f32,
    pub walk_speed: f32,
    pub cape_lean: f32,
    pub cape_flap: f32,
}

// ---------------------------------------------------------------------------
// Meshes
// ---------------------------------------------------------------------------

fn cubes() -> CubeListBuilder {
    CubeListBuilder::create()
}

/// A mesh with one part named "part" at the origin (Java's part()).
fn single(c: CubeListBuilder, w: u32, h: u32) -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    mesh.add_or_replace_child(root, "part", c, PartPose::ZERO);
    LayerDefinition::create(mesh, w, h)
}

/// One wing part for side s (1 = the +x wing, -1 its mirror image); boxes are {u, v, x, y, z, w, h, d}.
fn wing_part(
    mesh: &mut MeshDefinition,
    parent: PartId,
    name: &str,
    s: i32,
    x: f32,
    y: f32,
    z: f32,
    z_rot: f32,
    boxes: &[[f32; 8]],
) -> PartId {
    let sf = s as f32;
    let mut c = cubes().set_mirror(s < 0);
    for b in boxes {
        let bx = if s > 0 { b[2] } else { -b[2] - b[5] };
        c = c.tex_offs(b[0] as i32, b[1] as i32).add_box([bx, b[3], b[4]], [b[5], b[6], b[7]]);
    }
    mesh.add_or_replace_child(parent, name, c, PartPose::offset_and_rotation(sf * x, y, z, 0.0, 0.0, sf * z_rot))
}

fn feather_wing(s: i32) -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    let arm = wing_part(&mut mesh, root, "arm", s, 0.0, 0.0, 0.0, -0.62, &[
        [44.0, 0.0, -2.0, -0.25, 0.12, 7.25, 8.25, 0.0],
        [33.0, 16.0, -1.75, 0.0, 0.3, 7.75, 4.5, 0.0],
        [16.0, 22.0, -0.75, -0.25, 0.48, 6.5, 3.5, 0.0],
        [46.0, 22.0, 0.25, -0.75, -0.75, 5.5, 1.5, 1.5],
    ]);
    let forearm = wing_part(&mut mesh, arm, "forearm", s, 5.5, 0.0, 0.0, 0.47, &[
        [25.0, 0.0, -2.5, 0.0, 0.0, 9.5, 13.0, 0.0],
        [15.0, 16.0, -1.75, 0.25, 0.0, 9.0, 5.0, 0.0],
        [0.0, 22.0, -1.0, 0.0, 0.0, 8.0, 3.75, 0.0],
        [29.0, 22.0, -0.25, -0.75, -0.75, 7.0, 1.5, 1.5],
    ]);
    let hand = wing_part(&mut mesh, forearm, "hand", s, 6.5, 0.0, 0.0, 0.72, &[
        [0.0, 16.0, -0.5, -1.25, 0.3, 7.5, 5.5, 0.0],
        [0.0, 26.0, -0.5, -0.75, -0.75, 4.75, 1.5, 1.5],
    ]);
    wing_part(&mut mesh, hand, "p0", s, 0.4, 0.15, -0.18, -0.63, &[[10.0, 0.0, -1.5, -0.25, 0.0, 2.5, 14.5, 0.0]]);
    wing_part(&mut mesh, hand, "p1", s, 1.35, 0.15, -0.31, -0.71, &[[0.0, 0.0, -1.5, -0.25, 0.0, 2.5, 15.25, 0.0]]);
    wing_part(&mut mesh, hand, "p2", s, 2.3, 0.15, -0.44, -0.79, &[[5.0, 0.0, -1.5, -0.25, 0.0, 2.5, 15.5, 0.0]]);
    wing_part(&mut mesh, hand, "p3", s, 3.25, 0.15, -0.57, -0.88, &[[15.0, 0.0, -1.5, -0.25, 0.0, 2.5, 15.0, 0.0]]);
    wing_part(&mut mesh, hand, "p4", s, 4.2, 0.15, -0.7, -0.97, &[[20.0, 0.0, -1.5, -0.25, 0.0, 2.5, 13.75, 0.0]]);
    LayerDefinition::create(mesh, 64, 32)
}

fn membrane_wing(s: i32) -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    let arm = wing_part(&mut mesh, root, "arm", s, 0.0, 0.0, 0.0, -0.62, &[
        [17.0, 15.0, -4.0, -0.25, 0.0, 10.25, 8.75, 0.0],
        [0.0, 27.0, 0.25, -0.75, -0.75, 5.5, 1.5, 1.5],
    ]);
    let forearm = wing_part(&mut mesh, arm, "forearm", s, 5.5, 0.0, 0.0, 0.47, &[
        [3.0, 0.0, -2.25, -0.25, 0.0, 10.25, 13.5, 0.0],
        [38.0, 15.0, -0.25, -0.75, -0.75, 7.0, 1.5, 1.5],
    ]);
    let hand = wing_part(&mut mesh, forearm, "hand", s, 6.5, 0.0, 0.0, 0.52, &[
        [14.0, 27.0, -0.5, -0.75, -0.75, 2.25, 1.5, 1.5],
        [22.0, 27.0, -0.25, -2.75, -0.25, 0.5, 2.0, 0.5],
    ]);
    wing_part(&mut mesh, hand, "f0", s, 1.5, 0.0, -0.12, -0.45, &[[37.0, 0.0, -0.375, -0.25, -0.375, 0.75, 12.75, 0.75]]);
    wing_part(&mut mesh, hand, "f1", s, 1.5, 0.0, -0.24, -0.79, &[
        [24.0, 0.0, -6.25, -0.25, 0.0, 6.5, 14.0, 0.0],
        [0.0, 0.0, -0.375, -0.25, -0.375, 0.75, 13.75, 0.75],
    ]);
    wing_part(&mut mesh, hand, "f2", s, 1.5, 0.0, -0.36, -1.17, &[
        [40.0, 0.0, -7.25, -0.25, 0.0, 7.5, 13.0, 0.0],
        [55.0, 0.0, -0.375, -0.25, -0.375, 0.75, 12.25, 0.75],
    ]);
    wing_part(&mut mesh, hand, "f3", s, 1.5, 0.0, -0.48, -1.59, &[
        [0.0, 15.0, -6.75, -0.25, 0.0, 7.0, 11.5, 0.0],
        [14.0, 15.0, -0.375, -0.25, -0.375, 0.75, 9.25, 0.75],
    ]);
    LayerDefinition::create(mesh, 64, 32)
}

fn halo() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    for k in 0..8 {
        mesh.add_or_replace_child(
            root,
            &format!("s{k}"),
            cubes().tex_offs(0, 0).add_box([-2.0, -0.5, -5.5], [4.0, 1.0, 1.0]),
            PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.0, (k as f64 * PI / 4.0) as f32, 0.0),
        );
    }
    LayerDefinition::create(mesh, 32, 32)
}

/// Completionist Halo: Java's 16 rainbow frames as children "frame0".."frame15" (only the
/// current one is shown, see animate_cosmetic) and its four sparks as "spark0".."spark3".
fn completionist_halo() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    for f in 0..16 {
        let frame = mesh.add_or_replace_child(root, &format!("frame{f}"), cubes(), PartPose::ZERO);
        for k in 0..16 {
            let swatch = (k + f) % 16;
            mesh.add_or_replace_child(
                frame,
                &format!("s{k}"),
                cubes().tex_offs((swatch % 8) * 8, (swatch / 8) * 4).add_box([-1.1, -0.5, -5.5], [2.2, 1.0, 1.0]),
                PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.0, (k as f64 * PI / 8.0) as f32, 0.0),
            );
        }
    }
    for k in 0..4 {
        mesh.add_or_replace_child(
            root,
            &format!("spark{k}"),
            cubes().tex_offs(0, 16).add_box([-0.5, -0.5, -0.5], [1.0, 1.0, 1.0]),
            PartPose::ZERO,
        );
    }
    LayerDefinition::create(mesh, 64, 32)
}

fn crystal() -> LayerDefinition {
    single(cubes().tex_offs(0, 0).add_box([-1.0, -1.0, -1.0], [2.0, 2.0, 2.0]), 32, 32)
}

fn runes() -> LayerDefinition {
    single(cubes().tex_offs(0, 0).add_box([-8.0, 0.0, -8.0], [16.0, 0.0, 16.0]), 64, 32)
}

fn totem() -> LayerDefinition {
    single(cubes().tex_offs(0, 0).add_box([-8.0, -8.0, 0.0], [16.0, 16.0, 0.0]), 64, 32)
}

fn cloud() -> LayerDefinition {
    single(
        cubes()
            .tex_offs(0, 0).add_box([-5.0, 0.0, -4.0], [10.0, 2.0, 8.0])
            .tex_offs(0, 12).add_box([-7.0, 1.0, -3.0], [2.0, 1.0, 5.0])
            .tex_offs(16, 12).add_box([5.0, 1.0, -2.0], [2.0, 1.0, 5.0])
            .tex_offs(32, 12).add_box([-3.0, 1.0, -6.0], [6.0, 1.0, 2.0])
            .tex_offs(32, 16).add_box([-2.0, 1.0, 4.0], [6.0, 1.0, 2.0])
            .tex_offs(0, 20).add_box([-4.0, -1.0, -3.0], [4.0, 1.0, 4.0])
            .tex_offs(16, 20).add_box([1.0, -1.0, 0.0], [3.0, 1.0, 3.0]),
        64,
        32,
    )
}

fn heart() -> LayerDefinition {
    single(
        cubes()
            .tex_offs(0, 0).add_box([-2.5, -3.0, -1.0], [2.0, 1.0, 2.0])
            .tex_offs(8, 0).add_box([0.5, -3.0, -1.0], [2.0, 1.0, 2.0])
            .tex_offs(0, 4).add_box([-3.5, -2.0, -1.0], [7.0, 2.0, 2.0])
            .tex_offs(0, 8).add_box([-2.5, 0.0, -1.0], [5.0, 1.0, 2.0])
            .tex_offs(0, 12).add_box([-1.5, 1.0, -1.0], [3.0, 1.0, 2.0])
            .tex_offs(16, 0).add_box([-0.5, 2.0, -1.0], [1.0, 1.0, 2.0]),
        32,
        32,
    )
}

fn orb() -> LayerDefinition {
    single(
        cubes()
            .tex_offs(0, 0).add_box([-2.0, -2.0, -2.0], [4.0, 4.0, 4.0])
            .tex_offs(0, 8).add_box([-1.0, -1.0, -3.0], [2.0, 2.0, 6.0])
            .tex_offs(16, 0).add_box([-3.0, -1.0, -1.0], [6.0, 2.0, 2.0])
            .tex_offs(16, 4).add_box([-1.0, -3.0, -1.0], [2.0, 6.0, 2.0]),
        32,
        32,
    )
}

fn snowflake(u: i32) -> LayerDefinition {
    single(cubes().tex_offs(u, 0).add_box([-2.5, -2.5, 0.0], [5.0, 5.0, 0.0]), 32, 32)
}

fn beach_ball() -> LayerDefinition {
    single(
        cubes()
            .tex_offs(0, 0).add_box([-3.0, -3.0, -3.0], [6.0, 6.0, 6.0])
            .tex_offs(0, 12).add_box([-2.0, -2.0, -4.0], [4.0, 4.0, 8.0])
            .tex_offs(24, 0).add_box([-4.0, -2.0, -2.0], [8.0, 4.0, 4.0])
            .tex_offs(24, 8).add_box([-2.0, -4.0, -2.0], [4.0, 8.0, 4.0]),
        64,
        32,
    )
}

/// Ghost Aura: Java's three ghost models as "ghost0".."ghost2", each placed round the
/// player by animate_cosmetic (each ghost's skirt and arms move on their own).
fn ghosts() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    for k in 0..3 {
        let g = mesh.add_or_replace_child(root, &format!("ghost{k}"), cubes(), PartPose::ZERO);
        mesh.add_or_replace_child(g, "head", cubes().tex_offs(0, 0).add_box([-2.5, -5.0, -2.5], [5.0, 5.0, 5.0]), PartPose::ZERO);
        mesh.add_or_replace_child(g, "skirt", cubes().tex_offs(0, 10).add_box([-2.5, 0.0, -2.5], [5.0, 3.0, 5.0]), PartPose::ZERO);
        mesh.add_or_replace_child(
            g,
            "right_arm",
            cubes().tex_offs(20, 0).add_box([-2.0, -0.5, -0.5], [2.0, 1.0, 1.0]),
            PartPose::offset(-2.5, -1.5, -0.5),
        );
        mesh.add_or_replace_child(
            g,
            "left_arm",
            cubes().tex_offs(20, 0).add_box([0.0, -0.5, -0.5], [2.0, 1.0, 1.0]),
            PartPose::offset(2.5, -1.5, -0.5),
        );
    }
    LayerDefinition::create(mesh, 64, 32)
}

fn slime() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    mesh.add_or_replace_child(root, "core", cubes().tex_offs(0, 16).add_box([-3.0, -3.0, -3.0], [6.0, 6.0, 6.0]), PartPose::ZERO);
    mesh.add_or_replace_child(root, "jelly", cubes().tex_offs(0, 0).add_box([-4.0, -4.0, -4.0], [8.0, 8.0, 8.0]), PartPose::ZERO);
    LayerDefinition::create(mesh, 64, 32)
}

fn owl() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    mesh.add_or_replace_child(root, "body", cubes().tex_offs(0, 0).add_box([-3.0, -7.0, -2.5], [6.0, 7.0, 5.0]), PartPose::ZERO);
    mesh.add_or_replace_child(
        root,
        "head",
        cubes()
            .tex_offs(0, 12).add_box([-3.0, -5.0, -3.0], [6.0, 5.0, 6.0])
            .tex_offs(24, 0).add_box([-3.0, -7.0, -1.0], [1.0, 2.0, 1.0])
            .tex_offs(28, 0).add_box([2.0, -7.0, -1.0], [1.0, 2.0, 1.0])
            .tex_offs(24, 4).add_box([-0.5, -2.5, -3.6], [1.0, 1.0, 1.0]),
        PartPose::offset(0.0, -7.0, 0.0),
    );
    LayerDefinition::create(mesh, 64, 32)
}

fn owl_eyes() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    mesh.add_or_replace_child(
        root,
        "head",
        cubes().tex_offs(32, 12).add_box([-3.0, -5.0, -3.1], [6.0, 5.0, 0.0]),
        PartPose::offset(0.0, -7.0, 0.0),
    );
    LayerDefinition::create(mesh, 64, 32)
}

fn allay() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    mesh.add_or_replace_child(root, "head", cubes().tex_offs(0, 0).add_box([-2.5, -5.0, -2.5], [5.0, 5.0, 5.0]), PartPose::ZERO);
    mesh.add_or_replace_child(root, "body", cubes().tex_offs(0, 10).add_box([-1.5, 0.0, -1.0], [3.0, 4.0, 2.0]), PartPose::ZERO);
    mesh.add_or_replace_child(
        root,
        "right_arm",
        cubes().tex_offs(24, 0).add_box([-1.0, 0.0, -1.0], [1.0, 4.0, 2.0]),
        PartPose::offset(-1.5, 0.5, 0.0),
    );
    mesh.add_or_replace_child(
        root,
        "left_arm",
        cubes().tex_offs(24, 8).add_box([0.0, 0.0, -1.0], [1.0, 4.0, 2.0]),
        PartPose::offset(1.5, 0.5, 0.0),
    );
    mesh.add_or_replace_child(
        root,
        "right_wing",
        cubes().tex_offs(32, 0).add_box([0.0, -1.0, 0.0], [0.0, 5.0, 8.0]),
        PartPose::offset(-0.5, 0.5, 1.0),
    );
    mesh.add_or_replace_child(
        root,
        "left_wing",
        cubes().tex_offs(32, 14).add_box([0.0, -1.0, 0.0], [0.0, 5.0, 8.0]),
        PartPose::offset(0.5, 0.5, 1.0),
    );
    LayerDefinition::create(mesh, 64, 32)
}

fn baby_dragon() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    // sits on four short legs (y 0 is the top of the shoulder); the tail hangs down behind
    let body = mesh.add_or_replace_child(
        root,
        "body",
        cubes()
            .tex_offs(0, 0).add_box([-2.0, -6.0, -3.5], [4.0, 4.0, 7.0])
            .tex_offs(36, 0).add_box([-0.5, -7.0, -2.5], [1.0, 1.0, 1.0]).add_box([-0.5, -7.0, -0.5], [1.0, 1.0, 1.0]).add_box([-0.5, -7.0, 1.5], [1.0, 1.0, 1.0])
            .tex_offs(36, 4).add_box([-2.0, -2.0, -3.0], [1.0, 2.0, 1.0]).add_box([1.0, -2.0, -3.0], [1.0, 2.0, 1.0])
            .add_box([-2.0, -2.0, 2.0], [1.0, 2.0, 1.0]).add_box([1.0, -2.0, 2.0], [1.0, 2.0, 1.0]),
        PartPose::ZERO,
    );
    let neck = mesh.add_or_replace_child(
        body,
        "neck",
        cubes().tex_offs(24, 0).add_box([-1.0, -1.0, -3.0], [2.0, 2.0, 3.0]),
        PartPose::offset(0.0, -4.5, -3.5),
    );
    let head = mesh.add_or_replace_child(
        neck,
        "head",
        cubes().tex_offs(0, 12).add_box([-2.0, -2.0, -4.0], [4.0, 3.0, 4.0]).tex_offs(18, 12).add_box([-1.0, -1.0, -7.0], [2.0, 2.0, 3.0]),
        PartPose::offset(0.0, 0.0, -3.0),
    );
    mesh.add_or_replace_child(
        head,
        "horns",
        cubes().tex_offs(30, 12).add_box([-1.5, -2.0, -0.5], [1.0, 2.0, 1.0]).add_box([0.5, -2.0, -0.5], [1.0, 2.0, 1.0]),
        PartPose::offset_and_rotation(0.0, -2.0, -0.5, -0.6, 0.0, 0.0),
    );
    mesh.add_or_replace_child(
        head,
        "jaw",
        cubes().tex_offs(36, 12).add_box([-1.0, 0.0, -2.5], [2.0, 1.0, 3.0]),
        PartPose::offset(0.0, 1.0, -4.0),
    );
    let tail = mesh.add_or_replace_child(
        body,
        "tail",
        cubes().tex_offs(0, 20).add_box([-1.0, -1.0, 0.0], [2.0, 2.0, 4.0]),
        PartPose::offset(0.0, -4.5, 3.5),
    );
    mesh.add_or_replace_child(
        tail,
        "tail_end",
        cubes().tex_offs(14, 20).add_box([-0.5, -0.5, 0.0], [1.0, 1.0, 4.0]).tex_offs(26, 20).add_box([0.0, -2.5, 2.0], [0.0, 2.0, 2.0]),
        PartPose::offset(0.0, 0.0, 4.0),
    );
    let right_wing = mesh.add_or_replace_child(
        body,
        "right_wing",
        cubes().tex_offs(0, 28).add_box([-4.0, -0.5, -0.5], [4.0, 1.0, 1.0]).tex_offs(0, 32).add_box([-4.0, 0.0, 0.0], [4.0, 0.0, 5.0]),
        PartPose::offset(-2.0, -5.5, -1.5),
    );
    mesh.add_or_replace_child(
        right_wing,
        "tip",
        cubes().tex_offs(24, 28).add_box([-3.0, -0.5, -0.5], [3.0, 1.0, 1.0]).tex_offs(24, 32).add_box([-3.0, 0.0, 0.0], [3.0, 0.0, 5.0]),
        PartPose::offset(-4.0, 0.0, 0.0),
    );
    let left_wing = mesh.add_or_replace_child(
        body,
        "left_wing",
        cubes().mirror().tex_offs(0, 28).add_box([0.0, -0.5, -0.5], [4.0, 1.0, 1.0]).tex_offs(0, 32).add_box([0.0, 0.0, 0.0], [4.0, 0.0, 5.0]),
        PartPose::offset(2.0, -5.5, -1.5),
    );
    mesh.add_or_replace_child(
        left_wing,
        "tip",
        cubes().mirror().tex_offs(24, 28).add_box([0.0, -0.5, -0.5], [3.0, 1.0, 1.0]).tex_offs(24, 32).add_box([0.0, 0.0, 0.0], [3.0, 0.0, 5.0]),
        PartPose::offset(4.0, 0.0, 0.0),
    );
    LayerDefinition::create(mesh, 64, 64)
}

fn bee() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    mesh.add_or_replace_child(
        root,
        "body",
        cubes()
            .tex_offs(0, 0).add_box([-3.5, -3.5, -5.0], [7.0, 7.0, 10.0])
            .tex_offs(36, 0).add_box([-0.5, -0.5, 5.0], [1.0, 1.0, 2.0])
            .tex_offs(36, 4).add_box([-3.5, 3.5, -2.0], [7.0, 2.0, 0.0]).add_box([-3.5, 3.5, 0.0], [7.0, 2.0, 0.0]).add_box([-3.5, 3.5, 2.0], [7.0, 2.0, 0.0]),
        PartPose::ZERO,
    );
    mesh.add_or_replace_child(
        root,
        "antennae",
        cubes().tex_offs(44, 0).add_box([-2.0, -3.0, -0.5], [1.0, 3.0, 1.0]).add_box([1.0, -3.0, -0.5], [1.0, 3.0, 1.0]),
        PartPose::offset_and_rotation(0.0, -3.5, -4.5, 0.5, 0.0, 0.0),
    );
    mesh.add_or_replace_child(
        root,
        "right_wing",
        cubes().tex_offs(0, 18).add_box([-6.0, 0.0, 0.0], [6.0, 0.0, 5.0]),
        PartPose::offset(-1.0, -3.5, -2.0),
    );
    mesh.add_or_replace_child(
        root,
        "left_wing",
        cubes().mirror().tex_offs(0, 18).add_box([0.0, 0.0, 0.0], [6.0, 0.0, 5.0]),
        PartPose::offset(1.0, -3.5, -2.0),
    );
    LayerDefinition::create(mesh, 64, 32)
}

fn golden_sparkle() -> LayerDefinition {
    single(cubes().tex_offs(48, 20).add_box([-2.5, -2.5, 0.0], [5.0, 5.0, 0.0]), 64, 32)
}

fn pumpkin() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    mesh.add_or_replace_child(
        root,
        "body",
        cubes()
            .tex_offs(0, 0).add_box([-4.0, -3.0, -4.0], [8.0, 6.0, 8.0])
            .tex_offs(0, 14).add_box([-3.0, -4.0, -3.0], [6.0, 1.0, 6.0])
            .tex_offs(0, 21).add_box([-3.0, 3.0, -3.0], [6.0, 1.0, 6.0]),
        PartPose::ZERO,
    );
    mesh.add_or_replace_child(
        root,
        "stem",
        cubes().tex_offs(32, 0).add_box([-1.0, -2.0, -1.0], [2.0, 2.0, 2.0]),
        PartPose::offset_and_rotation(0.0, -4.0, 0.0, 0.1, 0.0, 0.2),
    );
    mesh.add_or_replace_child(
        root,
        "leaf",
        cubes().tex_offs(32, 4).add_box([0.0, 0.0, -1.0], [3.0, 0.0, 2.0]),
        PartPose::offset_and_rotation(0.8, -4.6, 0.0, 0.0, 0.5, -0.35),
    );
    // a thin wire loop hinged on the top edge of the body
    mesh.add_or_replace_child(
        root,
        "handle",
        cubes()
            .tex_offs(40, 0).add_box([-3.75, -6.0, -0.25], [0.5, 6.0, 0.5]).add_box([3.25, -6.0, -0.25], [0.5, 6.0, 0.5])
            .tex_offs(40, 8).add_box([-3.75, -6.5, -0.25], [7.5, 0.5, 0.5]),
        PartPose::offset(0.0, -3.0, 0.0),
    );
    LayerDefinition::create(mesh, 64, 64)
}

fn pumpkin_face() -> LayerDefinition {
    single(cubes().tex_offs(0, 32).add_box([-4.0, -3.0, -4.1], [8.0, 6.0, 0.0]), 64, 64)
}

fn baby_wither() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    // y 0 is the top of the shoulder bar; faces -z
    let shoulders = mesh.add_or_replace_child(root, "shoulders", cubes().tex_offs(0, 0).add_box([-7.0, 0.0, -1.0], [14.0, 2.0, 2.0]), PartPose::ZERO);
    mesh.add_or_replace_child(
        shoulders,
        "head",
        cubes().tex_offs(0, 4).add_box([-3.0, -6.0, -3.0], [6.0, 6.0, 6.0]),
        PartPose::offset(0.0, 0.5, 0.0),
    );
    mesh.add_or_replace_child(
        shoulders,
        "right_head",
        cubes().tex_offs(24, 4).add_box([-2.5, -5.0, -2.5], [5.0, 5.0, 5.0]),
        PartPose::offset(-5.5, 1.0, 0.0),
    );
    mesh.add_or_replace_child(
        shoulders,
        "left_head",
        cubes().tex_offs(24, 4).add_box([-2.5, -5.0, -2.5], [5.0, 5.0, 5.0]),
        PartPose::offset(5.5, 1.0, 0.0),
    );
    let spine = mesh.add_or_replace_child(
        shoulders,
        "spine",
        cubes()
            .tex_offs(44, 4).add_box([-1.0, 0.0, -1.0], [2.0, 7.0, 2.0])
            .tex_offs(0, 16).add_box([-3.5, 1.0, -1.5], [7.0, 1.0, 2.0]).add_box([-3.0, 3.0, -1.5], [6.0, 1.0, 2.0]).add_box([-2.5, 5.0, -1.5], [5.0, 1.0, 2.0]),
        PartPose::offset(0.0, 2.0, 0.0),
    );
    let tail = mesh.add_or_replace_child(
        spine,
        "tail",
        cubes().tex_offs(20, 16).add_box([-0.5, 0.0, -0.5], [1.0, 4.0, 1.0]),
        PartPose::offset(0.0, 7.0, 0.0),
    );
    mesh.add_or_replace_child(
        tail,
        "tail_end",
        cubes().tex_offs(20, 16).add_box([-0.5, 0.0, -0.5], [1.0, 3.0, 1.0]),
        PartPose::offset(0.0, 4.0, 0.0),
    );
    LayerDefinition::create(mesh, 64, 64)
}

fn baby_wither_glow() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    let shoulders = mesh.add_or_replace_child(root, "shoulders", cubes(), PartPose::ZERO);
    mesh.add_or_replace_child(
        shoulders,
        "head",
        cubes().tex_offs(0, 32).add_box([-3.0, -6.0, -3.05], [6.0, 6.0, 0.0]),
        PartPose::offset(0.0, 0.5, 0.0),
    );
    mesh.add_or_replace_child(
        shoulders,
        "right_head",
        cubes().tex_offs(16, 32).add_box([-2.5, -5.0, -2.55], [5.0, 5.0, 0.0]),
        PartPose::offset(-5.5, 1.0, 0.0),
    );
    mesh.add_or_replace_child(
        shoulders,
        "left_head",
        cubes().tex_offs(16, 32).add_box([-2.5, -5.0, -2.55], [5.0, 5.0, 0.0]),
        PartPose::offset(5.5, 1.0, 0.0),
    );
    let spine = mesh.add_or_replace_child(shoulders, "spine", cubes(), PartPose::offset(0.0, 2.0, 0.0));
    // between the first two ribs
    mesh.add_or_replace_child(
        spine,
        "star",
        cubes().tex_offs(32, 32).add_box([-0.5, -0.5, -0.5], [1.0, 1.0, 1.0]),
        PartPose::offset(0.0, 2.5, -1.6),
    );
    LayerDefinition::create(mesh, 64, 64)
}

fn phantom() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    let body = mesh.add_or_replace_child(root, "body", cubes().tex_offs(0, 0).add_box([-2.5, -1.5, -4.5], [5.0, 3.0, 9.0]), PartPose::ZERO);
    mesh.add_or_replace_child(
        body,
        "head",
        cubes().tex_offs(0, 12).add_box([-3.5, -1.5, -5.0], [7.0, 3.0, 5.0]),
        PartPose::offset_and_rotation(0.0, 0.0, -4.5, 0.2, 0.0, 0.0),
    );
    let tail = mesh.add_or_replace_child(
        body,
        "tail",
        cubes().tex_offs(28, 0).add_box([-1.5, -1.0, 0.0], [3.0, 2.0, 6.0]),
        PartPose::offset(0.0, -0.5, 4.5),
    );
    mesh.add_or_replace_child(
        tail,
        "tail_end",
        cubes().tex_offs(46, 0).add_box([-0.5, -0.5, 0.0], [1.0, 1.0, 6.0]),
        PartPose::offset(0.0, 0.0, 6.0),
    );
    let right_wing = mesh.add_or_replace_child(
        body,
        "right_wing",
        cubes().tex_offs(0, 20).add_box([-6.0, 0.0, 0.0], [6.0, 1.0, 8.0]),
        PartPose::offset(-2.5, -1.5, -4.0),
    );
    mesh.add_or_replace_child(
        right_wing,
        "tip",
        cubes().tex_offs(0, 30).add_box([-9.0, 0.0, 0.0], [9.0, 0.0, 8.0]),
        PartPose::offset(-6.0, 0.5, 0.0),
    );
    let left_wing = mesh.add_or_replace_child(
        body,
        "left_wing",
        cubes().mirror().tex_offs(0, 20).add_box([0.0, 0.0, 0.0], [6.0, 1.0, 8.0]),
        PartPose::offset(2.5, -1.5, -4.0),
    );
    mesh.add_or_replace_child(
        left_wing,
        "tip",
        cubes().mirror().tex_offs(0, 30).add_box([0.0, 0.0, 0.0], [9.0, 0.0, 8.0]),
        PartPose::offset(6.0, 0.5, 0.0),
    );
    LayerDefinition::create(mesh, 64, 64)
}

fn phantom_eyes() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    let body = mesh.add_or_replace_child(root, "body", cubes(), PartPose::ZERO);
    mesh.add_or_replace_child(
        body,
        "head",
        cubes().tex_offs(0, 56).add_box([-3.5, -1.5, -5.05], [7.0, 3.0, 0.0]),
        PartPose::offset_and_rotation(0.0, 0.0, -4.5, 0.2, 0.0, 0.0),
    );
    LayerDefinition::create(mesh, 64, 64)
}

fn ghast() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    mesh.add_or_replace_child(root, "body", cubes().tex_offs(0, 0).add_box([-4.0, -4.0, -4.0], [8.0, 8.0, 8.0]), PartPose::ZERO);
    // a 3x3 grid of tentacles, the middle row pushed back a little
    let lengths = [5, 7, 4, 6, 4, 7, 3, 5, 6];
    for k in 0..9 {
        let x = ((k % 3) as f32 - 1.0) * 2.6 + if k / 3 == 1 { 0.6 } else { 0.0 };
        let z = ((k / 3) as f32 - 1.0) * 2.6;
        mesh.add_or_replace_child(
            root,
            &format!("tentacle{k}"),
            cubes().tex_offs(32, 0).add_box([-0.5, 0.0, -0.5], [1.0, lengths[k] as f32, 1.0]),
            PartPose::offset(x, 3.8, z),
        );
    }
    // the tear; animate_cosmetic slides it down the face and lets it fall
    mesh.add_or_replace_child(root, "tear", cubes().tex_offs(40, 0).add_box([-0.5, -0.5, -0.5], [1.0, 1.0, 1.0]), PartPose::ZERO);
    LayerDefinition::create(mesh, 64, 32)
}

/// Creeper Buddy: Java draws head, body and one leg (four times) as separate models posed
/// with the pose stack; here they are one mesh, the legs "leg0".."leg3" at Java's offsets.
/// v is 0 for the normal skin, 32 for the white flash.
fn creeper(v: i32) -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    // front-right, front-left, back-right, back-left (Java's leg 0..3)
    for leg in 0..4 {
        mesh.add_or_replace_child(
            root,
            &format!("leg{leg}"),
            cubes().tex_offs(32, 12 + v).add_box([-1.5, 0.0, -1.5], [3.0, 4.0, 3.0]),
            PartPose::offset(if leg % 2 == 0 { -1.5 } else { 1.5 }, -4.0, if leg < 2 { -1.5 } else { 1.5 }),
        );
    }
    mesh.add_or_replace_child(root, "body", cubes().tex_offs(32, v).add_box([-3.0, -11.0, -2.0], [6.0, 7.0, 4.0]), PartPose::ZERO);
    mesh.add_or_replace_child(
        root,
        "head",
        cubes().tex_offs(0, v).add_box([-4.0, -8.0, -4.0], [8.0, 8.0, 8.0]),
        PartPose::offset(0.0, -11.0, 0.0),
    );
    LayerDefinition::create(mesh, 64, 64)
}

fn fox_spirit() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    // lying on the shoulder facing forward; the head turned outward and tilted onto its cheek
    let head = mesh.add_or_replace_child(
        root,
        "head",
        cubes().tex_offs(0, 12).add_box([-2.5, -2.0, -4.0], [5.0, 4.0, 4.0]).tex_offs(20, 12).add_box([-1.5, 0.0, -6.0], [3.0, 2.0, 2.0]),
        PartPose::offset_and_rotation(0.5, -2.0, -3.0, 0.08, -0.5, 0.18),
    );
    mesh.add_or_replace_child(
        head,
        "right_ear",
        cubes().tex_offs(32, 12).add_box([-1.0, -2.0, -0.5], [2.0, 2.0, 1.0]),
        PartPose::offset_and_rotation(-1.5, -2.0, -1.5, 0.0, 0.0, -0.15),
    );
    mesh.add_or_replace_child(
        head,
        "left_ear",
        cubes().tex_offs(38, 12).add_box([-1.0, -2.0, -0.5], [2.0, 2.0, 1.0]),
        PartPose::offset_and_rotation(1.5, -2.0, -1.5, 0.0, 0.0, 0.15),
    );
    mesh.add_or_replace_child(
        root,
        "paws",
        cubes().tex_offs(48, 12).add_box([-2.0, -1.0, -2.0], [1.0, 1.0, 2.0]).add_box([0.5, -1.0, -2.5], [1.0, 1.0, 2.0]),
        PartPose::offset(0.0, 0.0, -3.0),
    );
    // five tail segments {u, v, width, length}, each a child "tail" of the last
    let segments: [[i32; 4]; 5] = [[0, 22, 3, 3], [12, 22, 4, 3], [26, 22, 4, 3], [40, 22, 3, 3], [52, 22, 3, 3]];
    let mut parent = root;
    for k in 0..segments.len() {
        let s = segments[k];
        let w = s[2] as f32;
        let pose = if k == 0 {
            PartPose::offset(0.0, -1.75, 3.5)
        } else {
            PartPose::offset(0.0, 0.0, segments[k - 1][3] as f32 - 0.5)
        };
        parent = mesh.add_or_replace_child(
            parent,
            "tail",
            cubes().tex_offs(s[0], s[1]).add_box([-w / 2.0, -w / 2.0, 0.0], [w, w, s[3] as f32]),
            pose,
        );
    }
    LayerDefinition::create(mesh, 64, 64)
}

fn fox_body() -> LayerDefinition {
    single(cubes().tex_offs(0, 0).add_box([-3.0, -4.0, -3.0], [6.0, 4.0, 7.0]), 64, 64)
}

fn fox_wisp() -> LayerDefinition {
    single(cubes().tex_offs(48, 0).add_box([-1.0, -1.0, -1.0], [2.0, 2.0, 2.0]), 64, 64)
}

fn bat() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    mesh.add_or_replace_child(root, "body", cubes().tex_offs(0, 0).add_box([-1.5, -1.5, -2.0], [3.0, 3.0, 5.0]), PartPose::ZERO);
    mesh.add_or_replace_child(
        root,
        "head",
        cubes()
            .tex_offs(16, 0).add_box([-2.0, -2.5, -3.0], [4.0, 4.0, 3.0])
            .tex_offs(32, 0).add_box([-2.0, -4.5, -1.5], [1.0, 2.0, 1.0]).add_box([1.0, -4.5, -1.5], [1.0, 2.0, 1.0])
            .tex_offs(36, 0).add_box([-1.0, -0.5, -4.0], [2.0, 1.0, 1.0]),
        PartPose::offset(0.0, -0.5, -2.0),
    );
    let right_wing = mesh.add_or_replace_child(
        root,
        "right_wing",
        cubes().tex_offs(0, 10).add_box([-6.0, 0.0, -2.0], [6.0, 0.0, 5.0]),
        PartPose::offset(-1.5, -1.0, 0.0),
    );
    mesh.add_or_replace_child(
        right_wing,
        "tip",
        cubes().tex_offs(0, 16).add_box([-6.0, 0.0, -2.0], [6.0, 0.0, 5.0]),
        PartPose::offset(-6.0, 0.0, 0.0),
    );
    let left_wing = mesh.add_or_replace_child(
        root,
        "left_wing",
        cubes().mirror().tex_offs(0, 10).add_box([0.0, 0.0, -2.0], [6.0, 0.0, 5.0]),
        PartPose::offset(1.5, -1.0, 0.0),
    );
    mesh.add_or_replace_child(
        left_wing,
        "tip",
        cubes().mirror().tex_offs(0, 16).add_box([0.0, 0.0, -2.0], [6.0, 0.0, 5.0]),
        PartPose::offset(6.0, 0.0, 0.0),
    );
    LayerDefinition::create(mesh, 64, 32)
}

fn bat_eyes() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    mesh.add_or_replace_child(
        root,
        "head",
        cubes().tex_offs(0, 24).add_box([-2.0, -2.5, -3.05], [4.0, 4.0, 0.0]),
        PartPose::offset(0.0, -0.5, -2.0),
    );
    LayerDefinition::create(mesh, 64, 32)
}

fn reindeer() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    // faces -z; the body's middle at 0, the hooves 7.5 below
    mesh.add_or_replace_child(root, "body", cubes().tex_offs(0, 0).add_box([-3.0, -3.0, -5.0], [6.0, 6.0, 10.0]), PartPose::ZERO);
    let head = mesh.add_or_replace_child(
        root,
        "head",
        cubes()
            .tex_offs(32, 0).add_box([-1.5, -4.0, -1.5], [3.0, 5.0, 3.0])
            .tex_offs(0, 16).add_box([-2.5, -8.5, -3.0], [5.0, 5.0, 5.0])
            .tex_offs(20, 16).add_box([-1.5, -6.0, -5.0], [3.0, 2.0, 2.0])
            .tex_offs(0, 28).add_box([-2.0, -1.0, -2.0], [4.0, 1.0, 4.0])
            .tex_offs(16, 28).add_box([-1.0, -0.2, -2.8], [2.0, 2.0, 1.0]),
        PartPose::offset(0.0, -2.0, -4.5),
    );
    mesh.add_or_replace_child(
        head,
        "right_ear",
        cubes().tex_offs(32, 8).add_box([-2.0, -0.5, -0.5], [2.0, 1.0, 1.0]),
        PartPose::offset(-2.5, -7.5, -0.5),
    );
    mesh.add_or_replace_child(
        head,
        "left_ear",
        cubes().mirror().tex_offs(32, 8).add_box([0.0, -0.5, -0.5], [2.0, 1.0, 1.0]),
        PartPose::offset(2.5, -7.5, -0.5),
    );
    mesh.add_or_replace_child(
        head,
        "right_antler",
        cubes().tex_offs(44, 8).add_box([-4.0, -5.0, 0.0], [5.0, 5.0, 0.0]),
        PartPose::offset_and_rotation(-1.2, -8.5, -0.5, 0.0, 0.35, -0.15),
    );
    mesh.add_or_replace_child(
        head,
        "left_antler",
        cubes().mirror().tex_offs(44, 8).add_box([-1.0, -5.0, 0.0], [5.0, 5.0, 0.0]),
        PartPose::offset_and_rotation(1.2, -8.5, -0.5, 0.0, -0.35, 0.15),
    );
    for k in 0..4 {
        let x = if k % 2 == 0 { -1.8 } else { 1.8 };
        let z = if k < 2 { -3.5 } else { 3.5 };
        let name = format!("{}{}_leg", if k < 2 { "front_" } else { "back_" }, if k % 2 == 0 { "right" } else { "left" });
        mesh.add_or_replace_child(
            root,
            &name,
            cubes().set_mirror(k % 2 == 1).tex_offs(40, 16).add_box([-1.0, 0.0, -1.0], [2.0, 5.0, 2.0]),
            PartPose::offset(x, 2.5, z),
        );
    }
    mesh.add_or_replace_child(
        root,
        "tail",
        cubes().tex_offs(48, 16).add_box([-1.0, -1.0, 0.0], [2.0, 2.0, 1.0]),
        PartPose::offset_and_rotation(0.0, -2.5, 5.0, -0.5, 0.0, 0.0),
    );
    LayerDefinition::create(mesh, 64, 64)
}

fn reindeer_nose() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    mesh.add_or_replace_child(
        root,
        "head",
        cubes().tex_offs(44, 0).add_box([-1.0, -6.25, -5.8], [2.0, 2.0, 1.0]),
        PartPose::offset(0.0, -2.0, -4.5),
    );
    LayerDefinition::create(mesh, 64, 64)
}

fn crab() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    // faces -z with its feet at y 0
    mesh.add_or_replace_child(
        root,
        "shell",
        cubes()
            .tex_offs(0, 0).add_box([-4.0, -5.0, -3.0], [8.0, 3.0, 6.0])
            .tex_offs(28, 0).add_box([-3.0, -6.0, -2.0], [6.0, 1.0, 4.0])
            .tex_offs(28, 5).add_box([-3.0, -2.0, -2.0], [6.0, 1.0, 4.0]),
        PartPose::ZERO,
    );
    for s in [-1i32, 1] {
        let sf = s as f32;
        let side = if s < 0 { "right" } else { "left" };
        mesh.add_or_replace_child(
            root,
            &format!("{side}_eye"),
            cubes().set_mirror(s > 0).tex_offs(48, 0).add_box([-0.5, -3.0, -0.5], [1.0, 3.0, 1.0]),
            PartPose::offset(sf * 1.5, -5.5, -2.2),
        );
        let arm = mesh.add_or_replace_child(
            root,
            &format!("{side}_claw"),
            cubes().set_mirror(s > 0).tex_offs(52, 0).add_box([-0.5, -0.5, -3.0], [1.0, 1.0, 3.0]),
            PartPose::offset_and_rotation(sf * 3.5, -3.5, -3.0, 0.0, -sf * 0.5, 0.0),
        );
        let pincer = mesh.add_or_replace_child(
            arm,
            "pincer",
            cubes()
                .set_mirror(s > 0)
                .tex_offs(0, 9).add_box([-1.5, -1.5, -3.0], [3.0, 2.0, 3.0])
                .tex_offs(12, 9).add_box([-1.0, -0.5, -5.0], [2.0, 1.0, 2.0]),
            PartPose::offset(0.0, 0.0, -3.0),
        );
        mesh.add_or_replace_child(
            pincer,
            "jaw",
            cubes().set_mirror(s > 0).tex_offs(20, 9).add_box([-1.0, 0.0, -2.0], [2.0, 1.0, 2.0]),
            PartPose::offset(0.0, -1.5, -3.0),
        );
        for k in 0..3 {
            let thigh = mesh.add_or_replace_child(
                root,
                &format!("{side}_leg{k}"),
                cubes()
                    .set_mirror(s > 0)
                    .tex_offs(0, 14)
                    .add_box([if s < 0 { -3.0 } else { 0.0 }, -0.5, -0.5], [3.0, 1.0, 1.0]),
                PartPose::offset_and_rotation(sf * 4.0, -3.5, -1.5 + k as f32 * 1.6, 0.0, 0.0, -sf * 0.5),
            );
            mesh.add_or_replace_child(
                thigh,
                "shin",
                cubes().set_mirror(s > 0).tex_offs(8, 14).add_box([-0.5, 0.0, -0.5], [1.0, 5.0, 1.0]),
                PartPose::offset_and_rotation(sf * 3.0, 0.0, 0.0, 0.0, 0.0, sf * 0.25),
            );
        }
    }
    LayerDefinition::create(mesh, 64, 32)
}

fn love_bird() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    mesh.add_or_replace_child(root, "body", cubes().tex_offs(0, 0).add_box([-1.5, -1.5, -2.5], [3.0, 3.0, 5.0]), PartPose::ZERO);
    mesh.add_or_replace_child(
        root,
        "head",
        cubes()
            .tex_offs(16, 0).add_box([-1.5, -3.0, -2.5], [3.0, 3.0, 3.0])
            .tex_offs(28, 0).add_box([-0.5, -1.75, -3.5], [1.0, 1.0, 1.0])
            .tex_offs(32, 0).add_box([-1.5, -1.25, -3.9], [3.0, 3.0, 0.0]),
        PartPose::offset(0.0, -1.0, -2.5),
    );
    let right_wing = mesh.add_or_replace_child(
        root,
        "right_wing",
        cubes().tex_offs(0, 8).add_box([-4.0, 0.0, -1.5], [4.0, 0.0, 4.0]),
        PartPose::offset(-1.5, -1.25, -1.0),
    );
    mesh.add_or_replace_child(
        right_wing,
        "tip",
        cubes().tex_offs(0, 12).add_box([-3.0, 0.0, -1.0], [3.0, 0.0, 3.0]),
        PartPose::offset(-4.0, 0.0, 0.0),
    );
    let left_wing = mesh.add_or_replace_child(
        root,
        "left_wing",
        cubes().mirror().tex_offs(0, 8).add_box([0.0, 0.0, -1.5], [4.0, 0.0, 4.0]),
        PartPose::offset(1.5, -1.25, -1.0),
    );
    mesh.add_or_replace_child(
        left_wing,
        "tip",
        cubes().mirror().tex_offs(0, 12).add_box([0.0, 0.0, -1.0], [3.0, 0.0, 3.0]),
        PartPose::offset(4.0, 0.0, 0.0),
    );
    mesh.add_or_replace_child(
        root,
        "tail",
        cubes().tex_offs(20, 8).add_box([-1.5, 0.0, 0.0], [3.0, 0.0, 4.0]),
        PartPose::offset_and_rotation(0.0, -0.5, 2.5, -0.15, 0.0, 0.0),
    );
    LayerDefinition::create(mesh, 64, 32)
}

fn axolotl() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    mesh.add_or_replace_child(root, "body", cubes().tex_offs(0, 0).add_box([-2.5, -1.5, -4.0], [5.0, 3.0, 8.0]), PartPose::ZERO);
    let head = mesh.add_or_replace_child(
        root,
        "head",
        cubes().tex_offs(0, 11).add_box([-3.0, -2.0, -4.0], [6.0, 4.0, 4.0]),
        PartPose::offset(0.0, 0.0, -4.0),
    );
    mesh.add_or_replace_child(
        head,
        "right_gill",
        cubes().tex_offs(20, 11).add_box([-3.0, -2.0, 0.0], [3.0, 4.0, 0.0]),
        PartPose::offset_and_rotation(-3.0, 0.0, -2.0, 0.0, -0.6, 0.0),
    );
    mesh.add_or_replace_child(
        head,
        "left_gill",
        cubes().tex_offs(26, 11).add_box([0.0, -2.0, 0.0], [3.0, 4.0, 0.0]),
        PartPose::offset_and_rotation(3.0, 0.0, -2.0, 0.0, 0.6, 0.0),
    );
    mesh.add_or_replace_child(
        root,
        "tail",
        cubes().tex_offs(36, 0).add_box([0.0, -1.5, 0.0], [0.0, 3.0, 7.0]),
        PartPose::offset(0.0, 0.0, 4.0),
    );
    for k in 0..4 {
        mesh.add_or_replace_child(
            root,
            &format!("leg{k}"),
            cubes().tex_offs(26, 19).add_box([-0.5, 0.0, -0.5], [1.0, 2.0, 1.0]),
            PartPose::offset(if k % 2 == 0 { -2.5 } else { 2.5 }, 1.5, if k < 2 { -2.5 } else { 2.5 }),
        );
    }
    LayerDefinition::create(mesh, 64, 32)
}

fn mini_golem() -> LayerDefinition {
    let mut mesh = MeshDefinition::new();
    let root = mesh.root();
    mesh.add_or_replace_child(root, "body", cubes().tex_offs(0, 0).add_box([-3.0, -3.0, -2.0], [6.0, 6.0, 4.0]), PartPose::ZERO);
    mesh.add_or_replace_child(
        root,
        "head",
        cubes().tex_offs(0, 10).add_box([-2.5, -5.0, -2.5], [5.0, 5.0, 5.0]).tex_offs(20, 10).add_box([-0.5, -2.5, -3.5], [1.0, 2.0, 1.0]),
        PartPose::offset(0.0, -3.0, 0.0),
    );
    mesh.add_or_replace_child(
        root,
        "right_arm",
        cubes().tex_offs(24, 0).add_box([-2.0, 0.0, -1.0], [2.0, 8.0, 2.0]),
        PartPose::offset(-3.0, -3.0, 0.0),
    );
    mesh.add_or_replace_child(
        root,
        "left_arm",
        cubes().tex_offs(36, 0).add_box([0.0, 0.0, -1.0], [2.0, 8.0, 2.0]),
        PartPose::offset(3.0, -3.0, 0.0),
    );
    mesh.add_or_replace_child(
        root,
        "right_leg",
        cubes().tex_offs(48, 0).add_box([-1.0, 0.0, -1.0], [2.0, 4.0, 2.0]),
        PartPose::offset(-1.5, 3.0, 0.0),
    );
    mesh.add_or_replace_child(
        root,
        "left_leg",
        cubes().tex_offs(48, 8).add_box([-1.0, 0.0, -1.0], [2.0, 4.0, 2.0]),
        PartPose::offset(1.5, 3.0, 0.0),
    );
    LayerDefinition::create(mesh, 64, 32)
}

/// Every accessory and pet layer, by id.
pub fn accessory_layers() -> Vec<(String, LayerDefinition)> {
    let mut out: Vec<(String, LayerDefinition)> = Vec::new();
    let mut add = |kind: &str, id: &str, layer: &str, def: LayerDefinition| {
        out.push((format!("arsenal:{kind}_{id}#{layer}"), def));
    };
    let a = "accessory";
    for id in ["halo", "untouchable"] {
        add(a, id, "main", halo());
    }
    add(a, "orbiting_crystals", "main", crystal());
    // the +x wing; "#left" is Java's mirrored wing (featherWing(-1) / membraneWing(-1))
    for id in ["wings", "ember_wings", "vip_accessory"] {
        add(a, id, "main", feather_wing(1));
        add(a, id, "left", feather_wing(-1));
    }
    add(a, "ender_wings", "main", membrane_wing(1));
    add(a, "ender_wings", "left", membrane_wing(-1));
    add(a, "rune_circle", "main", runes());
    for id in [
        "totem_orbit",
        "bronze_sword",
        "ambassador_badge",
        "supporter_accessory",
        "first_week_badge",
        "daily_player",
        "popularity_stars",
        "lucky_clover",
    ] {
        add(a, id, "main", totem());
    }
    add(a, "cloud_trail", "main", cloud());
    add(a, "friendship", "main", heart());
    add(a, "linked_badge", "main", orb());
    add(a, "completionist_halo", "main", completionist_halo());
    add(a, "ghost_aura", "main", ghosts());
    add(a, "snowfall", "main", snowflake(0));
    add(a, "snowfall", "flake1", snowflake(10));
    add(a, "snowfall", "flake2", snowflake(20));
    add(a, "beach_ball", "main", beach_ball());

    let p = "pet";
    add(p, "slime", "main", slime());
    add(p, "owl", "main", owl());
    add(p, "night_owl", "main", owl());
    add(p, "night_owl", "eyes", owl_eyes());
    add(p, "allay", "main", allay());
    add(p, "baby_dragon", "main", baby_dragon());
    add(p, "vip_pet", "main", baby_dragon());
    add(p, "bee", "main", bee());
    add(p, "golden_bee", "main", bee());
    add(p, "golden_bee", "sparkle", golden_sparkle());
    add(p, "pumpkin_lantern", "main", pumpkin());
    add(p, "pumpkin_lantern", "face", pumpkin_face());
    add(p, "baby_wither", "main", baby_wither());
    add(p, "baby_wither", "glow", baby_wither_glow());
    add(p, "phantom", "main", phantom());
    add(p, "phantom", "eyes", phantom_eyes());
    add(p, "ghast", "main", ghast());
    add(p, "creeper_buddy", "main", creeper(0));
    add(p, "creeper_buddy", "flash", creeper(32));
    add(p, "fox_spirit", "main", fox_spirit());
    add(p, "fox_spirit", "body", fox_body());
    add(p, "fox_spirit", "wisp", fox_wisp());
    add(p, "bat", "main", bat());
    add(p, "bat", "eyes", bat_eyes());
    add(p, "reindeer", "main", reindeer());
    add(p, "reindeer", "nose", reindeer_nose());
    add(p, "crab", "main", crab());
    add(p, "love_bird", "main", love_bird());
    add(p, "axolotl", "main", axolotl());
    add(p, "mini_golem", "main", mini_golem());
    out
}

// ---------------------------------------------------------------------------
// Shared motion
// ---------------------------------------------------------------------------

/// Wings: the +x wing folded flat on the back while gliding (sweep, arm, forearm, hand, then each tip).
const WING_FOLD: [f32; 9] = [0.0, 1.3, -0.15, -0.1, -0.7, -0.74, -0.78, -0.82, -0.86];
const ENDER_WING_FOLD: [f32; 8] = [0.0, 1.3, -0.15, -0.1, -0.45, -0.55, -0.65, -0.75];
const WING_FOLD_SCALE: f32 = 0.55;
const WING_FOLD_TILT: f32 = 0.2;

fn lerp(delta: f32, from: f32, to: f32) -> f32 {
    from + (to - from) * delta
}

/// How far the wings are folded: Java eases this over 0.3 s; here folded while gliding, else open.
fn wing_fold(ctx: &Ctx) -> f32 {
    if ctx.gliding { 1.0 } else { 0.0 }
}

struct WingMotion {
    breath: f64,
    open: f64,
    a: f64,
    stroke: f64,
    sweep: f64,
    fan: f64,
}

/// The wings breathe, and every 9 seconds open out and beat twice.
fn wing_motion(t: f64) -> WingMotion {
    let breath = (t * 1.3).sin();
    let cycle = t % 9.0;
    let (mut open, mut beat) = (0.0, 0.0);
    if cycle < 2.2 {
        let u = cycle / 2.2;
        open = (PI * u).sin().powi(2);
        beat = u * 2.0;
    }
    let a = 2.0 * PI * beat;
    let stroke = a.sin();
    let sweep = 0.82 + 0.04 * breath - open * (0.42 + 0.12 * stroke);
    let fan = 0.012 * (1.0 + breath) + open * (0.05 + 0.04 * (1.0 - stroke));
    WingMotion { breath, open, a, stroke, sweep, fan }
}

/// Vanilla's left leg swing (HumanoidModel: cos(walk * 0.6662 + PI) * 1.4 * walkSpeed).
fn left_leg_x_rot(ctx: &Ctx) -> f32 {
    (ctx.walk * 0.6662 + std::f32::consts::PI).cos() * 1.4 * ctx.walk_speed
}

/// Vanilla's body lean while crouching (HumanoidModel: 0.5).
fn body_x_rot(ctx: &Ctx) -> f32 {
    if ctx.sneaking { 0.5 } else { 0.0 }
}

/// Pins sit just above the jacket layer, or on the chestplate.
fn surface(ctx: &Ctx) -> f64 {
    if ctx.has_chestplate { 3.15 } else { 2.4 }
}

/// Which side of the chest a pin goes on (1 = the +x side). Java moves left pins to the right
/// under the Winter Scarf; Ctx has no hat, so that case is not handled.
fn chest_side(left: bool) -> f64 {
    if left { 1.0 } else { -1.0 }
}

fn creeper_fuse(t: f64) -> (bool, f64) {
    let phase = t % 11.0;
    let fuse = phase > 9.2;
    (fuse, if fuse { (phase - 9.2) / 1.8 } else { 0.0 })
}

fn creeper_step(ctx: &Ctx) -> f32 {
    left_leg_x_rot(ctx).clamp(-0.9, 0.9) * 1.2
}

fn crab_busy(t: f64, ctx: &Ctx) -> f64 {
    let mv = (t * 0.9).cos();
    (mv.abs() * 1.2 + (left_leg_x_rot(ctx) as f64).abs() * 1.5).min(1.0)
}

// ---------------------------------------------------------------------------
// Draws
// ---------------------------------------------------------------------------

fn tr(x: f64, y: f64, z: f64) -> Step {
    Step::Translate([x as f32, y as f32, z as f32])
}
fn rx(a: f64) -> Step {
    Step::RotateX(a as f32)
}
fn ry(a: f64) -> Step {
    Step::RotateY(a as f32)
}
fn rz(a: f64) -> Step {
    Step::RotateZ(a as f32)
}
fn sc(s: f64) -> Step {
    Step::Scale([s as f32; 3])
}
fn sc3(x: f64, y: f64, z: f64) -> Step {
    Step::Scale([x as f32, y as f32, z as f32])
}

struct Draws<'a> {
    kind: &'a str,
    id: &'a str,
    out: Vec<Draw>,
}

impl Draws<'_> {
    fn add(&mut self, layer: &str, bone: &'static str, steps: Vec<Step>, glow: bool) {
        self.out.push(Draw {
            layer: format!("arsenal:{}_{}#{layer}", self.kind, self.id),
            texture: format!("arsenal:item/{}_{}", self.kind, self.id),
            bone,
            steps,
            glow,
        });
    }
}

/// Every draw Java's AccessoryLayer.draw makes for this accessory or pet this frame.
pub fn cosmetic_draws(kind: &str, id: &str, ctx: &Ctx) -> Vec<Draw> {
    let t = ctx.t as f64;
    let mut d = Draws { kind, id, out: Vec::new() };
    match (kind, id) {
        ("accessory", "halo" | "untouchable") => {
            d.add("main", "head", vec![tr(0.0, (-12.5 + (t * 2.0).sin() * 0.6) / 16.0, 0.0), ry(t * 1.2)], true);
        }
        ("accessory", "wings" | "ender_wings" | "ember_wings" | "vip_accessory") => {
            // Ender, Ember and VIP wings glow
            let glow = id != "wings";
            let folded: &[f32] = if id == "ender_wings" { &ENDER_WING_FOLD } else { &WING_FOLD };
            let fold = wing_fold(ctx);
            let m = wing_motion(t);
            let cape_degrees = 6.0 + ctx.cape_lean / 2.0 + ctx.cape_flap;
            let cape = (cape_degrees.clamp(6.0, 60.0) as f64).to_radians();
            // the root sits 1.5 px below the cape's hinge, so it moves back as the cape swings out
            let back = 1.5 * cape.tan() + 1.0 / cape.cos() - 1.0;
            for side in [-1.0f64, 1.0] {
                let scale = lerp(fold, 1.0, WING_FOLD_SCALE) as f64;
                let z = 3.6 + lerp(fold, back as f32, if side < 0.0 { 0.3 } else { 0.0 }) as f64;
                d.add(
                    if side > 0.0 { "main" } else { "left" },
                    "body",
                    vec![
                        tr(side * 1.5 / 16.0, 1.5 / 16.0, z / 16.0),
                        Step::RotateX(lerp(fold, (cape + 0.08) as f32, WING_FOLD_TILT)),
                        Step::RotateY((-side) as f32 * lerp(fold, m.sweep as f32, folded[0])),
                        sc(scale),
                    ],
                    glow,
                );
            }
        }
        ("accessory", "orbiting_crystals") => {
            for k in 0..3 {
                let kf = k as f64;
                let a = t * 1.6 + kf * PI * 2.0 / 3.0;
                d.add(
                    "main",
                    "root",
                    vec![tr(a.cos() * 0.6, (6.0 + (t * 2.5 + kf).sin() * 1.5) / 16.0, a.sin() * 0.6), ry(t * 3.0 + kf), rx(PI / 4.0)],
                    true,
                );
            }
        }
        ("accessory", "rune_circle") => {
            // on the ground under the feet
            let breath = 1.0 + (t * 1.3).sin() * 0.04;
            d.add("main", "root", vec![tr(0.0, 23.9 / 16.0, 0.0), ry(t * 0.6), sc3(breath, 1.0, breath)], true);
        }
        ("accessory", "totem_orbit") => {
            for k in 0..2 {
                let kf = k as f64;
                let a = t * 1.1 + kf * PI;
                d.add(
                    "main",
                    "root",
                    vec![tr(a.cos() * 0.65, (5.0 + (t * 1.4 + kf * PI).sin() * 1.5) / 16.0, a.sin() * 0.65), ry(t * 1.5 + kf), sc(0.3)],
                    false,
                );
            }
        }
        ("accessory", "cloud_trail") => {
            // hidden while sneaking, gliding, swimming or flying
            if !(ctx.sneaking || ctx.gliding || ctx.swimming_or_flying) {
                d.add("main", "root", vec![tr(0.0, (22.0 - (0.5 + (t * 1.6).sin() * 0.5)) / 16.0, 0.0)], false);
            }
        }
        ("accessory", "ambassador_badge" | "bronze_sword" | "supporter_accessory") => {
            // pinned flat on the chest: the shield / heart on the left, the Bronze Sword on the right
            let shield = id != "bronze_sword";
            d.add(
                "main",
                "body",
                vec![tr(chest_side(shield) * 2.2 / 16.0, 3.3 / 16.0, -surface(ctx) / 16.0), sc(0.22)],
                false,
            );
        }
        ("accessory", "first_week_badge") => {
            // the medal swings out as you lean forward and bounces as you walk
            let leg = left_leg_x_rot(ctx);
            let lean = body_x_rot(ctx).clamp(0.0, 0.6) * 0.8;
            let walk = leg.abs() * 0.12;
            d.add(
                "main",
                "body",
                vec![
                    tr(chest_side(true) * 2.2 / 16.0, 1.4 / 16.0, -surface(ctx) / 16.0),
                    Step::RotateX(-lean - walk),
                    Step::RotateZ(leg * 0.06),
                    sc(0.24),
                    tr(0.0, 8.0 / 16.0, 0.0),
                ],
                false,
            );
        }
        ("accessory", "daily_player") => {
            // on the outside of the left upper arm; 3 is the wide arm's cube maxX (2 on slim skins)
            d.add(
                "main",
                "left_arm",
                vec![tr((3.0 + surface(ctx) - 2.0) / 16.0, 1.6 / 16.0, 0.0), ry(-PI / 2.0), sc(0.22)],
                false,
            );
        }
        ("accessory", "friendship") => {
            for k in 0..2 {
                let kf = k as f64;
                let a = t * 1.3 + kf * PI;
                d.add(
                    "main",
                    "root",
                    vec![tr(a.cos() * 0.6, (8.0 + (t * 2.0 + kf * PI).sin() * 1.5) / 16.0, a.sin() * 0.6), ry(t * 1.8 + kf), sc(0.6)],
                    true,
                );
            }
        }
        ("accessory", "linked_badge") => {
            // at the collar (Java moves it under the Winter Scarf's knot; Ctx has no hat)
            let s = 0.36 * (1.0 + (t * 2.4).sin() * 0.04);
            d.add(
                "main",
                "body",
                vec![tr(0.0, 1.3 / 16.0, -(surface(ctx) + 3.0 * 0.36 - 0.3) / 16.0), sc(s)],
                true,
            );
        }
        ("accessory", "completionist_halo") => {
            // frame and sparks are posed in animate_cosmetic
            d.add("main", "head", vec![tr(0.0, (-12.5 + (t * 2.0).sin() * 0.6) / 16.0, 0.0), ry(t * 0.8)], true);
        }
        ("accessory", "popularity_stars") => {
            for k in 0..5 {
                let kf = k as f64;
                let a = t * 0.52 + kf * PI * 2.0 / 5.0;
                d.add(
                    "main",
                    "root",
                    vec![
                        tr(a.cos() * 0.5, (-9.5 + a.sin() * 1.2 + (t * 1.7 + kf).sin() * 0.4) / 16.0, a.sin() * 0.5),
                        ry(t * 1.4 + kf),
                        sc(0.2),
                    ],
                    true,
                );
            }
        }
        ("accessory", "lucky_clover") => {
            d.add(
                "main",
                "root",
                vec![
                    tr(0.55, (-9.0 + (t * 1.3).sin() * 0.7) / 16.0, -0.05),
                    ry(t * PI * 2.0 / 7.0),
                    rz((t * 0.9).sin() * 0.12),
                    sc(0.28),
                ],
                false,
            );
        }
        ("accessory", "ghost_aura") => {
            // the three ghosts are placed round you in animate_cosmetic
            d.add("main", "root", Vec::new(), true);
        }
        ("accessory", "snowfall") => {
            for k in 0..16i32 {
                let kf = k as f64;
                let period = 6.0 + ((k * 7 % 5) as f64) * 0.8;
                let p = (t / period + kf * 0.618) % 1.0;
                let size = (p * PI).sin() * (0.5 + ((k % 3) as f64) * 0.1);
                if size < 0.04 {
                    continue;
                }
                let ang = kf * 2.4 + (t * 0.3 + kf).sin() * 0.35;
                let r = 0.55 + ((k * 5 % 7) as f64) * 0.05;
                let layer = ["main", "flake1", "flake2"][(k % 3) as usize];
                d.add(
                    layer,
                    "root",
                    vec![
                        tr(ang.cos() * r, (-16.0 + p * 39.0) / 16.0, ang.sin() * r),
                        ry(t * (0.8 + ((k % 4) as f64) * 0.3) + kf),
                        rx((t * 1.1 + kf).sin() * 0.5),
                        sc(size),
                    ],
                    true,
                );
            }
        }
        ("accessory", "beach_ball") => {
            let a = -t * 0.7;
            let hop = (t * PI / 0.75).sin().abs();
            let squash = (1.0 - hop * 5.0).max(0.0) * 0.16;
            d.add(
                "main",
                "root",
                vec![
                    tr(a.cos() * 0.95, (24.0 - 4.0 * (1.0 - squash) - hop * 9.0) / 16.0, a.sin() * 0.95),
                    ry(PI - a),
                    sc3(1.0 + squash * 0.6, 1.0 - squash, 1.0 + squash * 0.6),
                    rx(t * 2.6),
                ],
                false,
            );
        }

        ("pet", "slime") => {
            // by the right shoulder, hopping: flattens where a hop starts, stretches on the way up
            let hop = (t * 2.2).sin().abs();
            let land = (1.0 - hop * 3.5).max(0.0);
            let squish = land * 0.09 - hop * 0.04;
            d.add(
                "main",
                "root",
                vec![
                    tr(-0.6, (0.2 - hop * 3.4) / 16.0, 0.15),
                    ry((t * 0.7).sin() * 0.5),
                    sc3(0.625 + squish, 0.625 - squish, 0.625 + squish),
                ],
                false,
            );
        }
        ("pet", "owl" | "night_owl") => {
            let steps = vec![tr(6.0 / 16.0, 0.0, 0.0), sc(0.5)];
            d.add("main", "body", steps.clone(), false);
            if id == "night_owl" {
                d.add("eyes", "body", steps, true);
            }
        }
        ("pet", "pumpkin_lantern") => {
            let bob = (t * 1.5).sin();
            let steps = vec![
                tr(-0.62, (-4.5 + bob * 0.9) / 16.0, 0.1),
                ry(t * PI * 2.0 / 12.0),
                rz((t * 1.1).sin() * 0.08),
                rx((t * 0.8).sin() * 0.06),
                sc(0.55),
            ];
            d.add("main", "root", steps.clone(), false);
            d.add("face", "root", steps, true);
        }
        ("pet", "allay") => {
            let bob = (t * 1.8).sin();
            d.add(
                "main",
                "root",
                vec![
                    tr(-0.6, (-3.5 + bob * 0.8) / 16.0, 0.1),
                    ry((t * 0.6).sin() * 0.35),
                    rx(0.12 + bob * 0.04),
                    sc(0.6),
                ],
                true,
            );
        }
        ("pet", "baby_dragon" | "vip_pet") => {
            d.add("main", "body", vec![tr(6.5 / 16.0, 0.0, 0.0), sc(0.5)], false);
        }
        ("pet", "bee" | "golden_bee") => {
            let a = t * 0.9;
            let steps = vec![
                tr(a.cos() * 0.6, (-5.0 + (a * 2.0).sin() * 1.5 + (t * 9.0).sin() * 0.2) / 16.0, a.sin() * 0.6),
                ry(PI - a),
                rx((a * 2.0).cos() * 0.3),
                rz(0.3),
                sc(0.45),
            ];
            d.add("main", "root", steps.clone(), false);
            if id == "golden_bee" {
                // glitter trailing behind, in the bee's own frame
                for k in 0..3 {
                    let kf = k as f64;
                    let phase = (t * 0.8 + kf / 3.0) % 1.0;
                    let size = (phase * PI).sin() * 0.9;
                    if size < 0.05 {
                        continue;
                    }
                    let mut s = steps.clone();
                    s.push(tr(((kf - 1.0) * 3.0 + (t * 2.0 + kf).sin()) / 16.0, (1.5 + phase * 5.0) / 16.0, (7.0 + phase * 12.0) / 16.0));
                    s.push(ry(t * 3.0 + kf * 2.0));
                    s.push(sc(size));
                    d.add("sparkle", "root", s, true);
                }
            }
        }
        ("pet", "baby_wither") => {
            let bob = (t * 1.2).sin();
            let steps = vec![
                tr(0.42, (-3.5 + bob * 0.9) / 16.0, 0.5),
                ry((t * 0.5).sin() * 0.25),
                rz((t * 0.9).sin() * 0.05),
                sc(0.45),
            ];
            d.add("main", "root", steps.clone(), false);
            d.add("glow", "root", steps, true);
        }
        ("pet", "phantom") => {
            let a = t * 0.55;
            let steps = vec![
                tr(a.cos() * 1.0, (-17.0 + (t * 0.9).sin() * 1.5) / 16.0, a.sin() * 1.0),
                ry(PI - a),
                rz(0.35),
                rx((t * 0.9).cos() * 0.12),
                sc(0.35),
            ];
            d.add("main", "root", steps.clone(), false);
            d.add("eyes", "root", steps, true);
        }
        ("pet", "ghast") => {
            let bob = (t * 1.1).sin();
            d.add(
                "main",
                "root",
                vec![
                    tr(-0.68, (-7.0 + bob * 1.1) / 16.0, 0.1),
                    ry((t * 0.4).sin() * 0.4),
                    rz((t * 0.7).sin() * 0.05),
                    sc(0.5),
                ],
                false,
            );
        }
        ("pet", "creeper_buddy") => {
            // by your right foot; every 11 seconds it swells and flashes white
            let (fuse, k) = creeper_fuse(t);
            let flash = fuse && (k * k * 40.0).sin() > 0.0;
            let swell = if fuse { 1.0 + 0.1 * k + (k * 50.0).sin() * 0.015 * k } else { 1.0 };
            d.add(
                if flash { "flash" } else { "main" },
                "root",
                vec![tr(-0.72, 24.0 / 16.0, -0.1), sc(0.5), sc3(swell, 1.0 + (swell - 1.0) * 0.5, swell)],
                flash,
            );
        }
        ("pet", "fox_spirit") => {
            // curled up asleep on the left shoulder, breathing; wisps rise off it
            let breath = (t * 1.4).sin();
            let base = vec![tr(5.5 / 16.0, (-0.3 - (t * 0.7).sin() * 0.3) / 16.0, 0.3 / 16.0), sc(0.5)];
            let mut body = base.clone();
            body.push(sc3(1.0 + breath * 0.025, 1.0 + breath * 0.06, 1.0));
            d.add("body", "body", body, true);
            d.add("main", "body", base.clone(), true);
            for k in 0..3 {
                let kf = k as f64;
                let phase = (t / 3.2 + kf / 3.0) % 1.0;
                let size = (phase * PI).sin() * 0.6;
                if size < 0.05 {
                    continue;
                }
                let mut s = base.clone();
                s.push(tr(
                    ((phase * 6.0 + kf * 2.1).sin() * 3.0 + (kf - 1.0) * 2.0) / 16.0,
                    (-3.0 - phase * 14.0) / 16.0,
                    ((phase * 5.0 + kf).cos() * 2.0 + (kf - 1.0) * 2.5) / 16.0,
                ));
                s.push(ry(t * 2.0 + kf));
                s.push(sc(size));
                d.add("wisp", "body", s, true);
            }
        }
        ("pet", "bat") => {
            let a = t * 1.1;
            let steps = vec![
                tr(a.cos() * 0.7, (-11.0 + (a * 2.7).sin() * 1.5 + (t * 11.0).sin() * 0.35) / 16.0, a.sin() * 0.7),
                ry(PI - a),
                rz(0.4),
                sc(0.42),
            ];
            d.add("main", "root", steps.clone(), false);
            d.add("eyes", "root", steps, true);
        }
        ("pet", "reindeer") => {
            let stride = t * 6.5;
            let steps = vec![
                tr(0.72, (-6.0 + (t * 1.4).sin() + (stride * 2.0).sin() * 0.25) / 16.0, 0.1),
                ry((t * 0.35).sin() * 0.3),
                rx(stride.sin() * 0.05 - 0.05),
                sc(0.5),
            ];
            d.add("main", "root", steps.clone(), false);
            d.add("nose", "root", steps, true);
        }
        ("pet", "love_bird") => {
            let a = t * 0.8;
            d.add(
                "main",
                "root",
                vec![
                    tr(a.cos() * 0.62, (-10.0 + (a * 2.0).sin() * 1.2 + (t * 9.0).sin() * 0.25) / 16.0, a.sin() * 0.62),
                    ry(PI - a),
                    rz(0.3),
                    rx((a * 2.0).cos() * 0.15),
                    sc(0.6),
                ],
                false,
            );
        }
        ("pet", "axolotl") => {
            let a = t * 0.7;
            d.add(
                "main",
                "root",
                vec![
                    tr(a.cos() * 0.75, (4.0 + (t * 1.4).sin() * 1.2) / 16.0, a.sin() * 0.75),
                    ry(PI - a),
                    rz((t * 1.4).sin() * 0.1),
                    sc(0.6),
                ],
                false,
            );
        }
        ("pet", "mini_golem") => {
            d.add("main", "root", vec![tr(0.72, 24.0 / 16.0, -0.1), sc(0.6), tr(0.0, -7.0 / 16.0, 0.0)], false);
        }
        ("pet", "crab") => {
            let busy = crab_busy(t, ctx);
            d.add(
                "main",
                "root",
                vec![tr(0.68 + (t * 0.9).sin() * 0.16, (24.0 - (t * 16.0).sin().abs() * 0.3 * busy) / 16.0, -0.05), sc(0.5)],
                false,
            );
        }
        _ => {}
    }
    d.out
}

// ---------------------------------------------------------------------------
// Part poses
// ---------------------------------------------------------------------------

/// The part at `path` under `m`.
fn part<'a>(m: &'a mut ModelPart, path: &[&str]) -> Option<&'a mut ModelPart> {
    let mut p = m;
    for name in path {
        p = p.get_child_mut(name)?;
    }
    Some(p)
}

/// Runs `f` on the part at `path`, if this model has it (the extra eye / glow models
/// share their main model's part names, so one call poses both).
fn set(m: &mut ModelPart, path: &[&str], f: impl FnOnce(&mut ModelPart)) {
    if let Some(p) = part(m, path) {
        f(p);
    }
}

/// Wings: the +x wing ("#main") or its mirror ("#left", told apart by the forearm's pivot).
fn animate_wing(model: &mut ModelPart, t: f64, fold: f32, tip: &str, folded: &[f32]) {
    let side = match part(model, &["arm", "forearm"]) {
        Some(f) if f.x < 0.0 => -1.0f64,
        _ => 1.0,
    };
    let sf = side as f32;
    let m = wing_motion(t);
    let tips = folded.len() - 4;
    let Some(arm) = model.get_child_mut("arm") else { return };
    // a negative turn raises the +x wing; the mirror image turns the other way
    arm.z_rot += (side * (-0.04 * m.breath - m.open * (0.18 + 0.5 * m.stroke))) as f32;
    arm.z_rot = lerp(fold, arm.z_rot, sf * folded[1]);
    let Some(forearm) = arm.get_child_mut("forearm") else { return };
    forearm.z_rot += (side * (0.03 * m.breath - m.open * (0.1 + 0.3 * (m.a - 0.8).sin()))) as f32;
    forearm.z_rot = lerp(fold, forearm.z_rot, sf * folded[2]);
    let Some(hand) = forearm.get_child_mut("hand") else { return };
    hand.z_rot += (side * (0.04 * m.breath - m.open * (0.42 + 0.3 * (m.a - 1.6).sin()))) as f32;
    hand.z_rot = lerp(fold, hand.z_rot, sf * folded[3]);
    for k in 0..tips {
        if let Some(p) = hand.get_child_mut(&format!("{tip}{k}")) {
            p.z_rot -= (side * m.fan * k as f64) as f32;
            p.z_rot = lerp(fold, p.z_rot, sf * folded[4 + k]);
        }
    }
}

/// The part poses Java sets for this accessory or pet each frame; call after reset_all_poses().
pub fn animate_cosmetic(kind: &str, id: &str, model: &mut ModelPart, ctx: &Ctx) {
    let t = ctx.t as f64;
    let f = |v: f64| v as f32;
    match (kind, id) {
        ("accessory", "wings" | "ember_wings" | "vip_accessory") => animate_wing(model, t, wing_fold(ctx), "p", &WING_FOLD),
        ("accessory", "ender_wings") => animate_wing(model, t, wing_fold(ctx), "f", &ENDER_WING_FOLD),
        ("accessory", "completionist_halo") => {
            // a new rainbow frame 12 times a second
            let frame = (t * 12.0) as i64 % 16;
            for k in 0..16 {
                set(model, &[format!("frame{k}").as_str()], |p| p.visible = k == frame);
            }
            // four sparks circling the other way, rising, falling and twinkling
            for k in 0..4 {
                let kf = k as f64;
                let a = -t * 2.4 + kf * PI / 2.0;
                let s = f(0.7 + (t * 6.0 + kf * 1.7).sin() * 0.35);
                set(model, &[format!("spark{k}").as_str()], |p| {
                    p.set_pos(f(a.cos() * 5.0), f((t * 3.0 + kf * 1.3).sin() * 1.3), f(a.sin() * 5.0));
                    (p.x_scale, p.y_scale, p.z_scale) = (s, s, s);
                });
            }
        }
        ("accessory", "ghost_aura") => {
            // three ghosts drifting round you at chest height (Java's pose stack, as each ghost's own pose)
            for k in 0..3 {
                let kf = k as f64;
                let a = t * 0.6 + kf * PI * 2.0 / 3.0;
                let wave = (t * 2.2 + kf * 1.7).sin() * 0.35;
                set(model, &[format!("ghost{k}").as_str()], |g| {
                    g.set_pos(f(a.cos() * 0.72 * 16.0), f(4.0 + (t * 1.7 + kf * 2.0).sin() * 1.2), f(a.sin() * 0.72 * 16.0));
                    g.set_rotation(0.15, f(PI - a), 0.0);
                    (g.x_scale, g.y_scale, g.z_scale) = (0.55, 0.55, 0.55);
                    set(g, &["skirt"], |p| p.x_rot = f((t * 3.0 + kf).sin() * 0.15));
                    set(g, &["right_arm"], |p| p.z_rot = f(-0.5 - wave));
                    set(g, &["left_arm"], |p| p.z_rot = f(0.5 + wave));
                });
            }
        }

        ("pet", "owl" | "night_owl") => {
            // the head (and the Night Owl's eyes) looks around now and then
            let look = (t * 0.45).sin() + 0.6 * (t * 1.3).sin();
            set(model, &["head"], |p| {
                p.y_rot = f(look.clamp(-1.0, 1.0) * 1.1);
                p.x_rot = f((t * 0.9).sin() * 0.08);
            });
        }
        ("pet", "pumpkin_lantern") => {
            set(model, &["handle"], |p| p.x_rot = f((t * 1.5 - 1.2).sin() * 0.35));
        }
        ("pet", "allay") => {
            let bob = (t * 1.8).sin();
            let flutter = f(0.6 + (t * 22.0).sin() * 0.35);
            set(model, &["right_wing"], |p| p.y_rot = -flutter);
            set(model, &["left_wing"], |p| p.y_rot = flutter);
            // every 7 seconds the outer arm swings up and waves
            let phase = t % 7.0;
            let raise = if phase < 2.2 { ((phase / 2.2 * PI).sin() * 1.8).min(1.0) } else { 0.0 };
            set(model, &["right_arm"], |p| {
                p.z_rot = f(0.1 + raise * (2.3 + (t * 10.0).sin() * 0.35));
                p.x_rot = f(-0.3 * (1.0 - raise));
            });
            set(model, &["left_arm"], |p| {
                p.z_rot = f(-0.1 - bob * 0.05);
                p.x_rot = -0.3;
            });
        }
        ("pet", "baby_dragon" | "vip_pet") => {
            let beat = (t * 1.6).sin();
            let wing = f(1.1 + beat * 0.4);
            let tip = f(0.25 + (t * 1.6 - 0.7).sin() * 0.3);
            set(model, &["body", "right_wing"], |p| {
                p.z_rot = wing;
                p.y_rot = 1.0;
            });
            set(model, &["body", "left_wing"], |p| {
                p.z_rot = -wing;
                p.y_rot = -1.0;
            });
            set(model, &["body", "right_wing", "tip"], |p| p.z_rot = tip);
            set(model, &["body", "left_wing", "tip"], |p| p.z_rot = -tip);
            let look = ((t * 0.5).sin() + 0.5 * (t * 1.7).sin()).clamp(-1.0, 1.0);
            set(model, &["body", "neck"], |p| {
                p.x_rot = f(-0.6 + (t * 1.2).sin() * 0.05);
                p.y_rot = f(look * 0.35);
            });
            set(model, &["body", "neck", "head"], |p| {
                p.x_rot = f(0.6 + (t * 0.8).sin() * 0.12);
                p.y_rot = f(look * 0.45);
            });
            set(model, &["body", "neck", "head", "jaw"], |p| p.x_rot = f((t * 0.7).sin().max(0.0).powi(12) * 0.5));
            set(model, &["body", "tail"], |p| {
                p.x_rot = -0.7;
                p.y_rot = f((t * 1.1).sin() * 0.25);
            });
            set(model, &["body", "tail", "tail_end"], |p| {
                p.x_rot = -0.4;
                p.y_rot = f((t * 1.1 - 0.8).sin() * 0.4);
            });
        }
        ("pet", "bee" | "golden_bee") => {
            let buzz = f(0.3 + (t * 45.0).sin() * 0.5);
            set(model, &["right_wing"], |p| p.z_rot = buzz);
            set(model, &["left_wing"], |p| p.z_rot = -buzz);
        }
        ("pet", "baby_wither") => {
            // the same angles on the main model and the glow model (Java copies them over)
            set(model, &["shoulders", "head"], |p| {
                p.y_rot = f(((t * 0.4).sin() + 0.5 * (t * 1.1).sin()).clamp(-1.0, 1.0) * 0.6);
                p.x_rot = f((t * 0.7).sin() * 0.1);
            });
            set(model, &["shoulders", "right_head"], |p| {
                p.y_rot = f(-0.25 + (t * 0.6 + 2.0).sin() * 0.45);
                p.x_rot = f((t * 0.9 + 1.0).sin() * 0.12);
                p.z_rot = -0.2;
            });
            set(model, &["shoulders", "left_head"], |p| {
                p.y_rot = f(0.25 + (t * 0.55 + 4.0).sin() * 0.45);
                p.x_rot = f((t * 0.8 + 3.0).sin() * 0.12);
                p.z_rot = 0.2;
            });
            set(model, &["shoulders", "spine"], |p| p.x_rot = f(0.3 + (t * 1.2 - 0.5).sin() * 0.1));
            set(model, &["shoulders", "spine", "tail"], |p| {
                p.x_rot = f(0.25 + (t * 1.2 - 1.3).sin() * 0.2);
                p.z_rot = f((t * 0.9).sin() * 0.15);
            });
            set(model, &["shoulders", "spine", "tail", "tail_end"], |p| {
                p.x_rot = f(0.3 + (t * 1.2 - 2.1).sin() * 0.3);
                p.z_rot = f((t * 0.9 - 0.8).sin() * 0.25);
            });
            // the nether star (glow model only) turns over and over
            set(model, &["shoulders", "spine", "star"], |p| {
                p.y_rot = f(t * 2.2);
                p.x_rot = f(t * 1.3);
            });
        }
        ("pet", "phantom") => {
            let flap = f((t * 2.6).sin() * 0.45);
            let tip = f((t * 2.6 - 0.7).sin() * 0.35);
            set(model, &["body", "right_wing"], |p| p.z_rot = flap);
            set(model, &["body", "left_wing"], |p| p.z_rot = -flap);
            set(model, &["body", "right_wing", "tip"], |p| p.z_rot = tip);
            set(model, &["body", "left_wing", "tip"], |p| p.z_rot = -tip);
            set(model, &["body", "tail"], |p| p.x_rot = f(-0.05 + (t * 2.6 - 1.2).sin() * 0.12));
            set(model, &["body", "tail", "tail_end"], |p| p.x_rot = f((t * 2.6 - 2.0).sin() * 0.2));
        }
        ("pet", "ghast") => {
            let bob = (t * 1.1).sin();
            for k in 0..9 {
                let kf = k as f64;
                set(model, &[format!("tentacle{k}").as_str()], |p| {
                    p.x_rot = f(0.12 + (t * 1.7 + kf * 0.9).sin() * 0.2 - bob * 0.08);
                    p.z_rot = f((t * 1.3 + kf * 1.7).sin() * 0.12);
                });
            }
            // every 11 seconds a tear slides down the cheek, then falls
            let p = t % 11.0;
            set(model, &["tear"], |tear| {
                tear.visible = p < 2.2;
                if tear.visible {
                    let y = if p < 1.4 { -0.5 + p / 1.4 * 4.5 } else { 4.0 + 12.0 * (p - 1.4) * (p - 1.4) };
                    tear.x = -2.5;
                    tear.y = f(y);
                    tear.z = f(if p < 1.4 { -4.4 } else { -4.4 - (p - 1.4) * 0.8 });
                }
            });
        }
        ("pet", "creeper_buddy") => {
            // front-right and back-left step with your legs, then the other pair
            let step = creeper_step(ctx);
            for leg in 0..4 {
                set(model, &[format!("leg{leg}").as_str()], |p| p.x_rot = if leg == 0 || leg == 3 { step } else { -step });
            }
            // the head looks around slowly; while it fuses it turns up to look at you
            let (fuse, _) = creeper_fuse(t);
            let look = (t * 0.4).sin() * 0.6 + (t * 1.1).sin() * 0.2;
            set(model, &["head"], |p| {
                p.y_rot = f(if fuse { -0.7 } else { look });
                p.x_rot = f(if fuse { -0.35 } else { (t * 0.9).sin() * 0.05 });
            });
        }
        ("pet", "fox_spirit") => {
            let breath = (t * 1.4).sin();
            set(model, &["head"], |p| p.x_rot = f(0.08 + breath * 0.03));
            set(model, &["head", "right_ear"], |p| {
                p.z_rot = f(-0.15 + (t * 0.83).sin().max(0.0).powi(30) * 0.5 * (t * 40.0).sin());
            });
            set(model, &["head", "left_ear"], |p| {
                p.z_rot = f(0.15 - (t * 0.61 + 2.0).sin().max(0.0).powi(30) * 0.5 * (t * 40.0).sin());
            });
            // the tail sways, the sway running down to the white tip
            let mut seg = model.get_child_mut("tail");
            let mut k = 0;
            while let Some(s) = seg {
                let kf = k as f64;
                s.y_rot = f((if k == 0 { 1.3 } else { 0.75 }) + (t * 0.9 - kf * 0.6).sin() * (0.04 + 0.03 * kf));
                k += 1;
                seg = s.get_child_mut("tail");
            }
        }
        ("pet", "bat") => {
            let flap = f((t * 16.0).sin() * 0.75);
            let tip = f((t * 16.0 - 0.6).sin() * 0.5);
            set(model, &["right_wing"], |p| p.z_rot = flap);
            set(model, &["left_wing"], |p| p.z_rot = -flap);
            set(model, &["right_wing", "tip"], |p| p.z_rot = tip);
            set(model, &["left_wing", "tip"], |p| p.z_rot = -tip);
        }
        ("pet", "reindeer") => {
            let stride = t * 6.5;
            let look = ((t * 0.5).sin() + 0.5 * (t * 1.3).sin()).clamp(-1.0, 1.0);
            // head angles also turn the nose model
            set(model, &["head"], |p| {
                p.y_rot = f(look * 0.5);
                p.x_rot = f((stride + 0.6).sin() * 0.08);
            });
            set(model, &["head", "right_ear"], |p| {
                p.z_rot = f(-0.35 + (t * 0.9).sin().max(0.0).powi(30) * 0.6 * (t * 35.0).sin());
            });
            set(model, &["head", "left_ear"], |p| {
                p.z_rot = f(0.35 - (t * 0.7 + 2.0).sin().max(0.0).powi(30) * 0.6 * (t * 35.0).sin());
            });
            set(model, &["front_right_leg"], |p| p.x_rot = f(stride.sin() * 0.7));
            set(model, &["front_left_leg"], |p| p.x_rot = f((stride + 0.5).sin() * 0.7));
            set(model, &["back_right_leg"], |p| p.x_rot = f((stride + 2.5).sin() * 0.7));
            set(model, &["back_left_leg"], |p| p.x_rot = f((stride + 3.0).sin() * 0.7));
            set(model, &["tail"], |p| p.y_rot = f((t * 9.0).sin() * 0.4));
        }
        ("pet", "love_bird") => {
            // every few seconds it glides a moment with its wings spread
            let glide = (t * 1.1).sin().max(0.0).powi(6);
            let flap = f((1.0 - glide) * (t * 18.0).sin() * 0.8 - glide * 0.1);
            let tip = f((1.0 - glide) * (t * 18.0 - 0.7).sin() * 0.5);
            set(model, &["right_wing"], |p| p.z_rot = flap);
            set(model, &["left_wing"], |p| p.z_rot = -flap);
            set(model, &["right_wing", "tip"], |p| p.z_rot = tip);
            set(model, &["left_wing", "tip"], |p| p.z_rot = -tip);
            set(model, &["head"], |p| {
                p.y_rot = f(-0.45 + (t * 0.7).sin() * 0.15);
                p.z_rot = f((t * 1.3).sin() * 0.2);
            });
            set(model, &["tail"], |p| p.x_rot = f(-0.15 + (t * 18.0).sin() * 0.06 * (1.0 - glide)));
        }
        ("pet", "axolotl") => {
            let wag = f((t * 5.0).sin() * 0.45);
            set(model, &["tail"], |p| p.y_rot = wag);
            set(model, &["head"], |p| p.y_rot = -wag * 0.4);
            set(model, &["head", "right_gill"], |p| p.y_rot = f(-0.6 + (t * 3.0).sin() * 0.25));
            set(model, &["head", "left_gill"], |p| p.y_rot = f(0.6 - (t * 3.0).sin() * 0.25));
            for k in 0..4 {
                let kf = k as f64;
                set(model, &[format!("leg{k}").as_str()], |p| p.x_rot = f(0.7 + (t * 5.0 + kf * 1.6).sin() * 0.4));
            }
        }
        ("pet", "mini_golem") => {
            // walks in step with you
            let step = left_leg_x_rot(ctx).clamp(-0.9, 0.9) * 1.2;
            set(model, &["right_leg"], |p| p.x_rot = step);
            set(model, &["left_leg"], |p| p.x_rot = -step);
            set(model, &["right_arm"], |p| p.x_rot = -step * 0.7);
            set(model, &["left_arm"], |p| p.x_rot = step * 0.7);
            set(model, &["head"], |p| p.y_rot = f((t * 0.4).sin() * 0.5 + (t * 1.1).sin() * 0.15));
        }
        ("pet", "crab") => {
            let busy = crab_busy(t, ctx);
            for s in [-1.0f64, 1.0] {
                let side = if s < 0.0 { "right" } else { "left" };
                for k in 0..3 {
                    let kf = k as f64;
                    // the legs tick in turns, a side half a step after the other
                    set(model, &[format!("{side}_leg{k}").as_str()], |p| {
                        p.z_rot = f(-s * (0.5 + (t * 16.0 + kf * 2.1 + (s + 1.0) * 0.8).sin() * 0.3 * busy));
                    });
                }
                let open = (t * 1.7 + (s + 1.0) * 1.3).sin().max(0.0).powi(6);
                set(model, &[format!("{side}_claw").as_str()], |p| p.x_rot = f(-open * 0.45));
                set(model, &[format!("{side}_claw").as_str(), "pincer", "jaw"], |p| {
                    p.x_rot = f(-open * 0.7 * (0.6 + 0.4 * (t * 9.0).sin().abs()));
                });
                set(model, &[format!("{side}_eye").as_str()], |p| {
                    p.y_rot = f((t * 0.6 + s * 1.3).sin() * 0.5);
                    p.z_rot = f(s * 0.15 + (t * 0.8 + s).sin() * 0.08);
                });
            }
        }
        _ => {}
    }
}
