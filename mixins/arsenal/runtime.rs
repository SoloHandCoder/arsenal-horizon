// Arsenal on Horizon: the Arsenal PvP mod as a native mod. Right Shift opens Arsenal's menu with every module the
// Java mod has, in its categories; the ones Horizon can run here switch on and off and have their settings, the others
// show greyed out ("Java only"). Settings are one file per key in the mod's data folder (`native_setting`), and the
// same settings show in Options > Mods > Arsenal. This file is both the mod's Cargo library (its tests) and the module
// the bundle adds to the game, as `game::arsenal_native`.

use std::cell::RefCell;

/// The mod's id (its settings folder and its keys' lines), name and version.
pub const ID: &str = "arsenal";
pub const NAME: &str = "Arsenal";
pub const VERSION: &str = "1.1.0";
pub const DESCRIPTION: &str = "Arsenal's PvP mod on Horizon. Right Shift opens Arsenal's menu: Info HUD, coordinates, compass, HUD theme, zoom, hand view model and camera tweaks, with every other Arsenal module listed.";

/// The keys' names and defaults: the menu (Right Shift, as in the Java mod) and zoom (C).
pub const MENU_KEY: &str = "menu";
pub const DEFAULT_MENU_KEY: &str = "key.keyboard.right.shift";
pub const ZOOM_KEY: &str = "zoom";
pub const DEFAULT_ZOOM_KEY: &str = "key.keyboard.c";

/// The Java mod's categories, in its order.
pub const CATEGORIES: [&str; 9] = ["HUD", "Combat", "Enemies", "Crystal & Anchor", "Mace", "World", "Hands & Camera", "Chat & Social", "Utility"];

/// One module of the Java mod (the list below is copied from it by scripts/java_modules.py).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JavaModule {
    pub id: &'static str,
    pub name: &'static str,
    /// Index in `CATEGORIES`.
    pub category: usize,
    pub description: &'static str,
}

// BEGIN JAVA MODULES
pub const JAVA_MODULES: &[JavaModule] = &[
    JavaModule { id: "armor_hud", name: "Armor HUD", category: 0, description: "Your armor next to the hotbar with durability bars and numbers." },
    JavaModule { id: "attack_bar", name: "Attack Charge Bar", category: 0, description: "Slim attack charge bar under the crosshair, green when full." },
    JavaModule { id: "boss_bar_tweaks", name: "Boss Bar Tweaks", category: 0, description: "Hide boss bars, make them smaller, move them down or limit how many show at once." },
    JavaModule { id: "bow_charge", name: "Bow Charge %", category: 0, description: "Bow, crossbow and trident charge in percent while drawing." },
    JavaModule { id: "clock_info", name: "Clock & Server Info", category: 0, description: "Real time, server address and estimated server TPS." },
    JavaModule { id: "combo_reach", name: "Combo & Reach", category: 0, description: "Hits in a row without being hit, and the reach of your last hit." },
    JavaModule { id: "compass", name: "Compass", category: 0, description: "Compass bar with N/E/S/W and your exact heading." },
    JavaModule { id: "cooldown_hud", name: "Cooldown HUD", category: 0, description: "Cooldown bars for pearls, wind charges, chorus fruit, shields..." },
    JavaModule { id: "coordinates", name: "Coordinates", category: 0, description: "XYZ, facing, biome and Nether/Overworld coordinates. Tells you in chat where you died." },
    JavaModule { id: "custom_crosshair", name: "Crosshair Editor", category: 0, description: "Draw your own crosshair pixel by pixel (Open Editor). Shared with setup codes." },
    JavaModule { id: "fight_timer", name: "Fight Timer", category: 0, description: "How long the current fight has lasted." },
    JavaModule { id: "fps_graph", name: "FPS & Ping Graph", category: 0, description: "A small live graph of your FPS and ping, so lag spikes are easy to spot." },
    JavaModule { id: "hotbar_style", name: "Hotbar Style", category: 0, description: "The hotbar in your HUD Theme's look: themed background, accent selected slot (outline, filled or underline) and optional slot numbers." },
    JavaModule { id: "hud_theme", name: "HUD Theme", category: 0, description: "One look for every Arsenal HUD: 20 themes (Glass, Neon, Midnight, Sunset...) plus your own background, gradient, border, corners, padding, colours and rainbow." },
    JavaModule { id: "info_hud", name: "Info HUD", category: 0, description: "Keystrokes, clicks per second, FPS, ping and coordinates." },
    JavaModule { id: "inventory_style", name: "Inventory Style", category: 0, description: "Your inventory, the creative inventory, chests, ender chests, barrels and shulker boxes in your HUD Theme's look: themed panel, tabs, slot boxes and titles." },
    JavaModule { id: "inventory_totem", name: "Inventory Totem", category: 0, description: "Totem icon above the hotbar with how many totems you carry." },
    JavaModule { id: "item_counter", name: "Item Counter", category: 0, description: "Totems, gapples, pearls, crystals, XP bottles and more you have left, in one strip." },
    JavaModule { id: "lag_meter", name: "Lag Meter", category: 0, description: "Estimated server TPS and your ping, with a warning when the server lags or your ping spikes." },
    JavaModule { id: "notification_feed", name: "Notification Feed", category: 0, description: "The last few fight events (pops, kills, shield breaks, pearls) at the top right." },
    JavaModule { id: "potion_timers", name: "Potion Timers", category: 0, description: "Your active effects with level and time left." },
    JavaModule { id: "ready_check", name: "Ready Check", category: 0, description: "A small checklist: offhand totem, full armour with durability left, golden apples, pearls, and crystal gear on crystal servers. Turns green when you're ready; can show only what's missing." },
    JavaModule { id: "scoreboard_tweaks", name: "Scoreboard Tweaks", category: 0, description: "Hide the sidebar's red numbers or the whole sidebar, make it smaller or move it." },
    JavaModule { id: "session_clock", name: "Session Clock", category: 0, description: "How long you have played since joining, the real time if you like, and an optional break reminder." },
    JavaModule { id: "session_goals", name: "Session Goals", category: 0, description: "Set goals for this session (kills, K/D, playtime, practice wins, totem pops survived) and watch the progress bars fill; a toast when you reach one. Starts over every session." },
    JavaModule { id: "session_stats", name: "Session Stats", category: 0, description: "Kills, deaths, K/D and totems popped this session." },
    JavaModule { id: "speed_display", name: "Speed Display", category: 0, description: "Your speed in blocks per second." },
    JavaModule { id: "tab_ping", name: "Tab List Ping", category: 0, description: "Ping as coloured numbers in the tab list." },
    JavaModule { id: "target_hud", name: "Target HUD", category: 0, description: "Panel for the player you are fighting: head, health, armor, item, pops. Its look follows HUD Theme." },
    JavaModule { id: "title_tweaks", name: "Title Tweaks", category: 0, description: "Hide, scale or move the big title and subtitle and the action bar text, and make titles go away sooner." },
    JavaModule { id: "auto_hitbox", name: "Auto Hitbox", category: 1, description: "Only show hitboxes while holding a weapon (never with crystals)." },
    JavaModule { id: "combat_hitboxes", name: "Combat Hitboxes", category: 1, description: "Player hitboxes; the one you aim at turns red when a full hit would land." },
    JavaModule { id: "combat_tag_timer", name: "Combat Tag Timer", category: 1, description: "Reads the server's own \"you are in combat\" messages (chat, action bar, titles) and counts down until you may log out, with a sound when you're safe. The messages it looks for can be changed." },
    JavaModule { id: "combo_pop", name: "Combo Counter Pop", category: 1, description: "Your combo pops up next to the crosshair and bumps bigger with every hit; it ends when you get hit." },
    JavaModule { id: "crit_indicator", name: "Crit Indicator", category: 1, description: "Gold star next to the crosshair when your next hit is a critical hit." },
    JavaModule { id: "damage_numbers", name: "Damage Numbers", category: 1, description: "Floating number showing how much health your hit took off." },
    JavaModule { id: "death_recap", name: "Death Recap", category: 1, description: "After you die: damage taken in the last 10 s, who hit you last, with what, and their pops. In chat and on the death screen." },
    JavaModule { id: "durability_warning", name: "Durability Warning", category: 1, description: "Sound and flashing piece when your armor is about to break." },
    JavaModule { id: "dynamic_crosshair", name: "Dynamic Crosshair", category: 1, description: "Crosshair color shows when a hit lands; optional dot/cross/circle shapes." },
    JavaModule { id: "fight_summary", name: "Fight Summary", category: 1, description: "After a kill or death: hits, accuracy, damage dealt and taken, combo and pops. Plus your win/loss record per player this session." },
    JavaModule { id: "hitmarker", name: "Hitmarker", category: 1, description: "A small X flashes around the crosshair when your hit really lands. Optional sound." },
    JavaModule { id: "kill_streaks", name: "Kill Streak Sounds", category: 1, description: "Double Kill, Triple Kill... with a sound for kills in quick succession." },
    JavaModule { id: "low_health_warning", name: "Low Health Warning", category: 1, description: "Red pulsing screen edges and a heartbeat when your health is low." },
    JavaModule { id: "offhand_totem_warning", name: "Offhand Totem Warning", category: 1, description: "Warning when your offhand has no totem but you still carry some." },
    JavaModule { id: "potion_warning", name: "Potion Run-out Warning", category: 1, description: "A sound and a warning a few seconds before Strength, Speed or another good effect runs out." },
    JavaModule { id: "retotem_timer", name: "Re-totem Timer", category: 1, description: "After your totem pops: how fast a new totem was in your offhand (\"Re-totem 0.35 s\"), with your session average and best. Only measures, never moves items." },
    JavaModule { id: "shield_break_sound", name: "Shield Break Sound", category: 1, description: "Sound and \"Shield broken!\" when you disable the shield of a player you hit." },
    JavaModule { id: "shield_helper", name: "Shield Helper", category: 1, description: "Enemy shields are red while disabled by an axe, green when usable." },
    JavaModule { id: "sprint_indicator", name: "Sprint Reset Indicator", category: 1, description: "Green bolt when your next hit gives sprint knockback." },
    JavaModule { id: "tnt_timer", name: "TNT Timer", category: 1, description: "Shows how long until lit TNT blows up, above the TNT: green, yellow, then red." },
    JavaModule { id: "enemy_armor_alert", name: "Enemy Armor Alert", category: 2, description: "Red warning when the player you fight has almost broken armor." },
    JavaModule { id: "eating_warning", name: "Enemy Eating Warning", category: 2, description: "EATING GAPPLE / EATING / DRINKING above enemies while they eat or drink." },
    JavaModule { id: "pearl_timer", name: "Enemy Pearl Timer", category: 2, description: "Purple countdown above players who just threw an ender pearl." },
    JavaModule { id: "nametag_health", name: "Health on Nametags", category: 2, description: "Health and absorption after each player's name." },
    JavaModule { id: "opponent_intel", name: "Opponent Intel", category: 2, description: "During a fight, a card with what your game saw the player you fight use: totems popped, golden apples eaten, pearls and wind charges thrown, armour worn down or broken. Starts over every fight; uses Fight Summary's fight tracking." },
    JavaModule { id: "nametag_ping", name: "Ping on Nametags", category: 2, description: "Each player's ping after their name." },
    JavaModule { id: "target_low_hp", name: "Target Low HP Alert", category: 2, description: "One ding and a message when the player you fight drops under a health threshold." },
    JavaModule { id: "totem_pop_alerts", name: "Totem Pop Alerts", category: 2, description: "Chat line and sound when someone pops a totem." },
    JavaModule { id: "totem_pop_counter", name: "Totem Pop Counter", category: 2, description: "-N after a player's name for every totem they popped. Starts over when they die, respawn or you beat them; reset all with F10." },
    JavaModule { id: "anchor_helper", name: "Anchor Helper", category: 3, description: "Respawn anchor charges, and its blast damage to you vs the nearest enemy." },
    JavaModule { id: "crystal_damage_preview", name: "Crystal Damage Preview", category: 3, description: "Damage to you vs the nearest enemy on the crystal you look at or are about to place." },
    JavaModule { id: "crystal_helper", name: "Crystal Helper", category: 3, description: "Only sends crystal placements that work and stops you from badly blowing yourself up." },
    JavaModule { id: "crystal_optimizer", name: "Crystal Optimizer", category: 3, description: "Crystals you hit disappear instantly instead of waiting for the server (they come back if the server keeps them)." },
    JavaModule { id: "crystal_preview", name: "Crystal Preview", category: 3, description: "Outline where a held crystal would go: green if it fits, red if not." },
    JavaModule { id: "obsidian_guard", name: "Obsidian Click Guard", category: 3, description: "Left-clicking the top of obsidian or bedrock does nothing (no packet, no swing), so misclicks while crystalling don't slow you down. Pickaxes mine normally." },
    JavaModule { id: "small_crystals", name: "Smaller Crystals", category: 3, description: "End crystals drawn smaller, optionally without spinning. Hitbox unchanged." },
    JavaModule { id: "fall_preview", name: "Fall Damage Preview", category: 4, description: "While falling: the fall damage you will take on landing (water, slime, hay and beds count)." },
    JavaModule { id: "mace_aim", name: "Mace Aim", category: 4, description: "While falling with a mace: a line to the player below and your bonus damage." },
    JavaModule { id: "mace_optimizer", name: "Mace Optimizer", category: 4, description: "Mace smash sound and particles play instantly." },
    JavaModule { id: "big_items", name: "Big Items", category: 5, description: "Items you pick are drawn much bigger while they lie on the ground, so valuable loot stands out. Add items in its settings." },
    JavaModule { id: "block_outline", name: "Block Outline", category: 5, description: "Custom color and thickness for the block selection outline." },
    JavaModule { id: "item_names", name: "Dropped Item Names", category: 5, description: "Dropped items you can see show their name and count above them; valuable loot is gold and can glow." },
    JavaModule { id: "fullbright", name: "Fullbright", category: 5, description: "See in the dark without night vision." },
    JavaModule { id: "hide_nametags", name: "Hide Nametags", category: 5, description: "Hides the name tags above players' heads, also names servers draw above them. Friends can keep theirs; can be limited to fights." },
    JavaModule { id: "hit_effects", name: "Hit Effects", category: 5, description: "Custom hurt color, crit particles on every hit, optional hit sound." },
    JavaModule { id: "kill_effects", name: "Kill Effects", category: 5, description: "Lightning or totem particles (only for you) when a player you hit dies." },
    JavaModule { id: "low_fire", name: "Low Fire on Ground", category: 5, description: "Fire on the ground is drawn lower so it doesn't block your view." },
    JavaModule { id: "no_fog", name: "No Fog", category: 5, description: "Removes fog at render distance and in water/lava. Blindness and Darkness still work." },
    JavaModule { id: "particle_limiter", name: "Particle Limiter", category: 5, description: "Fewer particles for more FPS. Explosion particles are hidden by default." },
    JavaModule { id: "world_colors", name: "Sky & World Colors", category: 5, description: "Your own sky, fog and cloud colours. Nights still get dark unless you turn off Follow daylight." },
    JavaModule { id: "sound_tweaks", name: "Sound Tweaks", category: 5, description: "Volume for explosions, totem pops, player hits and mace smashes." },
    JavaModule { id: "weather_time", name: "Weather & Time", category: 5, description: "Only on your screen: no rain and a fixed time of day." },
    JavaModule { id: "world_look", name: "World Look", category: 5, description: "Color grading for the world: Dark Blue, Night, Warm, Cold... or your own." },
    JavaModule { id: "old_animations", name: "1.8 Animations", category: 6, description: "1.8 looks on your screen only: the old sword block-hit pose when you hold right-click, and lower held items." },
    JavaModule { id: "armor_color", name: "Armor Color", category: 6, description: "Tints your armor (and optionally friends' or everyone's). Lighter armor like iron shows colours best." },
    JavaModule { id: "camera_tweaks", name: "Camera Tweaks", category: 6, description: "Third-person camera distance, and no camera pushed in by walls." },
    JavaModule { id: "enchant_glint", name: "Enchant Glint", category: 6, description: "Enchant glint colour (red, blue, green, gold, white) and strength." },
    JavaModule { id: "free_look", name: "Free Look", category: 6, description: "Hold Left Alt to look around your character in third person without turning: you keep walking and aiming the same way. Let go to get your view back." },
    JavaModule { id: "hand_tint", name: "Hand Tint & Glow", category: 6, description: "Tints the items in your hands (your HUD Theme colour by default) and can make them glow brighter." },
    JavaModule { id: "view_model", name: "Hand View Model", category: 6, description: "Move, scale and tilt the items in your hands." },
    JavaModule { id: "hide_own_particles", name: "Hide Own Effect Particles", category: 6, description: "Hides the potion swirls coming off you in first person." },
    JavaModule { id: "motion_blur", name: "Motion Blur", category: 6, description: "Smooth blur when you turn or move the camera. Adjustable strength; the HUD stays sharp. Works with World Look." },
    JavaModule { id: "no_hurt_cam", name: "No Hurt-Cam", category: 6, description: "Removes the camera tilt when you take damage." },
    JavaModule { id: "no_pumpkin_overlay", name: "No Pumpkin Overlay", category: 6, description: "Removes the dark carved pumpkin overlay while you wear a pumpkin on your head." },
    JavaModule { id: "smaller_pvp", name: "Smaller PvP", category: 6, description: "Smaller totem animation, first-person shield/totem and fire overlay. Low Shield draws your raised shield lower, like old versions." },
    JavaModule { id: "swing_animation", name: "Swing Animation", category: 6, description: "Faster or slower hand swing (visual only)." },
    JavaModule { id: "totem_pop_style", name: "Totem Pop Style", category: 6, description: "How your totem pops look and sound: animation size, particles, a different sound and a coloured screen flash." },
    JavaModule { id: "zoom", name: "Zoom", category: 6, description: "Hold C to zoom, scroll to zoom further." },
    JavaModule { id: "arsenal_messages", name: "Arsenal Messages", category: 7, description: "Where Arsenal's own messages appear: chat, above the hotbar, small toasts, or only the chat's Arsenal tab. All of them are kept in that tab." },
    JavaModule { id: "arsenal_users", name: "Arsenal Users", category: 7, description: "The Arsenal logo next to players who also use Arsenal. Only a one-way hash of your account id is stored." },
    JavaModule { id: "chat_filter", name: "Chat Filter", category: 7, description: "Hides chat lines containing your words." },
    JavaModule { id: "chat_hotkeys", name: "Chat Hotkeys", category: 7, description: "Up to 8 keys that each send a chat message or command you set (gg, /spawn, /kit pvp). Only while playing, with a short wait so they can't spam." },
    JavaModule { id: "chat_mentions", name: "Chat Mentions", category: 7, description: "Highlights chat lines that mention you, with a soft ping." },
    JavaModule { id: "chat_tweaks", name: "Chat Tweaks", category: 7, description: "Timestamps, repeated messages stacked as (x3), longer chat history." },
    JavaModule { id: "chat_copy", name: "Click to Copy Chat", category: 7, description: "Click a line in chat to copy it without colours; shift-click copies just the sender's name. Links still work as before." },
    JavaModule { id: "discord_presence", name: "Discord Rich Presence", category: 7, description: "Your Discord profile shows you play with Arsenal: the server, the fight you're in and your wins this session. Only talks to the Discord app on your PC." },
    JavaModule { id: "friends", name: "Friends", category: 7, description: "Mark friends (middle mouse on a player or /arsenal friend add): green name, never treated as enemies." },
    JavaModule { id: "mention_ping", name: "Mention Ping", category: 7, description: "A sound, a highlight and a flashing taskbar icon when another player writes your name or one of your words in chat. Your own messages and server announcements never ping; with a cooldown." },
    JavaModule { id: "name_hider", name: "Name Hider", category: 7, description: "Streamer mode: your name shows as a text you choose in chat, the tab list, name tags, scoreboard, titles and the HUD. Only on your screen." },
    JavaModule { id: "nametag_style", name: "Nametag Style", category: 7, description: "Smaller nametags and an optional background." },
    JavaModule { id: "party_hud", name: "Party", category: 7, description: "Your party: friends on the same server with their health, distance and an arrow pointing to them." },
    JavaModule { id: "pvp_tiers", name: "PvP Tiers", category: 7, description: "Shows players' PvP tier (HT1, LT3...) next to their name on name tags and in the tab list, from the public PvP tier list at pvptiers.com." },
    JavaModule { id: "album_colors", name: "Spotify Album Colors", category: 7, description: "While a song plays in Spotify, parts of the game take the colours of its album cover: HUD accent, sky, fog and more, fading between songs. Only a tint while it's on: your own colour settings are never changed. Reads Spotify like Spotify Lyrics (no login)." },
    JavaModule { id: "spotify_lyrics", name: "Spotify Lyrics", category: 7, description: "The lyrics of the song playing in Spotify on this PC, in sync: big tilted lines whose words pop in as they're sung (Cinematic), or floating in the world and on the HUD (Classic), with a card or toast for each new song. No Spotify login: reads Spotify from your system and asks lrclib.net for the lyrics, or else the Arsenal website, which tries other lyric sites (only the song's title, artist, album and length are sent)." },
    JavaModule { id: "wardrobe_kill_effects", name: "Wardrobe Kill Effects", category: 7, description: "Your kill effect from the Wardrobe plays where a player you defeat falls, and other Arsenal users' effects play for their kills. Only on your screen." },
    JavaModule { id: "usage_stats", name: "Anonymous Usage Stats", category: 8, description: "Counts you once a day (as a one-way hash, no name or IP) with your Arsenal and Minecraft version, so the author sees how many people play." },
    JavaModule { id: "auto_reconnect", name: "Auto Reconnect", category: 8, description: "Reconnects to the server after a disconnect, with a Cancel button." },
    JavaModule { id: "better_tooltips", name: "Better Tooltips", category: 8, description: "Durability as numbers (green/yellow/red), and what food restores (hunger, saturation and effects like Regeneration II) in item tooltips." },
    JavaModule { id: "cloud_backup", name: "Cloud Backup", category: 8, description: "Backs your Arsenal setup and all your profiles up under your Minecraft account (checked with Mojang's certificate, so only you can read it). Profiles are backed up as soon as you save them. Restore everything on any PC." },
    JavaModule { id: "crash_reports", name: "Crash Reports", category: 8, description: "If Arsenal ever crashes your game, the next start asks whether to send the error (no names or paths) so it gets fixed." },
    JavaModule { id: "food_preview", name: "Food Preview", category: 8, description: "Saturation as golden outlines on the hunger bar; the hunger and saturation the food in your hand would add flash in place. Optional saturation numbers." },
    JavaModule { id: "hypixel_skyblock", name: "Hypixel SkyBlock", category: 8, description: "Hypixel SkyBlock helpers, information only: a HUD with your area, purse, bits and the SkyBlock date, plus optional item prices in tooltips (bazaar, lowest BIN, NPC, from Hypixel's public API), Booster Cookie, God Potion and ability timers, a dungeon run card, slayer quest progress, your pet and a filter for spammy lines. Rules-safe: it only reads what the sidebar, tab list and chat already show, never plays for you or sends anything to Hypixel's game server, and only runs on Hypixel while you're in SkyBlock." },
    JavaModule { id: "inventory_cursor_speed", name: "Inventory Sensitivity", category: 8, description: "Experimental: a separate mouse speed in inventories. Your real cursor is never moved; Arsenal draws its own. Press F8 to turn it off instantly." },
    JavaModule { id: "performance_mode", name: "Performance Mode", category: 8, description: "One switch (give it a key) for more FPS: heavy visual features turn off, particles are capped lower and lyrics stop animating. Switching it off puts every feature back exactly as it was, even after a restart." },
    JavaModule { id: "quick_menu", name: "Quick Menu", category: 8, description: "Hold V for a wheel of your favourite features (☆); let go over one to switch it." },
    JavaModule { id: "screenshot_clipboard", name: "Screenshot to Clipboard", category: 8, description: "F2 screenshots are also copied to the clipboard as an image." },
    JavaModule { id: "server_notes", name: "Server Notes", category: 8, description: "Your own private notes per server: write them from the multiplayer list (Note button), with /arsenal note or a key. Shown when you hover the server and, if you like, in the tab list. Never shared." },
    JavaModule { id: "server_profiles", name: "Server Profiles", category: 8, description: "Loads one of your profiles automatically when you join a server. Set it with Auto in Profiles." },
    JavaModule { id: "session_summary", name: "Session Summary", category: 8, description: "When you leave a server: playtime, kills, deaths, K/D, totem pops and your best combo. Can add them to your leaderboard totals on the website." },
    JavaModule { id: "shulker_preview", name: "Shulker Box Preview", category: 8, description: "Hovering a shulker box shows its contents as a grid in the shulker's colour." },
    JavaModule { id: "toggle_sprint", name: "Toggle Sprint & Sneak", category: 8, description: "Press sprint (or sneak) once to keep it on, again to stop. Shows it on the HUD." },
    JavaModule { id: "update_notice", name: "Update Notice", category: 8, description: "Updates Arsenal automatically (or asks first), from the signed Arsenal API." },
];
// END JAVA MODULES

/// A setting: its key (its file), its label and its choices (stored value, shown value).
pub struct Field {
    pub key: &'static str,
    pub label: &'static str,
    pub options: &'static [(&'static str, &'static str)],
    pub default: usize,
}

const ONOFF: &[(&str, &str)] = &[("false", "OFF"), ("true", "ON")];
pub const CORNERS: &[(&str, &str)] = &[("top_left", "Top left"), ("top_right", "Top right"), ("bottom_left", "Bottom left"), ("bottom_right", "Bottom right")];
pub const ACCENTS: &[(&str, &str)] = &[("gold", "Gold"), ("red", "Red"), ("blue", "Blue"), ("green", "Green"), ("purple", "Purple"), ("white", "White")];
/// The accents' colours, as `ACCENTS`.
pub const ACCENT_RGB: [u32; 6] = [0xF5B83D, 0xFF5A5A, 0x5AA9FF, 0x5CE08A, 0xB884FF, 0xFFFFFF];
pub const ZOOMS: &[(&str, &str)] = &[("2", "2x"), ("3", "3x"), ("4", "4x"), ("6", "6x"), ("8", "8x")];
pub const ITEM_SIZES: &[(&str, &str)] = &[("100", "100%"), ("90", "90%"), ("80", "80%"), ("70", "70%"), ("60", "60%"), ("50", "50%")];
pub const DISTANCES: &[(&str, &str)] = &[("4", "4 blocks (vanilla)"), ("2", "2 blocks"), ("3", "3 blocks"), ("6", "6 blocks"), ("8", "8 blocks")];
pub const FOGS: &[(&str, &str)] = &[("normal", "Normal"), ("less", "Less"), ("none", "None")];

pub const INFO: usize = 0;
pub const FPS: usize = 1;
pub const CPS: usize = 2;
pub const COORDS: usize = 3;
pub const FACING: usize = 4;
pub const CORNER: usize = 5;
pub const THEME: usize = 6;
pub const ACCENT: usize = 7;
pub const ZOOM_ON: usize = 8;
pub const ZOOM: usize = 9;
pub const SMOOTH: usize = 10;
pub const VIEW: usize = 11;
pub const ITEM: usize = 12;
pub const CAMERA: usize = 13;
pub const DISTANCE: usize = 14;
pub const FOG: usize = 15;

pub const FIELDS: [Field; 16] = [
    Field { key: "info", label: "Info HUD", options: ONOFF, default: 1 },
    Field { key: "fps", label: "FPS", options: ONOFF, default: 1 },
    Field { key: "cps", label: "Clicks per second", options: ONOFF, default: 1 },
    Field { key: "coords", label: "Coordinates", options: ONOFF, default: 1 },
    Field { key: "facing", label: "Compass", options: ONOFF, default: 1 },
    Field { key: "corner", label: "HUD corner", options: CORNERS, default: 0 },
    Field { key: "theme", label: "HUD Theme", options: ONOFF, default: 1 },
    Field { key: "accent", label: "Accent", options: ACCENTS, default: 0 },
    Field { key: "zoom_on", label: "Zoom", options: ONOFF, default: 1 },
    Field { key: "zoom", label: "Zoom level", options: ZOOMS, default: 2 },
    Field { key: "smooth", label: "Smooth zoom", options: ONOFF, default: 1 },
    Field { key: "view_model", label: "Hand View Model", options: ONOFF, default: 0 },
    Field { key: "item", label: "Held item size", options: ITEM_SIZES, default: 0 },
    Field { key: "camera", label: "Camera Tweaks", options: ONOFF, default: 0 },
    Field { key: "distance", label: "Third-person distance", options: DISTANCES, default: 0 },
    Field { key: "fog", label: "Fog", options: FOGS, default: 0 },
];

/// The Java modules Horizon runs here: (Java module id, the setting that switches it, its other settings).
pub const ON_HORIZON: &[(&str, usize, &[usize])] = &[
    ("info_hud", INFO, &[FPS, CPS, CORNER]),
    ("coordinates", COORDS, &[CORNER]),
    ("compass", FACING, &[CORNER]),
    ("hud_theme", THEME, &[ACCENT]),
    ("zoom", ZOOM_ON, &[ZOOM, SMOOTH]),
    ("view_model", VIEW, &[ITEM]),
    ("camera_tweaks", CAMERA, &[DISTANCE, FOG]),
];

/// What Horizon runs of the Java module `id`, if it runs it.
pub fn on_horizon(id: &str) -> Option<(usize, &'static [usize])> {
    ON_HORIZON.iter().find(|m| m.0 == id).map(|m| (m.1, m.2))
}

const ZOOM_MIN: f32 = 1.25;
const ZOOM_MAX: f32 = 10.0;
/// Each wheel step multiplies the zoom by this.
const ZOOM_STEP: f32 = 1.25;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// Each field's choice, as `FIELDS`.
    pub v: [usize; 16],
}

impl Default for Settings {
    fn default() -> Self {
        let mut v = [0; 16];
        for (i, f) in FIELDS.iter().enumerate() {
            v[i] = f.default;
        }
        Self { v }
    }
}

impl Settings {
    /// From the stored values (`get` reads one key's file); the defaults for the ones missing or unreadable.
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let mut s = Self::default();
        for (i, f) in FIELDS.iter().enumerate() {
            if let Some(value) = get(f.key) {
                let value = value.trim().to_ascii_lowercase();
                if let Some(at) = f.options.iter().position(|o| o.0 == value || o.1.eq_ignore_ascii_case(&value)) {
                    s.v[i] = at;
                }
            }
        }
        s
    }

    pub fn on(&self, field: usize) -> bool {
        FIELDS[field].options == ONOFF && self.v[field] == 1
    }

    /// The label of field `i` with its value: "Zoom level: 4x".
    pub fn label(&self, i: usize) -> String {
        format!("{}: {}", FIELDS[i].label, FIELDS[i].options[self.v[i]].1)
    }

    /// Options > Mods > Arsenal: every setting, one button each.
    pub fn labels(&self) -> Vec<String> {
        (0..FIELDS.len()).map(|i| self.label(i)).collect()
    }

    /// A click on setting `i`: its next choice.
    pub fn click(&mut self, i: usize) {
        if let Some(f) = FIELDS.get(i) {
            self.v[i] = (self.v[i] + 1) % f.options.len();
        }
    }

    /// What to store, key by key.
    pub fn stored(&self) -> Vec<(&'static str, String)> {
        FIELDS.iter().enumerate().map(|(i, f)| (f.key, f.options[self.v[i]].0.to_owned())).collect()
    }

    fn number(&self, field: usize) -> f32 {
        FIELDS[field].options[self.v[field]].0.parse().unwrap_or(1.0)
    }

    /// The panel's and the menu's accent (0xRRGGBB): HUD Theme's, else gold.
    pub fn accent(&self) -> u32 {
        if self.on(THEME) { ACCENT_RGB[self.v[ACCENT]] } else { ACCENT_RGB[0] }
    }

    pub fn corner(&self) -> usize {
        self.v[CORNER]
    }

    /// The held item's scale (Hand View Model).
    pub fn item_scale(&self) -> f32 {
        if self.on(VIEW) { self.number(ITEM) / 100.0 } else { 1.0 }
    }

    /// The third-person camera's distance for vanilla's `distance` (4 blocks, scaled by the player's size), scaled
    /// the same way (Camera Tweaks).
    pub fn camera_distance(&self, distance: f32) -> f32 {
        if self.on(CAMERA) { distance * self.number(DISTANCE) / 4.0 } else { distance }
    }

    /// The environmental fog's start and end for vanilla's, pushed away by Camera Tweaks' fog. Only ever further,
    /// and end stays past start, as Horizon requires.
    pub fn fog_range(&self, start: f32, end: f32) -> (f32, f32) {
        match (self.on(CAMERA), self.v[FOG]) {
            (true, 1) => (start * 2.0, (end * 2.0).max(start * 2.0 + 1.0)),
            (true, 2) => (1.0e6, 1.0e6 + 1.0),
            _ => (start, end),
        }
    }

    pub fn zoom_level(&self) -> f32 {
        self.number(ZOOM)
    }
}

/// The Right Shift menu's own state.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Menu {
    pub category: usize,
    /// The first module row shown (the wheel moves it).
    pub scroll: usize,
    /// The module whose settings are open (index in `JAVA_MODULES`).
    pub detail: Option<usize>,
}

#[derive(Default)]
struct State {
    /// Read once, then kept; the settings pages replace it.
    settings: Option<Settings>,
    /// The zoom while the key is held; None when it is up.
    held: Option<f32>,
    menu: Menu,
    /// The menu's close button was clicked: closed on the next tick (the click hook can't close screens).
    close: bool,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

/// The settings, read once through `get`.
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    STATE.with_borrow_mut(|s| *s.settings.get_or_insert_with(|| Settings::read(get)))
}

/// Replaces the settings (a settings page changed them).
pub fn set_settings(settings: Settings) {
    STATE.with_borrow_mut(|s| s.settings = Some(settings));
}

pub fn menu() -> Menu {
    STATE.with_borrow(|s| s.menu.clone())
}

pub fn set_menu(menu: Menu) {
    STATE.with_borrow_mut(|s| s.menu = menu);
}

/// Asks for the menu to close on the next tick.
pub fn request_close() {
    STATE.with_borrow_mut(|s| s.close = true);
}

/// Whether the menu should close now (asked since the last tick).
pub fn take_close() -> bool {
    STATE.with_borrow_mut(|s| std::mem::take(&mut s.close))
}

/// The zoom key went down (only while playing) or up.
pub fn zoom_key(playing: bool, pressed: bool, get: impl Fn(&str) -> Option<String>) {
    if pressed && !playing {
        return;
    }
    let s = settings(get);
    if !s.on(ZOOM_ON) {
        return;
    }
    STATE.with_borrow_mut(|st| st.held = pressed.then(|| s.zoom_level()));
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

/// The zoom in effect: (factor, smooth, turning scale), none when the key is up. Turning slows with the zoom.
pub fn view() -> Option<(f32, bool, f32)> {
    STATE.with_borrow(|s| {
        let factor = s.held?;
        let smooth = s.settings.unwrap_or_default().on(SMOOTH);
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

/// The panel's lines, label and value, as the settings pick them.
pub fn lines(s: &Settings, r: &Readings) -> Vec<(&'static str, String)> {
    let mut out = Vec::new();
    if s.on(INFO) && s.on(FPS) {
        out.push(("FPS", r.fps.to_string()));
    }
    if s.on(INFO) && s.on(CPS) {
        out.push(("CPS", format!("{} | {}", r.cps.0, r.cps.1)));
    }
    if let Some(([x, y, z], yaw)) = r.player {
        if s.on(COORDS) {
            let b = |v: f64| v.floor() as i64;
            out.push(("XYZ", format!("{} {} {}", b(x), b(y), b(z))));
        }
        if s.on(FACING) {
            out.push(("Facing", facing(yaw).to_owned()));
        }
    }
    out
}

/// One thing to draw: a rectangle `[x, y, w, h]` in 0xAARRGGBB, or text at `[x, y]` in 0xRRGGBB with a shadow or not.
#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Fill([f32; 4], u32),
    Text(String, [f32; 2], u32, bool),
}

/// The panel's background, as the HUD's own boxes are drawn.
pub const BACKGROUND: u32 = 0x9010_1014;
pub const WHITE: u32 = 0xFFFFFF;
const PAD: f32 = 4.0;
const STRIPE: f32 = 2.0;
const LINE: f32 = 10.0;
const GAP: f32 = 5.0;

/// The panel for `lines` in `accent` (0xRRGGBB), measured with `width`: its shapes (from its top left) and its size.
/// None when there is nothing to show.
pub fn panel(lines: &[(&str, String)], accent: u32, width: &dyn Fn(&str) -> f32) -> Option<(Vec<Shape>, f32, f32)> {
    if lines.is_empty() {
        return None;
    }
    let labels = lines.iter().map(|(l, _)| width(l)).fold(0.0, f32::max);
    let values = lines.iter().map(|(_, v)| width(v)).fold(0.0, f32::max);
    let w = STRIPE + PAD + labels + GAP + values + PAD;
    let h = PAD * 2.0 + lines.len() as f32 * LINE - 2.0;
    let mut out = vec![Shape::Fill([0.0, 0.0, w, h], BACKGROUND), Shape::Fill([0.0, 0.0, STRIPE, h], 0xFF00_0000 | accent)];
    for (i, (label, value)) in lines.iter().enumerate() {
        let y = PAD + i as f32 * LINE;
        out.push(Shape::Text((*label).to_owned(), [STRIPE + PAD, y], accent, true));
        out.push(Shape::Text(value.clone(), [STRIPE + PAD + labels + GAP, y], WHITE, true));
    }
    Some((out, w, h))
}

/// What a click on the menu does.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    Close,
    Category(usize),
    /// Switch the Horizon module (index in `JAVA_MODULES`) on or off.
    Toggle(usize),
    /// Open the module's settings.
    Open(usize),
    Back,
    /// A click on setting `i` (index in `FIELDS`).
    Setting(usize),
}

const MENU_BG: u32 = 0xF014_141A;
const SIDEBAR_BG: u32 = 0xF00E_0E12;
const ROW_BG: u32 = 0x18FF_FFFF;
const ROW_HOVER: u32 = 0x30FF_FFFF;
const MUTED: u32 = 0x9A9AA8;
const GREY: u32 = 0x5E5E6A;
const ROW: f32 = 26.0;
const SIDE: f32 = 104.0;
const HEADER: f32 = 24.0;

fn inside(r: [f32; 4], p: [f32; 2]) -> bool {
    p[0] >= r[0] && p[1] >= r[1] && p[0] < r[0] + r[2] && p[1] < r[1] + r[3]
}

/// `text` cut to fit `max` pixels, ending in "..." when cut.
pub fn fit(text: &str, max: f32, width: &dyn Fn(&str) -> f32) -> String {
    if width(text) <= max {
        return text.to_owned();
    }
    let mut out: String = text.to_owned();
    while !out.is_empty() && width(&format!("{out}...")) > max {
        out.pop();
    }
    format!("{}...", out.trim_end())
}

/// The modules of `category`, as indexes in `JAVA_MODULES`: Horizon's first, then the others, each by name.
pub fn category_modules(category: usize) -> Vec<usize> {
    let mut list: Vec<usize> = (0..JAVA_MODULES.len()).filter(|&i| JAVA_MODULES[i].category == category).collect();
    list.sort_by_key(|&i| (on_horizon(JAVA_MODULES[i].id).is_none(), JAVA_MODULES[i].name.to_ascii_lowercase()));
    list
}

/// How many module rows fit in a menu drawn in `screen` (`[x, y, w, h]`, GUI pixels).
pub fn rows_shown(screen: [f32; 4]) -> usize {
    let (_, h) = menu_size(screen);
    (((h - HEADER - 8.0) / ROW).floor().max(1.0)) as usize
}

fn menu_size(screen: [f32; 4]) -> (f32, f32) {
    ((screen[2] - 16.0).min(500.0).max(240.0), (screen[3] - 16.0).min(320.0).max(140.0))
}

/// The menu in `screen` (`[x, y, w, h]`, GUI pixels) with the pointer at `mouse`: what to draw, and where a click does
/// what (the last one under the pointer wins).
pub fn menu_layout(
    screen: [f32; 4],
    mouse: [f32; 2],
    m: &Menu,
    s: &Settings,
    width: &dyn Fn(&str) -> f32,
) -> (Vec<Shape>, Vec<([f32; 4], Action)>) {
    let accent = s.accent();
    let (w, h) = menu_size(screen);
    let x0 = (screen[0] + (screen[2] - w) / 2.0).round();
    let y0 = (screen[1] + (screen[3] - h) / 2.0).round();
    let mut out = vec![
        Shape::Fill([0.0, 0.0, screen[2] + screen[0], screen[3] + screen[1]], 0x7000_0000),
        Shape::Fill([x0, y0, w, h], MENU_BG),
        Shape::Fill([x0, y0, SIDE, h], SIDEBAR_BG),
        Shape::Fill([x0, y0, w, 2.0], 0xFF00_0000 | accent),
    ];
    let mut hits = Vec::new();
    out.push(Shape::Text("ARSENAL".into(), [x0 + 10.0, y0 + 9.0], accent, true));
    out.push(Shape::Text("on Horizon".into(), [x0 + 10.0, y0 + h - 14.0], GREY, false));
    let close = [x0 + w - 18.0, y0 + 5.0, 14.0, 14.0];
    out.push(Shape::Fill(close, if inside(close, mouse) { ROW_HOVER } else { ROW_BG }));
    out.push(Shape::Text("x".into(), [close[0] + 4.0, close[1] + 3.0], WHITE, false));
    hits.push((close, Action::Close));

    // The categories.
    for (i, name) in CATEGORIES.iter().enumerate() {
        let r = [x0, y0 + HEADER + 4.0 + i as f32 * 16.0, SIDE, 16.0];
        let selected = match m.detail.and_then(|d| JAVA_MODULES.get(d)) {
            Some(j) => j.category == i,
            None => m.category == i,
        };
        if selected {
            out.push(Shape::Fill(r, ROW_HOVER));
            out.push(Shape::Fill([r[0], r[1], 2.0, r[3]], 0xFF00_0000 | accent));
        } else if inside(r, mouse) {
            out.push(Shape::Fill(r, ROW_BG));
        }
        out.push(Shape::Text(fit(name, SIDE - 16.0, width), [r[0] + 9.0, r[1] + 4.0], if selected { WHITE } else { MUTED }, false));
        hits.push((r, Action::Category(i)));
    }

    let cx = x0 + SIDE + 8.0;
    let cw = w - SIDE - 16.0;
    let top = y0 + HEADER;
    match m.detail.and_then(|d| JAVA_MODULES.get(d)) {
        Some(module) => {
            let back = [cx, top, width("< Back") + 10.0, 14.0];
            out.push(Shape::Fill(back, if inside(back, mouse) { ROW_HOVER } else { ROW_BG }));
            out.push(Shape::Text("< Back".into(), [back[0] + 5.0, back[1] + 3.0], WHITE, false));
            hits.push((back, Action::Back));
            out.push(Shape::Text(module.name.into(), [back[0] + back[2] + 8.0, top + 3.0], accent, true));
            out.push(Shape::Text(fit(module.description, cw, width), [cx, top + 20.0], MUTED, false));
            let mut y = top + 36.0;
            if let Some((toggle, fields)) = on_horizon(module.id) {
                for &f in std::iter::once(&toggle).chain(fields.iter()) {
                    if y + 18.0 > y0 + h - 4.0 {
                        break;
                    }
                    let r = [cx, y, cw, 16.0];
                    out.push(Shape::Fill(r, if inside(r, mouse) { ROW_HOVER } else { ROW_BG }));
                    let label = if f == toggle { format!("Enabled: {}", FIELDS[f].options[s.v[f]].1) } else { s.label(f) };
                    out.push(Shape::Text(fit(&label, cw - 12.0, width), [r[0] + 6.0, r[1] + 4.0], WHITE, false));
                    hits.push((r, Action::Setting(f)));
                    y += 19.0;
                }
            } else {
                out.push(Shape::Text("Only in the Java version of Arsenal for now.".into(), [cx, y], GREY, false));
            }
        }
        None => {
            let list = category_modules(m.category);
            let shown = rows_shown(screen);
            let first = m.scroll.min(list.len().saturating_sub(shown));
            for (row, &i) in list.iter().skip(first).take(shown).enumerate() {
                let module = &JAVA_MODULES[i];
                let r = [cx, top + row as f32 * ROW, cw, ROW - 3.0];
                let here = on_horizon(module.id);
                let hover = inside(r, mouse);
                out.push(Shape::Fill(r, if hover && here.is_some() { ROW_HOVER } else { ROW_BG }));
                let right = 44.0;
                let (name_rgb, desc_rgb) = if here.is_some() { (WHITE, MUTED) } else { (GREY, GREY) };
                out.push(Shape::Text(fit(module.name, cw - right - 12.0, width), [r[0] + 6.0, r[1] + 3.0], name_rgb, false));
                out.push(Shape::Text(fit(module.description, cw - right - 12.0, width), [r[0] + 6.0, r[1] + 13.0], desc_rgb, false));
                match here {
                    Some((toggle, _)) => {
                        hits.push((r, Action::Open(i)));
                        let pill = [r[0] + r[2] - 30.0, r[1] + 6.0, 22.0, 11.0];
                        let on = s.v[toggle] == 1;
                        out.push(Shape::Fill(pill, if on { 0xFF00_0000 | accent } else { 0x50FF_FFFF }));
                        let knob = if on { pill[0] + pill[2] - 10.0 } else { pill[0] + 1.0 };
                        out.push(Shape::Fill([knob, pill[1] + 1.0, 9.0, 9.0], 0xFFFF_FFFF));
                        hits.push(([pill[0] - 4.0, r[1], pill[2] + 8.0, r[3]], Action::Toggle(i)));
                    }
                    None => {
                        let tag = "Java only";
                        out.push(Shape::Text(tag.into(), [r[0] + r[2] - width(tag) - 6.0, r[1] + 8.0], GREY, false));
                    }
                }
            }
            if list.len() > shown {
                let track = [cx + cw + 3.0, top, 2.0, shown as f32 * ROW - 3.0];
                out.push(Shape::Fill(track, ROW_BG));
                let part = shown as f32 / list.len() as f32;
                let at = first as f32 / list.len() as f32;
                out.push(Shape::Fill([track[0], track[1] + track[3] * at, 2.0, (track[3] * part).max(6.0)], 0xFF00_0000 | accent));
            }
        }
    }
    (out, hits)
}

/// What a click at `mouse` does, from the same layout as the drawing.
pub fn menu_click(screen: [f32; 4], mouse: [f32; 2], m: &Menu, s: &Settings, width: &dyn Fn(&str) -> f32) -> Option<Action> {
    let (_, hits) = menu_layout(screen, mouse, m, s, width);
    hits.iter().rev().find(|(r, _)| inside(*r, mouse)).map(|(_, a)| *a)
}

/// Applies `action` to the menu and the settings; true when the settings changed (to be stored).
pub fn apply(action: Action, m: &mut Menu, s: &mut Settings) -> bool {
    match action {
        Action::Close => {
            request_close();
            false
        }
        Action::Category(c) => {
            *m = Menu { category: c, scroll: 0, detail: None };
            false
        }
        Action::Back => {
            m.detail = None;
            false
        }
        Action::Open(i) => {
            m.detail = Some(i);
            false
        }
        Action::Toggle(i) => match on_horizon(JAVA_MODULES[i].id) {
            Some((toggle, _)) => {
                s.click(toggle);
                true
            }
            None => false,
        },
        Action::Setting(f) => {
            s.click(f);
            true
        }
    }
}

/// The wheel over the menu: `steps` up (positive) or down moves the list.
pub fn menu_scroll(m: &mut Menu, steps: f64, screen: [f32; 4]) {
    if m.detail.is_some() {
        return;
    }
    let len = category_modules(m.category).len();
    let max = len.saturating_sub(rows_shown(screen));
    let next = m.scroll.min(max) as i64 - steps.signum() as i64;
    m.scroll = next.clamp(0, max as i64) as usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stored<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        |key| pairs.iter().find(|p| p.0 == key).map(|p| p.1.to_owned())
    }

    fn width(t: &str) -> f32 {
        t.len() as f32 * 6.0
    }

    #[test]
    fn settings_round_trip_and_fall_back_to_defaults() {
        assert_eq!(Settings::read(stored(&[])), Settings::default());
        let mut s = Settings::read(stored(&[("corner", "bottom_right"), ("zoom", "6\n"), ("fog", "nope"), ("item", "55")]));
        assert_eq!((s.corner(), s.zoom_level(), s.v[FOG], s.v[ITEM]), (3, 6.0, 0, 0));
        s.click(CORNER);
        s.click(FOG);
        s.click(INFO);
        let back = s.stored();
        let pairs: Vec<(&str, &str)> = back.iter().map(|(k, v)| (*k, v.as_str())).collect();
        assert_eq!(Settings::read(stored(&pairs)), s);
        assert_eq!(s.label(INFO), "Info HUD: OFF");
        assert_eq!(s.label(CORNER), "HUD corner: Top left");
        assert_eq!(s.labels().len(), FIELDS.len());
    }

    #[test]
    fn modules_change_things_only_while_on() {
        let mut s = Settings::default();
        s.v[ITEM] = 2;
        s.v[DISTANCE] = 1;
        s.v[FOG] = 2;
        assert_eq!((s.item_scale(), s.camera_distance(4.0), s.fog_range(10.0, 20.0)), (1.0, 4.0, (10.0, 20.0)));
        s.click(VIEW);
        s.click(CAMERA);
        assert!((s.item_scale() - 0.8).abs() < 1e-6);
        assert_eq!(s.camera_distance(4.0), 2.0);
        let (a, b) = s.fog_range(10.0, 20.0);
        assert!(b > a && a > 1000.0);
        s.v[THEME] = 0;
        s.v[ACCENT] = 2;
        assert_eq!(s.accent(), ACCENT_RGB[0]);
        s.v[THEME] = 1;
        assert_eq!(s.accent(), ACCENT_RGB[2]);
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
        let r = Readings { fps: 144, cps: (7, 2), player: Some(([10.7, 64.0, -3.2], 180.0)) };
        let s = Settings::default();
        let l = lines(&s, &r);
        assert_eq!(l[2], ("XYZ", "10 64 -4".to_owned()));
        assert_eq!(l[3], ("Facing", "North -Z".to_owned()));
        let (shapes, w, h) = panel(&l, 0xF5B83D, &width).unwrap();
        assert_eq!(w, 2.0 + 4.0 + 36.0 + 5.0 + 48.0 + 4.0);
        assert_eq!(h, 8.0 + 40.0 - 2.0);
        assert_eq!(shapes[1], Shape::Fill([0.0, 0.0, 2.0, h], 0xFFF5B83D));
        assert_eq!(lines(&s, &Readings::default()).len(), 2);
        let mut none = s;
        none.v[INFO] = 0;
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

    #[test]
    fn every_horizon_module_is_a_java_module() {
        for (id, toggle, fields) in ON_HORIZON {
            assert!(JAVA_MODULES.iter().any(|m| m.id == *id), "{id} isn't in the Java mod's list");
            assert_eq!(FIELDS[*toggle].options, ONOFF);
            assert!(fields.iter().all(|&f| f < FIELDS.len()));
        }
        assert!(JAVA_MODULES.len() > 100);
        assert!(JAVA_MODULES.iter().all(|m| m.category < CATEGORIES.len()));
    }

    #[test]
    fn the_menu_clicks_what_it_draws() {
        let screen = [0.0, 0.0, 640.0, 360.0];
        let mut s = Settings::default();
        let mut m = Menu { category: 0, scroll: 0, detail: None };
        // HUD lists Horizon's modules first: the first row's switch toggles it.
        let list = category_modules(0);
        let first = list[0];
        assert!(on_horizon(JAVA_MODULES[first].id).is_some());
        let (shapes, hits) = menu_layout(screen, [0.0, 0.0], &m, &s, &width);
        assert!(shapes.iter().any(|sh| matches!(sh, Shape::Text(t, _, _, _) if t == "ARSENAL")));
        let toggle = hits.iter().find(|(_, a)| *a == Action::Toggle(first)).unwrap().0;
        let at = [toggle[0] + 2.0, toggle[1] + 2.0];
        let (field, _) = on_horizon(JAVA_MODULES[first].id).unwrap();
        let before = s.v[field];
        let action = menu_click(screen, at, &m, &s, &width).unwrap();
        assert_eq!(action, Action::Toggle(first));
        assert!(apply(action, &mut m, &mut s));
        assert_ne!(s.v[field], before);
        // A greyed-out module opens nothing and switches nothing.
        let java_only = *list.iter().find(|&&i| on_horizon(JAVA_MODULES[i].id).is_none()).unwrap();
        assert!(!hits.iter().any(|(_, a)| *a == Action::Toggle(java_only) || *a == Action::Open(java_only)));
        // Categories, the settings view and back.
        let cat = hits.iter().find(|(_, a)| *a == Action::Category(6)).unwrap().0;
        let a = menu_click(screen, [cat[0] + 3.0, cat[1] + 3.0], &m, &s, &width).unwrap();
        apply(a, &mut m, &mut s);
        assert_eq!(m.category, 6);
        let zoom = JAVA_MODULES.iter().position(|j| j.id == "zoom").unwrap();
        apply(Action::Open(zoom), &mut m, &mut s);
        let (_, hits) = menu_layout(screen, [0.0, 0.0], &m, &s, &width);
        assert!(hits.iter().any(|(_, a)| *a == Action::Setting(ZOOM)));
        assert!(apply(Action::Setting(ZOOM), &mut m, &mut s));
        assert_eq!(s.zoom_level(), 6.0);
        apply(Action::Back, &mut m, &mut s);
        assert_eq!(m.detail, None);
        apply(Action::Close, &mut m, &mut s);
        assert!(take_close());
        assert!(!take_close());
    }

    #[test]
    fn the_wheel_scrolls_within_the_list() {
        let screen = [0.0, 0.0, 400.0, 200.0];
        let mut m = Menu { category: 0, scroll: 0, detail: None };
        let len = category_modules(0).len();
        let max = len.saturating_sub(rows_shown(screen));
        menu_scroll(&mut m, 1.0, screen);
        assert_eq!(m.scroll, 0);
        for _ in 0..200 {
            menu_scroll(&mut m, -1.0, screen);
        }
        assert_eq!(m.scroll, max);
        assert_eq!(fit("A long description", 60.0, &width), "A long...");
    }
}
