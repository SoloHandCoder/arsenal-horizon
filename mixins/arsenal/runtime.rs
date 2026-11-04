// Arsenal on Horizon: Arsenal's PvP HUD panel (FPS, clicks per second, coordinates, facing), its
// zoom, smaller held items, the third-person camera's distance and clearer fog, as a native mod the
// Arsenal Launcher builds into Horizon. Its settings are one file per key in the mod's data folder
// (`native_setting`). This file is both the mod's Cargo library (its tests) and the module the
// bundle adds to the game, as `game::arsenal_native`.

use std::cell::RefCell;

/// The mod's id (its settings folder and its key's line), name and version.
pub const ID: &str = "arsenal";
pub const NAME: &str = "Arsenal";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Arsenal's PvP HUD (FPS, clicks per second, coordinates, facing), zoom, smaller held items, third-person distance and clearer fog. Built in by the Arsenal Launcher.";

/// The zoom key's name, and its default (Arsenal's zoom key on Fabric).
pub const ZOOM_KEY: &str = "zoom";
pub const DEFAULT_ZOOM_KEY: &str = "key.keyboard.c";

pub const CORNERS: [&str; 4] = ["Top left", "Top right", "Bottom left", "Bottom right"];
/// The panel's accent: its stripe and its labels.
pub const ACCENTS: [(&str, u32); 6] = [
    ("Gold", 0xF5B83D),
    ("Red", 0xFF5A5A),
    ("Blue", 0x5AA9FF),
    ("Green", 0x5CE08A),
    ("Purple", 0xB884FF),
    ("White", 0xFFFFFF),
];
/// The zooms the key starts at, and the wheel's limits while it is held.
pub const ZOOMS: [u8; 5] = [2, 3, 4, 6, 8];
const ZOOM_MIN: f32 = 1.25;
const ZOOM_MAX: f32 = 10.0;
/// Each wheel step multiplies the zoom by this.
const ZOOM_STEP: f32 = 1.25;
/// The held item's size, in percent.
pub const ITEM_SIZES: [u8; 6] = [100, 90, 80, 70, 60, 50];
/// The third-person camera's distance in blocks; vanilla's is 4.
pub const CAMERA: [u8; 5] = [4, 2, 3, 6, 8];
pub const FOGS: [&str; 3] = ["Normal", "Less", "None"];

/// The panel's background, as the HUD's own boxes are drawn.
pub const BACKGROUND: u32 = 0x9010_1014;
/// Values are white.
pub const VALUE: u32 = 0xFFFFFF;
const PAD: f32 = 4.0;
const STRIPE: f32 = 2.0;
const LINE: f32 = 10.0;
const GAP: f32 = 5.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub hud: bool,
    /// One of `CORNERS`.
    pub corner: usize,
    pub fps: bool,
    pub cps: bool,
    pub coords: bool,
    pub facing: bool,
    /// One of `ACCENTS`.
    pub accent: usize,
    /// One of `ZOOMS`.
    pub zoom: usize,
    pub smooth: bool,
    /// One of `ITEM_SIZES`.
    pub item: usize,
    /// One of `CAMERA`.
    pub camera: usize,
    /// One of `FOGS`.
    pub fog: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            hud: true,
            corner: 0,
            fps: true,
            cps: true,
            coords: true,
            facing: true,
            accent: 0,
            zoom: 2,
            smooth: true,
            item: 0,
            camera: 0,
            fog: 0,
        }
    }
}

/// The index of `value` in `names`, else `fallback`.
fn named(value: Option<String>, names: &[&str], fallback: usize) -> usize {
    value
        .and_then(|v| names.iter().position(|n| n.eq_ignore_ascii_case(v.trim())))
        .unwrap_or(fallback)
}

/// The index of the number `value` in `numbers`, else `fallback`.
fn number(value: Option<String>, numbers: &[u8], fallback: usize) -> usize {
    value
        .and_then(|v| v.trim().parse::<u8>().ok())
        .and_then(|v| numbers.iter().position(|&n| n == v))
        .unwrap_or(fallback)
}

impl Settings {
    /// From the stored values (`get` reads one key's file); the defaults for the ones missing or
    /// unreadable.
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        let flag = |key: &str, default: bool| get(key).map_or(default, |v| v.trim() != "false");
        let accents: Vec<&str> = ACCENTS.iter().map(|a| a.0).collect();
        Self {
            hud: flag("hud", d.hud),
            corner: named(get("corner"), &CORNERS, d.corner),
            fps: flag("fps", d.fps),
            cps: flag("cps", d.cps),
            coords: flag("coords", d.coords),
            facing: flag("facing", d.facing),
            accent: named(get("accent"), &accents, d.accent),
            zoom: number(get("zoom"), &ZOOMS, d.zoom),
            smooth: flag("smooth", d.smooth),
            item: number(get("item"), &ITEM_SIZES, d.item),
            camera: number(get("camera"), &CAMERA, d.camera),
            fog: named(get("fog"), &FOGS, d.fog),
        }
    }

    /// The settings page's buttons.
    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        let camera = match CAMERA[self.camera] {
            4 => "4 blocks (vanilla)".to_owned(),
            n => format!("{n} blocks"),
        };
        vec![
            format!("HUD: {}", on(self.hud)),
            format!("HUD corner: {}", CORNERS[self.corner]),
            format!("FPS: {}", on(self.fps)),
            format!("Clicks per second: {}", on(self.cps)),
            format!("Coordinates: {}", on(self.coords)),
            format!("Facing: {}", on(self.facing)),
            format!("Accent: {}", ACCENTS[self.accent].0),
            format!("Zoom: {}x", ZOOMS[self.zoom]),
            format!("Smooth zoom: {}", on(self.smooth)),
            format!("Held item size: {}%", ITEM_SIZES[self.item]),
            format!("Third-person distance: {camera}"),
            format!("Fog: {}", FOGS[self.fog]),
        ]
    }

    /// A click on button `i` of `labels`.
    pub fn click(&mut self, i: usize) {
        let next = |at: &mut usize, len: usize| *at = (*at + 1) % len;
        match i {
            0 => self.hud = !self.hud,
            1 => next(&mut self.corner, CORNERS.len()),
            2 => self.fps = !self.fps,
            3 => self.cps = !self.cps,
            4 => self.coords = !self.coords,
            5 => self.facing = !self.facing,
            6 => next(&mut self.accent, ACCENTS.len()),
            7 => next(&mut self.zoom, ZOOMS.len()),
            8 => self.smooth = !self.smooth,
            9 => next(&mut self.item, ITEM_SIZES.len()),
            10 => next(&mut self.camera, CAMERA.len()),
            11 => next(&mut self.fog, FOGS.len()),
            _ => {}
        }
    }

    /// What to store, key by key.
    pub fn stored(&self) -> Vec<(&'static str, String)> {
        let flag = |b: bool| if b { "true" } else { "false" }.to_owned();
        vec![
            ("hud", flag(self.hud)),
            ("corner", CORNERS[self.corner].to_owned()),
            ("fps", flag(self.fps)),
            ("cps", flag(self.cps)),
            ("coords", flag(self.coords)),
            ("facing", flag(self.facing)),
            ("accent", ACCENTS[self.accent].0.to_owned()),
            ("zoom", ZOOMS[self.zoom].to_string()),
            ("smooth", flag(self.smooth)),
            ("item", ITEM_SIZES[self.item].to_string()),
            ("camera", CAMERA[self.camera].to_string()),
            ("fog", FOGS[self.fog].to_owned()),
        ]
    }

    /// The held item's scale.
    pub fn item_scale(&self) -> f32 {
        f32::from(ITEM_SIZES[self.item]) / 100.0
    }

    /// The third-person camera's distance for vanilla's `distance` (4 blocks, scaled by the
    /// player's size): scaled the same way.
    pub fn camera_distance(&self, distance: f32) -> f32 {
        distance * f32::from(CAMERA[self.camera]) / 4.0
    }

    /// The environmental fog's start and end for vanilla's, pushed away by `fog`. Only ever
    /// further, and end stays past start, as Horizon requires.
    pub fn fog_range(&self, start: f32, end: f32) -> (f32, f32) {
        match self.fog {
            1 => (start * 2.0, (end * 2.0).max(start * 2.0 + 1.0)),
            2 => (1.0e6, 1.0e6 + 1.0),
            _ => (start, end),
        }
    }
}

#[derive(Default)]
struct State {
    /// Read once, then kept; the settings page replaces it.
    settings: Option<Settings>,
    /// The zoom while the key is held; None when it is up.
    held: Option<f32>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

/// The settings, read once through `get`.
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    STATE.with_borrow_mut(|s| *s.settings.get_or_insert_with(|| Settings::read(get)))
}

/// Replaces the settings (the settings page changed them).
pub fn set_settings(settings: Settings) {
    STATE.with_borrow_mut(|s| s.settings = Some(settings));
}

/// The zoom key went down (only while playing) or up.
pub fn zoom_key(playing: bool, pressed: bool, get: impl Fn(&str) -> Option<String>) {
    if pressed && !playing {
        return;
    }
    let level = ZOOMS[settings(get).zoom];
    STATE.with_borrow_mut(|s| s.held = pressed.then(|| f32::from(level)));
}

/// Wheel steps: true when the zoom key is held and they zoom.
pub fn wheel(steps: f32) -> bool {
    STATE.with_borrow_mut(|s| {
        let Some(z) = &mut s.held else {
            return false;
        };
        *z = (*z * ZOOM_STEP.powf(steps)).clamp(ZOOM_MIN, ZOOM_MAX);
        true
    })
}

/// The zoom in effect: (factor, smooth, turning scale), none when the key is up. Turning slows
/// with the zoom, so aim keeps its feel.
pub fn view() -> Option<(f32, bool, f32)> {
    STATE.with_borrow(|s| {
        let factor = s.held?;
        let smooth = s.settings.unwrap_or_default().smooth;
        Some((factor, smooth, 1.0 / factor))
    })
}

/// What the panel shows this frame: the readings its lines need.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Readings {
    pub fps: u32,
    /// Attack and use clicks in the last second.
    pub cps: (u32, u32),
    /// The player's position and yaw, in a world.
    pub player: Option<([f64; 3], f32)>,
}

/// The compass point for `yaw` (0 faces south, 90 west), with the axis it walks along.
pub fn facing(yaw: f32) -> &'static str {
    match ((yaw / 90.0).round() as i32).rem_euclid(4) {
        0 => "South +Z",
        1 => "West -X",
        2 => "North -Z",
        _ => "East +X",
    }
}

/// The panel's lines, label and value, as `settings` picks them.
pub fn lines(settings: &Settings, r: &Readings) -> Vec<(&'static str, String)> {
    let mut out = Vec::new();
    if settings.fps {
        out.push(("FPS", r.fps.to_string()));
    }
    if settings.cps {
        out.push(("CPS", format!("{} | {}", r.cps.0, r.cps.1)));
    }
    if let Some(([x, y, z], yaw)) = r.player {
        if settings.coords {
            let b = |v: f64| v.floor() as i64;
            out.push(("XYZ", format!("{} {} {}", b(x), b(y), b(z))));
        }
        if settings.facing {
            out.push(("Facing", facing(yaw).to_owned()));
        }
    }
    out
}

/// One thing to draw, relative to the panel's top left.
#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Fill([f32; 4], u32),
    Text(String, [f32; 2], u32, bool),
}

/// The panel for `lines` in `accent` (0xRRGGBB), measured with `width`: its shapes and its size.
/// None when there is nothing to show.
pub fn panel(
    lines: &[(&str, String)],
    accent: u32,
    width: &dyn Fn(&str) -> f32,
) -> Option<(Vec<Shape>, f32, f32)> {
    if lines.is_empty() {
        return None;
    }
    let labels = lines.iter().map(|(l, _)| width(l)).fold(0.0, f32::max);
    let values = lines.iter().map(|(_, v)| width(v)).fold(0.0, f32::max);
    let w = STRIPE + PAD + labels + GAP + values + PAD;
    let h = PAD * 2.0 + lines.len() as f32 * LINE - 2.0;
    let mut out = vec![
        Shape::Fill([0.0, 0.0, w, h], BACKGROUND),
        Shape::Fill([0.0, 0.0, STRIPE, h], 0xFF00_0000 | accent),
    ];
    for (i, (label, value)) in lines.iter().enumerate() {
        let y = PAD + i as f32 * LINE;
        out.push(Shape::Text((*label).to_owned(), [STRIPE + PAD, y], accent, true));
        out.push(Shape::Text(value.clone(), [STRIPE + PAD + labels + GAP, y], VALUE, true));
    }
    Some((out, w, h))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stored<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        |key| pairs.iter().find(|p| p.0 == key).map(|p| p.1.to_owned())
    }

    #[test]
    fn settings_round_trip_and_fall_back_to_defaults() {
        assert_eq!(Settings::read(stored(&[])), Settings::default());
        let mut s = Settings::read(stored(&[("corner", "bottom right"), ("zoom", "6\n"), ("fog", "nope"), ("item", "55")]));
        assert_eq!((s.corner, ZOOMS[s.zoom], s.fog, s.item), (3, 6, 0, 0));
        s.click(1);
        s.click(11);
        s.click(0);
        let back: Vec<(&str, String)> = s.stored();
        let pairs: Vec<(&str, &str)> = back.iter().map(|(k, v)| (*k, v.as_str())).collect();
        assert_eq!(Settings::read(stored(&pairs)), s);
        assert_eq!(s.labels()[0], "HUD: OFF");
        assert_eq!(s.labels()[1], "HUD corner: Top left");
        assert_eq!(s.labels()[11], "Fog: Less");
        assert_eq!(s.labels().len(), s.stored().len());
    }

    #[test]
    fn camera_item_and_fog_only_change_when_asked() {
        let mut s = Settings::default();
        assert_eq!(s.camera_distance(4.0), 4.0);
        assert_eq!(s.item_scale(), 1.0);
        assert_eq!(s.fog_range(10.0, 20.0), (10.0, 20.0));
        s.click(10);
        assert_eq!(s.camera_distance(4.0), 2.0);
        s.item = 2;
        assert!((s.item_scale() - 0.8).abs() < 1e-6);
        s.fog = 1;
        assert_eq!(s.fog_range(10.0, 20.0), (20.0, 40.0));
        s.fog = 2;
        let (a, b) = s.fog_range(10.0, 20.0);
        assert!(b > a && a > 1000.0);
    }

    #[test]
    fn facing_follows_minecrafts_yaw() {
        assert_eq!(facing(0.0), "South +Z");
        assert_eq!(facing(90.0), "West -X");
        assert_eq!(facing(-180.0), "North -Z");
        assert_eq!(facing(-90.0), "East +X");
        assert_eq!(facing(359.0), "South +Z");
    }

    #[test]
    fn the_panel_lines_up_labels_and_values() {
        let r = Readings {
            fps: 144,
            cps: (7, 2),
            player: Some(([10.7, 64.0, -3.2], 180.0)),
        };
        let s = Settings::default();
        let l = lines(&s, &r);
        assert_eq!(l[2], ("XYZ", "10 64 -4".to_owned()));
        assert_eq!(l[3], ("Facing", "North -Z".to_owned()));
        let width = |t: &str| t.len() as f32 * 6.0;
        let (shapes, w, h) = panel(&l, 0xF5B83D, &width).unwrap();
        // Widest label "Facing" (36), widest value "North -Z" (48).
        assert_eq!(w, 2.0 + 4.0 + 36.0 + 5.0 + 48.0 + 4.0);
        assert_eq!(h, 8.0 + 40.0 - 2.0);
        assert_eq!(shapes[1], Shape::Fill([0.0, 0.0, 2.0, h], 0xFFF5B83D));
        assert!(shapes.contains(&Shape::Text("144".into(), [47.0, 4.0], VALUE, true)));
        // Out of a world: no coordinates or facing; nothing on: no panel.
        assert_eq!(lines(&s, &Readings::default()).len(), 2);
        let none = Settings { fps: false, cps: false, ..s };
        assert!(panel(&lines(&none, &Readings::default()), 0, &width).is_none());
    }

    #[test]
    fn the_wheel_zooms_only_while_the_key_is_held() {
        set_settings(Settings::default());
        assert!(!wheel(1.0));
        zoom_key(false, true, |_| None);
        assert_eq!(view(), None);
        zoom_key(true, true, |_| None);
        assert_eq!(view(), Some((4.0, true, 0.25)));
        assert!(wheel(1.0));
        assert_eq!(view().unwrap().0, 5.0);
        for _ in 0..20 {
            wheel(1.0);
        }
        assert_eq!(view().unwrap().0, ZOOM_MAX);
        zoom_key(true, false, |_| None);
        assert_eq!(view(), None);
    }
}
