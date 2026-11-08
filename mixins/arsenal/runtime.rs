// Arsenal on Horizon: the Arsenal PvP mod as a native mod. Right Shift opens Arsenal's menu as in the Java mod: a
// home page with Modules (every module the Java mod has, in its categories; the ones Horizon can run switch on and off
// with their settings, the others are greyed out with the reason), Browse (the website's crosshairs and HUD themes,
// applied with a click) and Cosmetics. Settings are one file per key in the mod's data folder (`native_setting`), and
// the same settings show in Options > Mods > Arsenal. This file is both the mod's Cargo library (its tests) and the
// module the bundle adds to the game, as `game::arsenal_native`.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// The mod's id (its settings folder and its keys' lines), name and version.
pub const ID: &str = "arsenal";
pub const NAME: &str = "Arsenal";
pub const VERSION: &str = "1.12.0";
pub const DESCRIPTION: &str = "Arsenal's PvP mod on Horizon. Right Shift opens Arsenal's menu: modules, the website's crosshairs and HUD themes, and every other Arsenal module listed.";
/// The website Browse reads.
pub const WEBSITE: &str = "https://arsenal.monoforge.net";

/// The keys' names and defaults: the menu (Right Shift, as in the Java mod) and zoom (C).
pub const MENU_KEY: &str = "menu";
pub const DEFAULT_MENU_KEY: &str = "key.keyboard.right.shift";
pub const ZOOM_KEY: &str = "zoom";
pub const DEFAULT_ZOOM_KEY: &str = "key.keyboard.c";

/// The Java mod's categories, in its order.
pub const CATEGORIES: [&str; 10] = ["HUD", "Combat", "Enemies", "Crystal & Anchor", "Mace", "World", "Hands & Camera", "Visuals", "Chat & Social", "Utility"];

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
    JavaModule { id: "health_hud", name: "Health HUD", category: 0, description: "Your health right under the crosshair: a number, a bar or both. Drag and size it in the HUD editor." },
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
    JavaModule { id: "item_names", name: "Dropped Item Names", category: 5, description: "Dropped items you can see show their name and count above them; valuable loot is gold and can glow." },
    JavaModule { id: "fullbright", name: "Fullbright", category: 5, description: "See in the dark without night vision." },
    JavaModule { id: "hide_nametags", name: "Hide Nametags", category: 5, description: "Hides the name tags above players' heads, also names servers draw above them. Friends can keep theirs; can be limited to fights." },
    JavaModule { id: "hit_effects", name: "Hit Effects", category: 5, description: "Custom hurt color, crit particles on every hit, optional hit sound." },
    JavaModule { id: "kill_effects", name: "Kill Effects", category: 5, description: "Lightning or totem particles (only for you) when a player you hit dies." },
    JavaModule { id: "low_fire", name: "Low Fire on Ground", category: 5, description: "Fire on the ground is drawn lower so it doesn't block your view." },
    JavaModule { id: "no_fog", name: "No Fog", category: 5, description: "Removes fog at render distance and in water/lava. Blindness and Darkness still work." },
    JavaModule { id: "particle_limiter", name: "Particle Limiter", category: 5, description: "Fewer particles for more FPS. Explosion particles are hidden by default." },
    JavaModule { id: "sound_tweaks", name: "Sound Tweaks", category: 5, description: "Volume for explosions, totem pops, player hits and mace smashes." },
    JavaModule { id: "weather_time", name: "Weather & Time", category: 5, description: "Only on your screen: no rain and a fixed time of day." },
    JavaModule { id: "old_animations", name: "1.8 Animations", category: 6, description: "1.8 looks on your screen only: the old sword block-hit pose when you hold right-click, and lower held items." },
    JavaModule { id: "blade_settings", name: "Blade Settings", category: 6, description: "Your Wardrobe blade your way: the inspect key, inspect speed and sounds, hide your hand so only the blade shows, and its size and place in your hand." },
    JavaModule { id: "camera_tweaks", name: "Camera Tweaks", category: 6, description: "Third-person camera distance, and no camera pushed in by walls." },
    JavaModule { id: "free_look", name: "Free Look", category: 6, description: "Hold Left Alt to look around your character in third person without turning: you keep walking and aiming the same way. Let go to get your view back." },
    JavaModule { id: "view_model", name: "Hand View Model", category: 6, description: "Move, scale and tilt the items in your hands." },
    JavaModule { id: "hide_own_particles", name: "Hide Own Effect Particles", category: 6, description: "Hides the potion swirls coming off you in first person." },
    JavaModule { id: "no_hurt_cam", name: "No Hurt-Cam", category: 6, description: "Removes the camera tilt when you take damage." },
    JavaModule { id: "no_pumpkin_overlay", name: "No Pumpkin Overlay", category: 6, description: "Removes the dark carved pumpkin overlay while you wear a pumpkin on your head." },
    JavaModule { id: "smaller_pvp", name: "Smaller PvP", category: 6, description: "Smaller totem animation, first-person shield/totem and fire overlay. Low Shield draws your raised shield lower, like old versions." },
    JavaModule { id: "swing_animation", name: "Swing Animation", category: 6, description: "Faster or slower hand swing (visual only)." },
    JavaModule { id: "totem_pop_style", name: "Totem Pop Style", category: 6, description: "How your totem pops look and sound: animation size, particles, a different sound and a coloured screen flash." },
    JavaModule { id: "zoom", name: "Zoom", category: 6, description: "Hold C to zoom, scroll to zoom further." },
    JavaModule { id: "ambience", name: "Ambience", category: 7, description: "Things floating in the air around you: fireflies, snow, petals, embers, sparkles or leaves, in your colour and size." },
    JavaModule { id: "armor_color", name: "Armor Color", category: 7, description: "Tints your armor (and optionally friends' or everyone's). Lighter armor like iron shows colours best." },
    JavaModule { id: "block_break_animation", name: "Block Break Animation", category: 7, description: "Blocks you break shatter into flying pieces, shrink away, burst or flash." },
    JavaModule { id: "block_outline", name: "Block Outline", category: 7, description: "Custom color and thickness for the block selection outline." },
    JavaModule { id: "bloom", name: "Bloom", category: 7, description: "Bright things glow softly: the sky, lights, fire, glowing knives and effects." },
    JavaModule { id: "crit_particles", name: "Crit Particles", category: 7, description: "Critical hit particles your way: stars, hearts, circles, sparks and more, in one colour, a gradient or a rainbow, as big and as many as you like." },
    JavaModule { id: "enchant_glint", name: "Enchant Glint", category: 7, description: "Enchant glint colour (red, blue, green, gold, white) and strength." },
    JavaModule { id: "hand_tint", name: "Hand Tint & Glow", category: 7, description: "Tints the items in your hands (your HUD Theme colour by default) and can make them glow brighter." },
    JavaModule { id: "jump_circles", name: "Jump Circles", category: 7, description: "When you jump, a smooth ring spreads out on the ground from your feet, in any colour or a rainbow." },
    JavaModule { id: "mist", name: "Mist", category: 7, description: "A low mist drifting over the ground around you, in the colour and thickness you pick." },
    JavaModule { id: "motion_blur", name: "Motion Blur", category: 7, description: "Smooth blur when you turn or move the camera. Adjustable strength; the HUD stays sharp. Works with World Look." },
    JavaModule { id: "place_animation", name: "Place Animation", category: 7, description: "Blocks you place pop in, flash, rise up or send out a ripple." },
    JavaModule { id: "world_colors", name: "Sky & World Colors", category: 7, description: "Your own sky, fog and cloud colours. Nights still get dark unless you turn off Follow daylight." },
    JavaModule { id: "sky_effects", name: "Sky Effects", category: 7, description: "The night sky comes alive: auroras, stars and constellations, shooting stars and meteor showers, galaxies, nebulae, planets and comets, in your colours." },
    JavaModule { id: "slick_floors", name: "Slick Floors", category: 7, description: "The ground around you looks wet: darker, glossy, with puddles and rain ripples." },
    JavaModule { id: "smooth_hotbar", name: "Smooth Hotbar", category: 7, description: "The hotbar's selection box glides to the slot you pick." },
    JavaModule { id: "smooth_inventory", name: "Smooth Inventory", category: 7, description: "Items you pick up in an inventory glide after the mouse, lean as you move them and pop when picked up." },
    JavaModule { id: "sweep_effect", name: "Sweep Effect", category: 7, description: "The sweeping edge slash your way: a coloured arc, a flat slash round the player, a ring, sparks or nothing, in your colours." },
    JavaModule { id: "swing_trails", name: "Swing Trails", category: 7, description: "A short glowing streak follows each swing of your weapon in first person." },
    JavaModule { id: "world_look", name: "World Look", category: 7, description: "Color grading for the world: Dark Blue, Night, Warm, Cold... or your own." },
    JavaModule { id: "arsenal_messages", name: "Arsenal Messages", category: 8, description: "Where Arsenal's own messages appear: chat, above the hotbar, small toasts, or only the chat's Arsenal tab. All of them are kept in that tab." },
    JavaModule { id: "arsenal_users", name: "Arsenal Users", category: 8, description: "The Arsenal logo next to players who also use Arsenal. Only a one-way hash of your account id is stored." },
    JavaModule { id: "chat_filter", name: "Chat Filter", category: 8, description: "Hides chat lines containing your words." },
    JavaModule { id: "chat_hotkeys", name: "Chat Hotkeys", category: 8, description: "Up to 8 keys that each send a chat message or command you set (gg, /spawn, /kit pvp). Only while playing, with a short wait so they can't spam." },
    JavaModule { id: "chat_mentions", name: "Chat Mentions", category: 8, description: "Highlights chat lines that mention you, with a soft ping." },
    JavaModule { id: "chat_tweaks", name: "Chat Tweaks", category: 8, description: "Timestamps, repeated messages stacked as (x3), longer chat history." },
    JavaModule { id: "chat_copy", name: "Click to Copy Chat", category: 8, description: "Click a line in chat to copy it without colours; shift-click copies just the sender's name. Links still work as before." },
    JavaModule { id: "death_message", name: "Death Message", category: 8, description: "Sends a chat message when you die, GG by default. Several messages split by | are picked at random, and {name} is the player who killed you." },
    JavaModule { id: "discord_presence", name: "Discord Rich Presence", category: 8, description: "Your Discord profile shows you play with Arsenal: the server, the fight you're in and your wins this session. Only talks to the Discord app on your PC." },
    JavaModule { id: "friends", name: "Friends", category: 8, description: "Mark friends (middle mouse on a player or /arsenal friend add): green name, never treated as enemies." },
    JavaModule { id: "kill_message", name: "Kill Message", category: 8, description: "Sends a chat message after you kill someone, GG by default. Several messages split by | are picked at random, and {name} is the player you killed." },
    JavaModule { id: "mention_ping", name: "Mention Ping", category: 8, description: "A sound, a highlight and a flashing taskbar icon when another player writes your name or one of your words in chat. Your own messages and server announcements never ping; with a cooldown." },
    JavaModule { id: "name_hider", name: "Name Hider", category: 8, description: "Streamer mode: your name shows as a text you choose in chat, the tab list, name tags, scoreboard, titles and the HUD. Only on your screen." },
    JavaModule { id: "nametag_style", name: "Nametag Style", category: 8, description: "Smaller nametags and an optional background." },
    JavaModule { id: "party_hud", name: "Party", category: 8, description: "Your party: friends on the same server with their health, distance and an arrow pointing to them." },
    JavaModule { id: "pvp_tiers", name: "PvP Tiers", category: 8, description: "Shows players' PvP tier (HT1, LT3...) next to their name on name tags and in the tab list, from the public PvP tier list at pvptiers.com." },
    JavaModule { id: "album_colors", name: "Spotify Album Colors", category: 8, description: "While a song plays in Spotify, parts of the game take the colours of its album cover: HUD accent, sky, fog and more, fading between songs. Only a tint while it's on: your own colour settings are never changed. Reads Spotify like Spotify Lyrics (no login)." },
    JavaModule { id: "spotify_lyrics", name: "Spotify Lyrics", category: 8, description: "The lyrics of the song playing in Spotify on this PC, in sync: big tilted lines whose words pop in as they're sung (Cinematic), or floating in the world and on the HUD (Classic), with a card or toast for each new song. No Spotify login: reads Spotify from your system and asks lrclib.net for the lyrics, or else the Arsenal website, which tries other lyric sites (only the song's title, artist, album and length are sent)." },
    JavaModule { id: "wardrobe_kill_effects", name: "Wardrobe Kill Effects", category: 8, description: "Your kill effect from the Wardrobe plays where a player you defeat falls, and other Arsenal users' effects play for their kills. Only on your screen." },
    JavaModule { id: "usage_stats", name: "Anonymous Usage Stats", category: 9, description: "Counts you once a day (as a one-way hash, no name or IP) with your Arsenal and Minecraft version, so the author sees how many people play." },
    JavaModule { id: "auto_reconnect", name: "Auto Reconnect", category: 9, description: "Reconnects to the server after a disconnect, with a Cancel button." },
    JavaModule { id: "better_tooltips", name: "Better Tooltips", category: 9, description: "Durability as numbers (green/yellow/red), and what food restores (hunger, saturation and effects like Regeneration II) in item tooltips." },
    JavaModule { id: "cloud_backup", name: "Cloud Backup", category: 9, description: "Backs your Arsenal setup and all your profiles up under your Minecraft account (checked with Mojang's certificate, so only you can read it). Profiles are backed up as soon as you save them. Restore everything on any PC." },
    JavaModule { id: "crash_reports", name: "Crash Reports", category: 9, description: "If Arsenal ever crashes your game, the next start asks whether to send the error (no names or paths) so it gets fixed." },
    JavaModule { id: "food_preview", name: "Food Preview", category: 9, description: "Saturation as golden outlines on the hunger bar; the hunger and saturation the food in your hand would add flash in place. Optional saturation numbers." },
    JavaModule { id: "hypixel_skyblock", name: "Hypixel SkyBlock", category: 9, description: "Hypixel SkyBlock helpers, information only: a HUD with your area, purse, bits and the SkyBlock date, plus optional item prices in tooltips (bazaar, lowest BIN, NPC, from Hypixel's public API), Booster Cookie, God Potion and ability timers, a dungeon run card, slayer quest progress, your pet and a filter for spammy lines. Rules-safe: it only reads what the sidebar, tab list and chat already show, never plays for you or sends anything to Hypixel's game server, and only runs on Hypixel while you're in SkyBlock." },
    JavaModule { id: "inventory_cursor_speed", name: "Inventory Sensitivity", category: 9, description: "Experimental: a separate mouse speed in inventories. Your real cursor is never moved; Arsenal draws its own. Press F8 to turn it off instantly." },
    JavaModule { id: "performance_mode", name: "Performance Mode", category: 9, description: "One switch (give it a key) for more FPS: heavy visual features turn off, particles are capped lower and lyrics stop animating. Switching it off puts every feature back exactly as it was, even after a restart." },
    JavaModule { id: "quick_menu", name: "Quick Menu", category: 9, description: "Hold V for a wheel of your favourite features (☆); let go over one to switch it." },
    JavaModule { id: "screenshot_clipboard", name: "Screenshot to Clipboard", category: 9, description: "F2 screenshots are also copied to the clipboard as an image." },
    JavaModule { id: "server_notes", name: "Server Notes", category: 9, description: "Your own private notes per server: write them from the multiplayer list (Note button), with /arsenal note or a key. Shown when you hover the server and, if you like, in the tab list. Never shared." },
    JavaModule { id: "server_profiles", name: "Server Profiles", category: 9, description: "Loads one of your profiles automatically when you join a server. Set it with Auto in Profiles." },
    JavaModule { id: "session_summary", name: "Session Summary", category: 9, description: "When you leave a server: playtime, kills, deaths, K/D, totem pops and your best combo. Can add them to your leaderboard totals on the website." },
    JavaModule { id: "shulker_preview", name: "Shulker Box Preview", category: 9, description: "Hovering a shulker box shows its contents as a grid in the shulker's colour." },
    JavaModule { id: "toggle_sprint", name: "Toggle Sprint & Sneak", category: 9, description: "Press sprint (or sneak) once to keep it on, again to stop. Shows it on the HUD." },
    JavaModule { id: "update_notice", name: "Update Notice", category: 9, description: "Updates Arsenal automatically (or asks first), from the signed Arsenal API." },
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
/// The accents; the last is the HUD theme applied from Browse.
pub const ACCENTS: &[(&str, &str)] = &[("gold", "Gold"), ("red", "Red"), ("blue", "Blue"), ("green", "Green"), ("purple", "Purple"), ("white", "White"), ("browse", "Theme from Browse")];
/// The accents' colours, as `ACCENTS` (but the last).
pub const ACCENT_RGB: [u32; 6] = [0xF5B83D, 0xFF5A5A, 0x5AA9FF, 0x5CE08A, 0xB884FF, 0xFFFFFF];
pub const ZOOMS: &[(&str, &str)] = &[("2", "2x"), ("3", "3x"), ("4", "4x"), ("6", "6x"), ("8", "8x")];
pub const ITEM_SIZES: &[(&str, &str)] = &[("100", "100%"), ("90", "90%"), ("80", "80%"), ("70", "70%"), ("60", "60%"), ("50", "50%")];
pub const DISTANCES: &[(&str, &str)] = &[("4", "4 blocks (vanilla)"), ("2", "2 blocks"), ("3", "3 blocks"), ("6", "6 blocks"), ("8", "8 blocks")];
pub const FOGS: &[(&str, &str)] = &[("less", "Less"), ("none", "None")];
pub const TAG_WHO: &[(&str, &str)] = &[("players", "Players"), ("all", "Players and mobs")];
pub const OUTLINE_COLORS: &[(&str, &str)] = &[("accent", "HUD accent"), ("white", "White"), ("red", "Red"), ("gold", "Gold"), ("blue", "Blue"), ("green", "Green")];
const OUTLINE_RGB: [u32; 5] = [0xFFFFFF, 0xFF5A5A, 0xF5B83D, 0x5AA9FF, 0x5CE08A];
pub const OUTLINE_WIDTHS: &[(&str, &str)] = &[("2", "Normal"), ("3", "Thick"), ("5", "Very thick"), ("1", "Thin")];
pub const HEALTH_SHOWS: &[(&str, &str)] = &[("both", "Number and bar"), ("number", "Number only"), ("bar", "Bar only")];
pub const HEALTH_UNITS: &[(&str, &str)] = &[("hearts", "Hearts (10 is full)"), ("points", "Health points (20 is full)")];
pub const KILL_TEXTS: &[(&str, &str)] = &[("GG", "GG"), ("gg wp", "gg wp"), ("Good fight!", "Good fight!"), ("gg {name}", "gg {name}"), ("ez", "ez")];
pub const DEATH_TEXTS: &[(&str, &str)] = &[("GG", "GG"), ("gg wp", "gg wp"), ("nice one {name}", "nice one {name}"), ("ff", "ff")];
pub const RING_COLORS: &[(&str, &str)] = &[("accent", "HUD accent"), ("white", "White"), ("rainbow", "Rainbow")];
pub const FILTER_WHAT: &[(&str, &str)] = &[("both", "Joins/leaves and ads"), ("joins", "Joins and leaves"), ("ads", "Ads and links")];
pub const FIRE_LOWERS: &[(&str, &str)] = &[("40", "40% lower"), ("20", "20% lower"), ("60", "60% lower"), ("80", "80% lower")];
pub const PARTICLE_KEEPS: &[(&str, &str)] = &[("50", "Keep half"), ("25", "Keep a quarter"), ("10", "Keep a tenth"), ("0", "None")];
pub const VIEW_TIMES: &[(&str, &str)] = &[("world", "The world's"), ("morning", "Morning"), ("noon", "Noon"), ("sunset", "Sunset"), ("midnight", "Midnight")];
pub const VIEW_WEATHERS: &[(&str, &str)] = &[("clear", "Clear"), ("world", "The world's"), ("rain", "Rain"), ("thunder", "Thunder")];
pub const SIZES: &[(&str, &str)] = &[("80", "80%"), ("90", "90%"), ("70", "70%"), ("60", "60%"), ("50", "50%"), ("100", "100%")];
pub const GLINTS: &[(&str, &str)] = &[("red", "Red"), ("blue", "Blue"), ("green", "Green"), ("yellow", "Gold"), ("white", "White"), ("cyan", "Cyan"), ("magenta", "Magenta"), ("orange", "Orange"), ("black", "Black")];
pub const HIT_COLORS: &[(&str, &str)] = &[("red", "Red"), ("blue", "Blue"), ("green", "Green"), ("yellow", "Gold"), ("white", "White"), ("magenta", "Magenta"), ("cyan", "Cyan")];
pub const CRIT_SHAPES: &[(&str, &str)] = &[("star", "Star"), ("heart", "Heart"), ("diamond", "Diamond"), ("circle", "Circle")];
pub const FX_COLORS: &[(&str, &str)] = &[("gold", "Gold"), ("accent", "HUD accent"), ("red", "Red"), ("white", "White"), ("rainbow", "Rainbow")];
pub const FLASH_COLORS: &[(&str, &str)] = &[("gold", "Gold"), ("green", "Green"), ("red", "Red"), ("blue", "Blue"), ("none", "No flash")];
pub const KILL_STYLES: &[(&str, &str)] = &[("burst", "Burst"), ("ring", "Ring"), ("lightning", "Lightning")];
pub const AMOUNTS: &[(&str, &str)] = &[("0.5", "Medium"), ("0.25", "Low"), ("0.75", "High"), ("1", "Full")];
pub const LOOKS: &[(&str, &str)] = &[("vivid", "Vivid"), ("cinematic", "Cinematic"), ("faded", "Faded"), ("noir", "Black and white"), ("warm", "Warm"), ("cold", "Cold")];
pub const TINTS: &[(&str, &str)] = &[("purple", "Purple"), ("blue", "Blue"), ("pink", "Pink"), ("green", "Green"), ("orange", "Orange"), ("red", "Red"), ("accent", "HUD accent")];
pub const MIST_COLORS: &[(&str, &str)] = &[("white", "White"), ("blue", "Blue"), ("purple", "Purple"), ("green", "Green")];
pub const GOALS: &[(&str, &str)] = &[("10", "10 kills"), ("5", "5 kills"), ("20", "20 kills"), ("50", "50 kills")];
pub const ARMOR_SHOWS: &[(&str, &str)] = &[("percent", "Percent"), ("left", "Durability left"), ("none", "Icons only")];
pub const DURA_AT: &[(&str, &str)] = &[("10", "10% left"), ("5", "5% left"), ("20", "20% left"), ("30", "30% left")];
pub const LOW_AT: &[(&str, &str)] = &[("4", "4 hearts or less"), ("3", "3 hearts or less"), ("5", "5 hearts or less"), ("6", "6 hearts or less")];

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
pub const SPEED: usize = 16;
pub const SESSION: usize = 17;
pub const CROSSHAIR: usize = 18;
pub const HIDE_BOSS: usize = 19;
pub const HIDE_TITLES: usize = 20;
pub const HIDE_SCORE: usize = 21;
pub const GRAPH: usize = 22;
pub const SNEAK: usize = 23;
pub const NO_FOG: usize = 24;
pub const HIDE_TAGS: usize = 25;
pub const TAGS_WHO: usize = 26;
pub const OUTLINE: usize = 27;
pub const OUTLINE_COLOR: usize = 28;
pub const OUTLINE_WIDTH: usize = 29;
pub const HEALTH: usize = 30;
pub const HEALTH_SHOW: usize = 31;
pub const HEALTH_UNIT: usize = 32;
pub const LOW_HEALTH: usize = 33;
pub const LOW_HEALTH_AT: usize = 34;
pub const HEARTBEAT: usize = 35;
pub const TOOLTIPS: usize = 36;
pub const SHULKER: usize = 37;
pub const SATURATION: usize = 38;
pub const THEME_TOOLTIPS: usize = 39;
pub const ARMOR: usize = 40;
pub const ARMOR_SHOW: usize = 41;
pub const COUNTER: usize = 42;
pub const OFFHAND_WARN: usize = 43;
pub const DURA_WARN: usize = 44;
pub const DURA_WARN_AT: usize = 45;
pub const READY: usize = 46;
pub const KILL_MSG: usize = 47;
pub const KILL_TEXT: usize = 48;
pub const DEATH_MSG: usize = 49;
pub const DEATH_TEXT: usize = 50;
pub const MENTION: usize = 51;
pub const COMBAT_TAG: usize = 52;
pub const POPS: usize = 53;
pub const POP_ALERTS: usize = 54;
pub const TARGET: usize = 55;
pub const HITMARKER: usize = 56;
pub const STREAKS: usize = 57;
pub const NAMETAG_HP: usize = 58;
pub const DAMAGE_NUMS: usize = 59;
pub const JUMP_RINGS: usize = 60;
pub const JUMP_COLOR: usize = 61;
pub const POTIONS: usize = 62;
pub const POTION_WARN: usize = 63;
pub const ATTACK_BAR: usize = 64;
pub const CRIT_IND: usize = 65;
pub const NAMETAG_PING: usize = 66;
pub const LAG: usize = 67;
pub const CLOCK: usize = 68;
pub const SESSION_STATS: usize = 69;
pub const FIGHT_TIMER: usize = 70;
pub const ENEMY_ARMOR: usize = 71;
pub const LOW_HP_ALERT: usize = 72;
pub const INV_TOTEM: usize = 73;
pub const COMBO_REACH: usize = 74;
pub const COMBO_POP: usize = 75;
pub const DEATH_RECAP: usize = 76;
pub const FEED: usize = 77;
pub const FIGHT_SUMMARY: usize = 78;
pub const SHIELD_SOUND: usize = 79;
pub const DYN_CROSSHAIR: usize = 80;
pub const COOLDOWN: usize = 81;
pub const TNT: usize = 82;
pub const ANCHOR: usize = 83;
pub const CRYSTAL_DMG: usize = 84;
pub const FALL: usize = 85;
pub const CHAT_FILTER: usize = 86;
pub const FILTER_KIND: usize = 87;
pub const FULLBRIGHT: usize = 88;
pub const HIT_FX: usize = 89;
pub const HIT_COLOR: usize = 90;
pub const LOW_FIRE: usize = 91;
pub const FIRE_LOWER: usize = 92;
pub const PARTICLES: usize = 93;
pub const PARTICLE_KEEP: usize = 94;
pub const WEATHER_TIME: usize = 95;
pub const VIEW_TIME: usize = 96;
pub const VIEW_WEATHER: usize = 97;
pub const NO_HURT: usize = 98;
pub const NO_PUMPKIN: usize = 99;
pub const SMALLER: usize = 100;
pub const SHIELD_SIZE: usize = 101;
pub const TOTEM_SIZE: usize = 102;
pub const GLINT: usize = 103;
pub const GLINT_COLOR: usize = 104;
pub const DISCORD: usize = 105;
pub const TIERS: usize = 106;
pub const USERS: usize = 107;
pub const CRITS: usize = 108;
pub const CRIT_SHAPE: usize = 109;
pub const CRIT_COLOR: usize = 110;
pub const SWEEP: usize = 111;
pub const SWEEP_COLOR: usize = 112;
pub const SWING: usize = 113;
pub const TOTEM_STYLE: usize = 114;
pub const TOTEM_FLASH: usize = 115;
pub const TOTEM_HIDE: usize = 116;
pub const KILL_FX: usize = 117;
pub const KILL_STYLE: usize = 118;
pub const HIDE_OWN_FX: usize = 119;
pub const BLOOM: usize = 120;
pub const BLOOM_AMT: usize = 121;
pub const LOOK: usize = 122;
pub const LOOK_KIND: usize = 123;
pub const COLORS: usize = 124;
pub const WORLD_TINT: usize = 125;
pub const MOTION: usize = 126;
pub const MOTION_AMT: usize = 127;
pub const MIST: usize = 128;
pub const MIST_COLOR: usize = 129;
pub const SKY_FX: usize = 130;
pub const AMBIENCE: usize = 131;
pub const HITBOXES: usize = 132;
pub const AUTO_HITBOX: usize = 133;
pub const RETOTEM: usize = 134;
pub const SHIELD_HELP: usize = 135;
pub const EATING: usize = 136;
pub const ITEM_NAMES: usize = 137;
pub const BOW: usize = 138;
pub const MACE_AIM: usize = 139;
pub const MACE_OPT: usize = 140;
pub const CRYSTAL_PREV: usize = 141;
pub const CRYSTAL_OPT: usize = 142;
pub const HOTKEYS: usize = 143;
pub const FRIENDS: usize = 144;
pub const SUMMARY: usize = 145;
pub const GOAL: usize = 146;
pub const GOAL_KILLS: usize = 147;
pub const INTEL: usize = 148;
pub const PEARLS: usize = 149;
const N: usize = 150;

pub const FIELDS: [Field; N] = [
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
    Field { key: "fog", label: "Fog", options: FOGS, default: 1 },
    Field { key: "speed", label: "Speed Display", options: ONOFF, default: 0 },
    Field { key: "session", label: "Session Clock", options: ONOFF, default: 0 },
    Field { key: "crosshair_on", label: "Crosshair Editor", options: ONOFF, default: 0 },
    Field { key: "hide_boss", label: "Hide the boss bar", options: ONOFF, default: 0 },
    Field { key: "hide_titles", label: "Hide titles and the action bar", options: ONOFF, default: 0 },
    Field { key: "hide_score", label: "Hide the scoreboard", options: ONOFF, default: 0 },
    Field { key: "fps_graph", label: "FPS Graph", options: ONOFF, default: 0 },
    Field { key: "toggle_sneak", label: "Toggle Sneak", options: ONOFF, default: 0 },
    Field { key: "no_fog", label: "No Fog", options: ONOFF, default: 0 },
    Field { key: "hide_tags", label: "Hide Nametags", options: ONOFF, default: 0 },
    Field { key: "tags_who", label: "Hide for", options: TAG_WHO, default: 0 },
    Field { key: "outline", label: "Block Outline", options: ONOFF, default: 0 },
    Field { key: "outline_color", label: "Outline colour", options: OUTLINE_COLORS, default: 0 },
    Field { key: "outline_width", label: "Outline thickness", options: OUTLINE_WIDTHS, default: 0 },
    Field { key: "health_hud", label: "Health HUD", options: ONOFF, default: 0 },
    Field { key: "health_show", label: "Show", options: HEALTH_SHOWS, default: 0 },
    Field { key: "health_units", label: "Units", options: HEALTH_UNITS, default: 0 },
    Field { key: "low_health", label: "Low Health Warning", options: ONOFF, default: 0 },
    Field { key: "low_health_at", label: "Warn at", options: LOW_AT, default: 0 },
    Field { key: "heartbeat", label: "Heartbeat sound", options: ONOFF, default: 1 },
    Field { key: "better_tooltips", label: "Better Tooltips", options: ONOFF, default: 0 },
    Field { key: "shulker_preview", label: "Shulker Preview", options: ONOFF, default: 0 },
    Field { key: "saturation", label: "Food Preview", options: ONOFF, default: 0 },
    Field { key: "theme_tooltips", label: "Tooltips in the theme's colours", options: ONOFF, default: 1 },
    Field { key: "armor_hud", label: "Armor HUD", options: ONOFF, default: 0 },
    Field { key: "armor_show", label: "Durability", options: ARMOR_SHOWS, default: 0 },
    Field { key: "item_counter", label: "Item Counter", options: ONOFF, default: 0 },
    Field { key: "offhand_warning", label: "Offhand Totem Warning", options: ONOFF, default: 0 },
    Field { key: "durability_warning", label: "Durability Warning", options: ONOFF, default: 0 },
    Field { key: "durability_at", label: "Warn at", options: DURA_AT, default: 0 },
    Field { key: "ready_check", label: "Ready Check", options: ONOFF, default: 0 },
    Field { key: "kill_message", label: "Kill Message", options: ONOFF, default: 0 },
    Field { key: "kill_text", label: "Message", options: KILL_TEXTS, default: 0 },
    Field { key: "death_message", label: "Death Message", options: ONOFF, default: 0 },
    Field { key: "death_text", label: "Message", options: DEATH_TEXTS, default: 0 },
    Field { key: "mention_ping", label: "Mention Ping", options: ONOFF, default: 0 },
    Field { key: "combat_tag", label: "Combat Tag Timer", options: ONOFF, default: 0 },
    Field { key: "pop_counter", label: "Totem Pop Counter", options: ONOFF, default: 0 },
    Field { key: "pop_alerts", label: "Totem Pop Alerts", options: ONOFF, default: 0 },
    Field { key: "target_hud", label: "Target HUD", options: ONOFF, default: 0 },
    Field { key: "hitmarker", label: "Hitmarker", options: ONOFF, default: 0 },
    Field { key: "kill_streaks", label: "Kill Streaks", options: ONOFF, default: 0 },
    Field { key: "nametag_health", label: "Nametag Health", options: ONOFF, default: 0 },
    Field { key: "damage_numbers", label: "Damage Numbers", options: ONOFF, default: 0 },
    Field { key: "jump_circles", label: "Jump Circles", options: ONOFF, default: 0 },
    Field { key: "jump_color", label: "Colour", options: RING_COLORS, default: 0 },
    Field { key: "potion_timers", label: "Potion Timers", options: ONOFF, default: 0 },
    Field { key: "potion_warning", label: "Potion Run-out Warning", options: ONOFF, default: 0 },
    Field { key: "attack_bar", label: "Attack Charge Bar", options: ONOFF, default: 0 },
    Field { key: "crit_indicator", label: "Crit Indicator", options: ONOFF, default: 0 },
    Field { key: "nametag_ping", label: "Ping on Nametags", options: ONOFF, default: 0 },
    Field { key: "lag_meter", label: "Lag Meter", options: ONOFF, default: 0 },
    Field { key: "clock_info", label: "Clock & Server Info", options: ONOFF, default: 0 },
    Field { key: "session_stats", label: "Session Stats", options: ONOFF, default: 0 },
    Field { key: "fight_timer", label: "Fight Timer", options: ONOFF, default: 0 },
    Field { key: "enemy_armor_alert", label: "Enemy Armor Alert", options: ONOFF, default: 0 },
    Field { key: "target_low_hp", label: "Target Low HP Alert", options: ONOFF, default: 0 },
    Field { key: "inventory_totem", label: "Inventory Totem", options: ONOFF, default: 0 },
    Field { key: "combo_reach", label: "Combo & Reach", options: ONOFF, default: 0 },
    Field { key: "combo_pop", label: "Combo Counter Pop", options: ONOFF, default: 0 },
    Field { key: "death_recap", label: "Death Recap", options: ONOFF, default: 0 },
    Field { key: "notification_feed", label: "Notification Feed", options: ONOFF, default: 0 },
    Field { key: "fight_summary", label: "Fight Summary", options: ONOFF, default: 0 },
    Field { key: "shield_break_sound", label: "Shield Break Sound", options: ONOFF, default: 0 },
    Field { key: "dynamic_crosshair", label: "Dynamic Crosshair", options: ONOFF, default: 0 },
    Field { key: "cooldown_hud", label: "Cooldown HUD", options: ONOFF, default: 0 },
    Field { key: "tnt_timer", label: "TNT Timer", options: ONOFF, default: 0 },
    Field { key: "anchor_helper", label: "Anchor Helper", options: ONOFF, default: 0 },
    Field { key: "crystal_damage", label: "Crystal Damage Preview", options: ONOFF, default: 0 },
    Field { key: "fall_preview", label: "Fall Damage Preview", options: ONOFF, default: 0 },
    Field { key: "chat_filter", label: "Chat Filter", options: ONOFF, default: 0 },
    Field { key: "filter_what", label: "Hide", options: FILTER_WHAT, default: 0 },
    Field { key: "fullbright", label: "Fullbright", options: ONOFF, default: 0 },
    Field { key: "hit_effects", label: "Hit Effects", options: ONOFF, default: 0 },
    Field { key: "hit_color", label: "Hit colour", options: HIT_COLORS, default: 0 },
    Field { key: "low_fire", label: "Low Fire", options: ONOFF, default: 0 },
    Field { key: "fire_lower", label: "Fire overlay", options: FIRE_LOWERS, default: 0 },
    Field { key: "particle_limiter", label: "Particle Limiter", options: ONOFF, default: 0 },
    Field { key: "particle_keep", label: "Particles", options: PARTICLE_KEEPS, default: 0 },
    Field { key: "weather_time", label: "Weather & Time", options: ONOFF, default: 0 },
    Field { key: "view_time", label: "Time", options: VIEW_TIMES, default: 0 },
    Field { key: "view_weather", label: "Weather", options: VIEW_WEATHERS, default: 0 },
    Field { key: "no_hurt_cam", label: "No Hurt-Cam", options: ONOFF, default: 0 },
    Field { key: "no_pumpkin", label: "No Pumpkin Overlay", options: ONOFF, default: 0 },
    Field { key: "smaller_pvp", label: "Smaller PvP", options: ONOFF, default: 0 },
    Field { key: "shield_size", label: "Shield size", options: SIZES, default: 0 },
    Field { key: "totem_size", label: "Totem animation size", options: SIZES, default: 0 },
    Field { key: "enchant_glint", label: "Enchant Glint", options: ONOFF, default: 0 },
    Field { key: "glint_color", label: "Glint colour", options: GLINTS, default: 0 },
    Field { key: "discord_presence", label: "Discord Rich Presence", options: ONOFF, default: 1 },
    Field { key: "pvp_tiers", label: "PvP Tiers", options: ONOFF, default: 0 },
    Field { key: "arsenal_users", label: "Arsenal Users", options: ONOFF, default: 1 },
    Field { key: "crit_particles", label: "Crit Particles", options: ONOFF, default: 0 },
    Field { key: "crit_shape", label: "Shape", options: CRIT_SHAPES, default: 0 },
    Field { key: "crit_color", label: "Colour", options: FX_COLORS, default: 0 },
    Field { key: "sweep_effect", label: "Sweep Effect", options: ONOFF, default: 0 },
    Field { key: "sweep_color", label: "Colour", options: FX_COLORS, default: 3 },
    Field { key: "swing_trails", label: "Swing Trails", options: ONOFF, default: 0 },
    Field { key: "totem_pop_style", label: "Totem Pop Style", options: ONOFF, default: 0 },
    Field { key: "totem_flash", label: "Flash", options: FLASH_COLORS, default: 0 },
    Field { key: "totem_particles", label: "Hide totem particles", options: ONOFF, default: 0 },
    Field { key: "kill_effects", label: "Kill Effects", options: ONOFF, default: 0 },
    Field { key: "kill_style", label: "Effect", options: KILL_STYLES, default: 0 },
    Field { key: "hide_own_particles", label: "Hide Own Effect Particles", options: ONOFF, default: 0 },
    Field { key: "bloom", label: "Bloom", options: ONOFF, default: 0 },
    Field { key: "bloom_amount", label: "Strength", options: AMOUNTS, default: 0 },
    Field { key: "world_look", label: "World Look", options: ONOFF, default: 0 },
    Field { key: "world_look_kind", label: "Look", options: LOOKS, default: 0 },
    Field { key: "world_colors", label: "Sky & World Colors", options: ONOFF, default: 0 },
    Field { key: "world_tint", label: "Colour", options: TINTS, default: 0 },
    Field { key: "motion_blur", label: "Motion Blur", options: ONOFF, default: 0 },
    Field { key: "motion_amount", label: "Strength", options: AMOUNTS, default: 1 },
    Field { key: "mist", label: "Mist", options: ONOFF, default: 0 },
    Field { key: "mist_color", label: "Colour", options: MIST_COLORS, default: 0 },
    Field { key: "sky_effects", label: "Sky Effects", options: ONOFF, default: 0 },
    Field { key: "ambience", label: "Ambience", options: ONOFF, default: 0 },
    Field { key: "combat_hitboxes", label: "Combat Hitboxes", options: ONOFF, default: 0 },
    Field { key: "auto_hitbox", label: "Auto Hitbox", options: ONOFF, default: 0 },
    Field { key: "retotem_timer", label: "Re-totem Timer", options: ONOFF, default: 0 },
    Field { key: "shield_helper", label: "Shield Helper", options: ONOFF, default: 0 },
    Field { key: "eating_warning", label: "Enemy Eating Warning", options: ONOFF, default: 0 },
    Field { key: "item_names", label: "Dropped Item Names", options: ONOFF, default: 0 },
    Field { key: "bow_charge", label: "Bow Charge %", options: ONOFF, default: 0 },
    Field { key: "mace_aim", label: "Mace Aim", options: ONOFF, default: 0 },
    Field { key: "mace_optimizer", label: "Mace Optimizer", options: ONOFF, default: 0 },
    Field { key: "crystal_preview", label: "Crystal Preview", options: ONOFF, default: 0 },
    Field { key: "crystal_optimizer", label: "Crystal Optimizer", options: ONOFF, default: 0 },
    Field { key: "chat_hotkeys", label: "Chat Hotkeys", options: ONOFF, default: 0 },
    Field { key: "friends", label: "Friends", options: ONOFF, default: 1 },
    Field { key: "session_summary", label: "Session Summary", options: ONOFF, default: 0 },
    Field { key: "session_goals", label: "Session Goals", options: ONOFF, default: 0 },
    Field { key: "goal_kills", label: "Goal", options: GOALS, default: 0 },
    Field { key: "opponent_intel", label: "Opponent Intel", options: ONOFF, default: 0 },
    Field { key: "pearl_timer", label: "Enemy Pearl Timer", options: ONOFF, default: 0 },
];

/// The Java modules Horizon runs here: (Java module id, the setting that switches it, its other settings).
pub const ON_HORIZON: &[(&str, usize, &[usize])] = &[
    ("info_hud", INFO, &[FPS, CPS, CORNER]),
    ("coordinates", COORDS, &[CORNER]),
    ("compass", FACING, &[CORNER]),
    ("speed_display", SPEED, &[CORNER]),
    ("session_clock", SESSION, &[CORNER]),
    ("fps_graph", GRAPH, &[CORNER]),
    ("hud_theme", THEME, &[ACCENT, THEME_TOOLTIPS]),
    ("custom_crosshair", CROSSHAIR, &[]),
    ("boss_bar_tweaks", HIDE_BOSS, &[]),
    ("title_tweaks", HIDE_TITLES, &[]),
    ("scoreboard_tweaks", HIDE_SCORE, &[]),
    ("zoom", ZOOM_ON, &[ZOOM, SMOOTH]),
    ("view_model", VIEW, &[ITEM]),
    ("camera_tweaks", CAMERA, &[DISTANCE]),
    ("no_fog", NO_FOG, &[FOG]),
    ("hide_nametags", HIDE_TAGS, &[TAGS_WHO]),
    ("block_outline", OUTLINE, &[OUTLINE_COLOR, OUTLINE_WIDTH]),
    ("toggle_sprint", SNEAK, &[]),
    ("health_hud", HEALTH, &[HEALTH_SHOW, HEALTH_UNIT]),
    ("low_health_warning", LOW_HEALTH, &[LOW_HEALTH_AT, HEARTBEAT]),
    ("better_tooltips", TOOLTIPS, &[]),
    ("shulker_preview", SHULKER, &[]),
    ("food_preview", SATURATION, &[]),
    ("armor_hud", ARMOR, &[ARMOR_SHOW]),
    ("item_counter", COUNTER, &[]),
    ("offhand_totem_warning", OFFHAND_WARN, &[]),
    ("durability_warning", DURA_WARN, &[DURA_WARN_AT]),
    ("ready_check", READY, &[]),
    ("kill_message", KILL_MSG, &[KILL_TEXT]),
    ("death_message", DEATH_MSG, &[DEATH_TEXT]),
    ("mention_ping", MENTION, &[]),
    ("combat_tag_timer", COMBAT_TAG, &[]),
    ("totem_pop_counter", POPS, &[]),
    ("totem_pop_alerts", POP_ALERTS, &[]),
    ("target_hud", TARGET, &[]),
    ("hitmarker", HITMARKER, &[]),
    ("kill_streaks", STREAKS, &[]),
    ("nametag_health", NAMETAG_HP, &[]),
    ("damage_numbers", DAMAGE_NUMS, &[]),
    ("jump_circles", JUMP_RINGS, &[JUMP_COLOR]),
    ("potion_timers", POTIONS, &[]),
    ("potion_warning", POTION_WARN, &[]),
    ("attack_bar", ATTACK_BAR, &[]),
    ("crit_indicator", CRIT_IND, &[]),
    ("nametag_ping", NAMETAG_PING, &[]),
    ("lag_meter", LAG, &[]),
    ("clock_info", CLOCK, &[]),
    ("session_stats", SESSION_STATS, &[]),
    ("fight_timer", FIGHT_TIMER, &[]),
    ("enemy_armor_alert", ENEMY_ARMOR, &[]),
    ("target_low_hp", LOW_HP_ALERT, &[]),
    ("inventory_totem", INV_TOTEM, &[]),
    ("combo_reach", COMBO_REACH, &[]),
    ("combo_pop", COMBO_POP, &[]),
    ("death_recap", DEATH_RECAP, &[]),
    ("notification_feed", FEED, &[]),
    ("fight_summary", FIGHT_SUMMARY, &[]),
    ("shield_break_sound", SHIELD_SOUND, &[]),
    ("dynamic_crosshair", DYN_CROSSHAIR, &[]),
    ("cooldown_hud", COOLDOWN, &[]),
    ("tnt_timer", TNT, &[]),
    ("anchor_helper", ANCHOR, &[]),
    ("crystal_damage_preview", CRYSTAL_DMG, &[]),
    ("fall_preview", FALL, &[]),
    ("chat_filter", CHAT_FILTER, &[FILTER_KIND]),
    ("fullbright", FULLBRIGHT, &[]),
    ("hit_effects", HIT_FX, &[HIT_COLOR]),
    ("low_fire", LOW_FIRE, &[FIRE_LOWER]),
    ("particle_limiter", PARTICLES, &[PARTICLE_KEEP]),
    ("weather_time", WEATHER_TIME, &[VIEW_TIME, VIEW_WEATHER]),
    ("no_hurt_cam", NO_HURT, &[]),
    ("no_pumpkin_overlay", NO_PUMPKIN, &[]),
    ("smaller_pvp", SMALLER, &[SHIELD_SIZE, TOTEM_SIZE]),
    ("enchant_glint", GLINT, &[GLINT_COLOR]),
    ("discord_presence", DISCORD, &[]),
    ("pvp_tiers", TIERS, &[]),
    ("arsenal_users", USERS, &[]),
    ("crit_particles", CRITS, &[CRIT_SHAPE, CRIT_COLOR]),
    ("sweep_effect", SWEEP, &[SWEEP_COLOR]),
    ("swing_trails", SWING, &[]),
    ("totem_pop_style", TOTEM_STYLE, &[TOTEM_FLASH, TOTEM_HIDE]),
    ("kill_effects", KILL_FX, &[KILL_STYLE]),
    ("hide_own_particles", HIDE_OWN_FX, &[]),
    ("bloom", BLOOM, &[BLOOM_AMT]),
    ("world_look", LOOK, &[LOOK_KIND]),
    ("world_colors", COLORS, &[WORLD_TINT]),
    ("motion_blur", MOTION, &[MOTION_AMT]),
    ("mist", MIST, &[MIST_COLOR]),
    ("sky_effects", SKY_FX, &[]),
    ("ambience", AMBIENCE, &[]),
    ("combat_hitboxes", HITBOXES, &[]),
    ("auto_hitbox", AUTO_HITBOX, &[]),
    ("retotem_timer", RETOTEM, &[]),
    ("shield_helper", SHIELD_HELP, &[]),
    ("eating_warning", EATING, &[]),
    ("item_names", ITEM_NAMES, &[]),
    ("bow_charge", BOW, &[]),
    ("mace_aim", MACE_AIM, &[]),
    ("mace_optimizer", MACE_OPT, &[]),
    ("crystal_preview", CRYSTAL_PREV, &[]),
    ("crystal_optimizer", CRYSTAL_OPT, &[]),
    ("chat_hotkeys", HOTKEYS, &[]),
    ("friends", FRIENDS, &[]),
    ("session_summary", SUMMARY, &[]),
    ("session_goals", GOAL, &[GOAL_KILLS]),
    ("opponent_intel", INTEL, &[]),
    ("pearl_timer", PEARLS, &[]),
];

/// What a module does on Horizon when that differs from the Java mod.
pub fn horizon_note(id: &str) -> Option<&'static str> {
    Some(match id {
        "custom_crosshair" => "Pick a crosshair in Browse > Crosshairs; it replaces the vanilla one.",
        "hud_theme" => "Pick a theme in Browse > Themes, or one of the accents here.",
        "boss_bar_tweaks" => "On Horizon: hides the boss bar.",
        "title_tweaks" => "On Horizon: hides titles, subtitles and the action bar.",
        "scoreboard_tweaks" => "On Horizon: hides the scoreboard sidebar.",
        "toggle_sprint" => "On Horizon: press sneak once to keep sneaking, again to stop. Horizon doesn't let mods hold sprint yet.",
        "no_fog" => "On Horizon: less or no distance fog in the open air; water and lava keep theirs.",
        "camera_tweaks" => "On Horizon: the third-person camera's distance.",
        "view_model" => "On Horizon: the size of the item in your hand.",
        "hide_nametags" => "On Horizon: hides name tags above players (or mobs too).",
        "health_hud" => "On Horizon: your health under the crosshair as a number, a bar or both, green to red, absorption in gold.",
        "low_health_warning" => "On Horizon: red pulsing screen edges and a heartbeat when your health is low.",
        "armor_hud" => "On Horizon: your armour and what's in your hands right of the hotbar, with their durability.",
        "kill_message" => "On Horizon: sends your message after a kill the server's death message credits to you. Pick it from the list ({name} is who you killed).",
        "death_message" => "On Horizon: sends your message when you die ({name} is who killed you, when the death message says).",
        "mention_ping" => "On Horizon: a sound and a note on screen when someone writes your name in chat.",
        "combat_tag_timer" => "On Horizon: a countdown when the server says you're in combat, gone when it says you're out.",
        "totem_pop_counter" => "On Horizon: how many totems each player near you popped, top right.",
        "totem_pop_alerts" => "On Horizon: a chat line and a sound when a player near you pops a totem.",
        "target_hud" => "On Horizon: the player you look at: health, absorption, armour, held item and totem pops.",
        "hitmarker" => "On Horizon: an X round the crosshair when your hit lands on a player.",
        "kill_streaks" => "On Horizon: Double Kill, Triple Kill... with a sound, for kills within 10 seconds.",
        "nametag_health" => "On Horizon: each player's health above their head, green to red.",
        "damage_numbers" => "On Horizon: the damage a player takes floats up from them.",
        "jump_circles" => "On Horizon: a ring spreads out at your feet when you jump.",
        "potion_timers" => "On Horizon: your effects and the time left on each, at the left.",
        "potion_warning" => "On Horizon: a sound and a warning 5 seconds before a good effect (Strength, Speed...) runs out.",
        "attack_bar" => "On Horizon: your attack charge as a bar under the crosshair.",
        "crit_indicator" => "On Horizon: a gold mark at the crosshair when a hit now would be a critical hit.",
        "nametag_ping" => "On Horizon: each player's ping above their head.",
        "lag_meter" => "On Horizon: your ping, coloured green to red.",
        "clock_info" => "On Horizon: the time and your ping, top left.",
        "session_stats" => "On Horizon: kills, deaths, K/D, totems you popped and time played this session.",
        "fight_timer" => "On Horizon: how long the current fight has lasted.",
        "enemy_armor_alert" => "On Horizon: a warning when a piece of a nearby enemy's armour is about to break.",
        "target_low_hp" => "On Horizon: a flash and a sound when the player you fight is low.",
        "inventory_totem" => "On Horizon: how many totems you have, next to the crosshair.",
        "combo_reach" => "On Horizon: your combo (hits without being hit) and the reach of your last hit.",
        "combo_pop" => "On Horizon: your combo number pops up with each hit.",
        "death_recap" => "On Horizon: when you die, who killed you and the damage you took in the last 10 seconds.",
        "notification_feed" => "On Horizon: kills, deaths and pops in a short feed, top right.",
        "fight_summary" => "On Horizon: a short card after a fight: hits, damage dealt, pops and how long it took.",
        "shield_break_sound" => "On Horizon: a sound when a player's shield near you is broken.",
        "dynamic_crosshair" => "On Horizon: a red ring round the crosshair when you aim at a player.",
        "cooldown_hud" => "On Horizon: ender pearl, chorus fruit, wind charge, shield and mace cooldowns above the hotbar.",
        "tnt_timer" => "On Horizon: the seconds left on each lit TNT, above it.",
        "anchor_helper" => "On Horizon: a respawn anchor's charges when you aim at it.",
        "crystal_damage_preview" => "On Horizon: about how much an end crystal near you would hit you (and the player you fight) for, above it.",
        "fall_preview" => "On Horizon: the fall damage you'd take landing now, under the crosshair.",
        "chat_filter" => "On Horizon: hides join and leave lines, ads and links (or both) from chat.",
        "fullbright" => "On Horizon: full brightness everywhere (your brightness comes back when it's off).",
        "hit_effects" => "On Horizon: hit players and mobs flash in your colour instead of red (Horizon's own hit colour).",
        "low_fire" => "On Horizon: the fire overlay when you burn sits lower, so you can see.",
        "particle_limiter" => "On Horizon: fewer particles (or none), for more frames in big fights.",
        "weather_time" => "On Horizon: your own time of day and weather, only on your screen.",
        "no_hurt_cam" => "On Horizon: no camera tilt when you're hurt.",
        "no_pumpkin_overlay" => "On Horizon: no pumpkin blur when you wear a carved pumpkin.",
        "smaller_pvp" => "On Horizon: a smaller shield in your hand and a smaller totem animation.",
        "enchant_glint" => "On Horizon: the enchantment glint in your colour.",
        "discord_presence" => "On Horizon: your Discord profile shows you play Horizon with Arsenal (through the Arsenal Launcher, so apps like Medal don't take its place).",
        "pvp_tiers" => "On Horizon: each player's best PvP tier (pvptiers.com) above their head.",
        "arsenal_users" => "On Horizon: a gold A above players who use Arsenal too.",
        "crit_particles" => "On Horizon: critical hits as stars, hearts, diamonds or circles in your colour, instead of vanilla's.",
        "sweep_effect" => "On Horizon: the sweeping edge slash as a coloured arc instead of vanilla's.",
        "swing_trails" => "On Horizon: a short streak follows each swing of your weapon.",
        "totem_pop_style" => "On Horizon: a coloured flash when your totem pops, and totem particles hidden if you like (the animation size is in Smaller PvP).",
        "kill_effects" => "On Horizon: a burst, ring or lightning where the player you killed was.",
        "hide_own_particles" => "On Horizon: your own potion effect particles are hidden (other players' still show).",
        "bloom" | "world_look" | "world_colors" | "motion_blur" | "mist" | "sky_effects" =>
            "On Horizon: drawn by Arsenal's shader pack. Pick Arsenal once in Options > Video Settings > Shaders > Shader Packs.",
        "ambience" => "On Horizon: fireflies at night, embers in the Nether and drifting specks by day around you.",
        "combat_hitboxes" => "On Horizon: players' hitboxes; the one you aim at turns red when a full hit would land.",
        "auto_hitbox" => "On Horizon: Combat Hitboxes only while you hold a weapon.",
        "retotem_timer" => "On Horizon: how fast a new totem was in your offhand after a pop.",
        "shield_helper" => "On Horizon: a red mark above a player whose shield an axe just disabled, green while they hold a usable one.",
        "eating_warning" => "On Horizon: EATING GAPPLE / EATING / DRINKING above players while they eat or drink.",
        "item_names" => "On Horizon: dropped items show their name and count; valuable loot is gold.",
        "bow_charge" => "On Horizon: your bow, crossbow or trident charge in percent while you draw.",
        "mace_aim" => "On Horizon: while falling with a mace, a line to the player below and your bonus damage.",
        "mace_optimizer" => "On Horizon: the mace smash sound plays the moment your hit lands.",
        "crystal_preview" => "On Horizon: a green box where a held crystal would go on obsidian or bedrock, red where it can't.",
        "crystal_optimizer" => "On Horizon: crystals you hit disappear at once instead of waiting for the server.",
        "chat_hotkeys" => "On Horizon: four keys (Options > Mods > Arsenal) that send gg, gl hf, /spawn and /kit.",
        "friends" => "On Horizon: middle-click a player (the Friend key) to mark them a friend: a green F above them, never your target.",
        "session_summary" => "On Horizon: when you join again, a card with your last session: time, kills, deaths, K/D, pops and best combo.",
        "session_goals" => "On Horizon: a kill goal for this session and how close you are.",
        "opponent_intel" => "On Horizon: during a fight, what you saw the player use: totems popped, golden apples eaten, shield breaks.",
        "pearl_timer" => "On Horizon: a purple countdown above a player who just threw an ender pearl.",
        "item_counter" => "On Horizon: totems, golden apples, pearls, crystals, obsidian and anchors you carry, left of the hotbar.",
        "offhand_totem_warning" => "On Horizon: a warning above the hotbar when your offhand has no totem but you still carry some.",
        "durability_warning" => "On Horizon: a flashing warning and a sound when a piece of armour is about to break.",
        "ready_check" => "On Horizon: offhand totem, full armour with durability left, golden apples and pearls; green when you're ready.",
        "better_tooltips" => "On Horizon: durability as numbers (green/yellow/red) and what food restores, in item tooltips.",
        "shulker_preview" => "On Horizon: hovering a shulker box shows its contents as a grid in the shulker's colour.",
        "food_preview" => "On Horizon: your saturation as a gold number next to the hunger bar (Horizon doesn't show mods the food in your hand yet).",
        _ => return None,
    })
}

/// Why a Java module doesn't run on Horizon yet: what Horizon's mod hooks lack for it.
pub fn why_not(id: &str) -> &'static str {
    const ACCOUNT: &[&str] = &["arsenal_messages", "arsenal_users", "friends", "party_hud", "pvp_tiers", "cloud_backup", "wardrobe_kill_effects", "kill_effects", "album_colors", "spotify_lyrics", "discord_presence", "session_goals", "quests", "usage_stats", "crash_reports", "update_notice", "server_profiles", "server_notes", "blade_settings", "duel_record"];
    const PLAYERS: &[&str] = &["nametag_ping", "enemy_armor_alert", "eating_warning", "pearl_timer", "target_low_hp", "opponent_intel", "tab_ping", "nametag_style", "combo_reach", "fight_timer", "fight_summary", "death_recap", "session_stats", "shield_break_sound", "combo_pop", "sprint_indicator", "session_summary"];
    const SELF: &[&str] = &["potion_timers", "cooldown_hud", "inventory_totem", "potion_warning", "retotem_timer",  "bow_charge", "attack_bar", "crit_indicator", "hotbar_style", "inventory_style", "inventory_cursor_speed", "quick_menu", "lag_meter", "clock_info", "notification_feed", "fall_preview"];
    const CHAT: &[&str] = &["chat_filter", "chat_hotkeys", "chat_mentions", "chat_tweaks", "chat_copy", "name_hider", "auto_reconnect", "hypixel_skyblock", "screenshot_clipboard"];
    if ACCOUNT.contains(&id) {
        "Needs your Arsenal account or another app, which Horizon doesn't pass to mods yet."
    } else if PLAYERS.contains(&id) {
        "Horizon doesn't tell mods about other players' health, hits, items or effects yet."
    } else if SELF.contains(&id) {
        "Horizon doesn't tell mods about your inventory, health, effects or the server yet."
    } else if CHAT.contains(&id) {
        "Horizon doesn't let mods read or change chat, server messages or connections yet."
    } else {
        "Horizon can't let mods draw or change this in the world yet."
    }
}

/// What Horizon runs of the Java module `id`, if it runs it.
pub fn on_horizon(id: &str) -> Option<(usize, &'static [usize])> {
    ON_HORIZON.iter().find(|m| m.0 == id).map(|m| (m.1, m.2))
}

const ZOOM_MIN: f32 = 1.25;
/// No real limit: the wheel keeps zooming in. This only keeps the view from breaking (a field of view of a
/// hundredth of a degree is still drawn fine).
const ZOOM_MAX: f32 = 10_000.0;
/// Each wheel step multiplies the zoom by this.
const ZOOM_STEP: f32 = 1.25;

/// A drawn crosshair from Browse: its grid (`#` drawn, `.` not), colour (0xRRGGBB) and pixel size.
#[derive(Clone, Debug, PartialEq)]
pub struct Crosshair {
    pub rows: Vec<String>,
    pub rgb: u32,
    pub size: u32,
}

/// A HUD theme from Browse: panel background (0xAARRGGBB), text and accent (0xRRGGBB).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Theme {
    pub background: u32,
    pub text: u32,
    pub accent: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    /// Each field's choice, as `FIELDS`.
    pub v: [usize; N],
    /// What Browse applied.
    pub theme: Option<Theme>,
    pub crosshair: Option<Crosshair>,
}

impl Default for Settings {
    fn default() -> Self {
        let mut v = [0; N];
        for (i, f) in FIELDS.iter().enumerate() {
            v[i] = f.default;
        }
        Self { v, theme: None, crosshair: None }
    }
}

/// "#AARRGGBB" or "#RRGGBB" (opaque) as 0xAARRGGBB.
pub fn parse_color(text: &str) -> Option<u32> {
    let hex = text.trim().trim_start_matches('#');
    let v = u32::from_str_radix(hex, 16).ok()?;
    match hex.len() {
        6 => Some(0xFF00_0000 | v),
        8 => Some(v),
        _ => None,
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
        s.theme = get("browse_theme").and_then(|t| {
            let c: Vec<u32> = t.split(',').filter_map(parse_color).collect();
            (c.len() == 3).then(|| Theme { background: c[0], text: c[1] & 0xFF_FFFF, accent: c[2] & 0xFF_FFFF })
        });
        s.crosshair = get("browse_crosshair").and_then(|t| {
            let mut parts = t.trim().split(';');
            let rows: Vec<String> = parts.next()?.split('/').map(str::to_owned).collect();
            let rgb = parse_color(parts.next()?)? & 0xFF_FFFF;
            let size = parts.next()?.parse().ok()?;
            crosshair_ok(&rows).then_some(Crosshair { rows, rgb, size })
        });
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
        let mut out: Vec<(&'static str, String)> = FIELDS.iter().enumerate().map(|(i, f)| (f.key, f.options[self.v[i]].0.to_owned())).collect();
        out.push(("browse_theme", self.theme.map_or(String::new(), |t| format!("#{:08X},#{:06X},#{:06X}", t.background, t.text, t.accent))));
        out.push(("browse_crosshair", self.crosshair.as_ref().map_or(String::new(), |c| format!("{};#{:06X};{}", c.rows.join("/"), c.rgb, c.size))));
        out
    }

    fn number(&self, field: usize) -> f32 {
        FIELDS[field].options[self.v[field]].0.parse().unwrap_or(1.0)
    }

    fn browse_theme(&self) -> Option<Theme> {
        if self.on(THEME) && self.v[ACCENT] == ACCENTS.len() - 1 { self.theme } else { None }
    }

    /// The panel's and the menu's accent (0xRRGGBB): HUD Theme's, else gold.
    pub fn accent(&self) -> u32 {
        match self.browse_theme() {
            Some(t) => t.accent,
            None if self.on(THEME) => ACCENT_RGB.get(self.v[ACCENT]).copied().unwrap_or(ACCENT_RGB[0]),
            None => ACCENT_RGB[0],
        }
    }

    /// The panel's background (0xAARRGGBB) and value text (0xRRGGBB).
    pub fn panel_colors(&self) -> (u32, u32) {
        self.browse_theme().map_or((BACKGROUND, WHITE), |t| (t.background, t.text))
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

    /// The environmental fog's start and end for vanilla's, pushed away by No Fog. Only ever further, and end stays
    /// past start, as Horizon requires.
    pub fn fog_range(&self, start: f32, end: f32) -> (f32, f32) {
        match (self.on(NO_FOG), self.v[FOG]) {
            (true, 0) => (start * 2.0, (end * 2.0).max(start * 2.0 + 1.0)),
            (true, _) => (1.0e6, 1.0e6 + 1.0),
            _ => (start, end),
        }
    }

    pub fn zoom_level(&self) -> f32 {
        self.number(ZOOM)
    }

    /// Whether the HUD layer `layer` (`minecraft:boss_overlay`…) is hidden.
    pub fn hides_layer(&self, layer: &str) -> bool {
        let name = layer.rsplit(':').next().unwrap_or(layer);
        match name {
            "boss_overlay" => self.on(HIDE_BOSS),
            "title" | "subtitle_overlay" | "overlay_message" => self.on(HIDE_TITLES),
            "scoreboard_sidebar" => self.on(HIDE_SCORE),
            "crosshair" => self.on(CROSSHAIR) && self.crosshair.is_some(),
            _ => false,
        }
    }

    /// Whether the name tag of an entity of type `kind` is hidden (Hide Nametags).
    pub fn hides_tag(&self, kind: &str) -> bool {
        self.on(HIDE_TAGS) && (self.v[TAGS_WHO] == 1 || kind == "minecraft:player")
    }

    /// The block outline's colour (0xAARRGGBB) and width, when Block Outline is on.
    pub fn outline(&self) -> Option<(u32, f32)> {
        if !self.on(OUTLINE) {
            return None;
        }
        let rgb = if self.v[OUTLINE_COLOR] == 0 { self.accent() } else { OUTLINE_RGB[self.v[OUTLINE_COLOR] - 1] };
        Some((0xFF00_0000 | rgb, self.number(OUTLINE_WIDTH)))
    }
}

/// A grid Browse can draw: 1 to 128 rows of `#` and `.`, at most 128 long, with something drawn.
fn crosshair_ok(rows: &[String]) -> bool {
    !rows.is_empty() && rows.len() <= 128 && rows.iter().all(|r| r.len() <= 128 && r.chars().all(|c| c == '#' || c == '.')) && rows.iter().any(|r| r.contains('#'))
}

/// A crosshair's drawn runs, centred on `center`, each cell `cell` GUI pixels: one rectangle per run of `#` in a row.
pub fn crosshair_shapes(c: &Crosshair, center: [f32; 2], cell: f32) -> Vec<Shape> {
    let h = c.rows.len() as f32;
    let w = c.rows.iter().map(|r| r.len()).max().unwrap_or(0) as f32;
    let (x0, y0) = (center[0] - w * cell / 2.0, center[1] - h * cell / 2.0);
    let mut out = Vec::new();
    for (y, row) in c.rows.iter().enumerate() {
        let bytes = row.as_bytes();
        let mut x = 0;
        while x < bytes.len() {
            if bytes[x] == b'#' {
                let start = x;
                while x < bytes.len() && bytes[x] == b'#' {
                    x += 1;
                }
                out.push(Shape::Fill([x0 + start as f32 * cell, y0 + y as f32 * cell, (x - start) as f32 * cell, cell], 0xFF00_0000 | c.rgb));
            } else {
                x += 1;
            }
        }
    }
    out
}

/// What the Right Shift menu shows.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Page {
    #[default]
    Home,
    Modules,
    Browse,
    Cosmetics,
}

/// The Right Shift menu's own state.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Menu {
    pub page: Page,
    pub category: usize,
    /// The first module row shown (the wheel moves it).
    pub scroll: usize,
    /// The module whose settings are open (index in `JAVA_MODULES`).
    pub detail: Option<usize>,
    /// Browse: the tab (index in `BROWSE_TABS`), the first row of cards shown, and what was just applied.
    pub tab: usize,
    pub browse_scroll: usize,
    pub notice: Option<String>,
}

#[derive(Default)]
struct State {
    /// Read once, then kept; the settings pages replace it.
    settings: Option<Settings>,
    /// The zoom while the key is held; None when it is up.
    held: Option<f32>,
    menu: Menu,
    /// Speed Display and Session Clock: the last position sampled and when, the speed measured, and when the player
    /// came into this world.
    sample: Option<(Instant, [f64; 3])>,
    speed: f64,
    joined: Option<Instant>,
    /// FPS Graph: one reading every quarter second, the newest last.
    fps: VecDeque<u32>,
    fps_at: Option<Instant>,
    /// Toggle Sneak: the sneak key as last read, and whether sneaking is kept on.
    sneak_down: bool,
    sneaking: bool,
    /// The block the outline is drawn on this frame.
    outlined: Option<[i32; 3]>,
    /// The menu's close button was clicked: closed on the next tick (the click hook can't close screens).
    close: bool,
    /// Health HUD: the player's health as the health rows last saw it, with the screen's GUI size and when.
    health: Option<(Health, Instant)>,
    /// Low Health Warning: when the heartbeat last played.
    heartbeat: Option<Instant>,
    /// Durability Warning: the pieces already warned about (the sound plays once for each).
    warned: Vec<&'static str>,
    /// The combat modules' state (chat, other players, the world).
    pvp: Pvp,
    /// The visuals Arsenal set (to give each back once when its module goes off), and the brightness before Fullbright.
    applied: [bool; 8],
    saved_gamma: Option<f32>,
    /// The block aimed at this frame.
    aimed: Option<[i32; 3]>,
    /// Arsenal's own particles: where, which (1 crit, 2 sweep), since when, which way.
    fx: Vec<([f64; 3], u8, Instant, [f64; 3])>,
    kill_fx: Vec<([f64; 3], Instant)>,
    totem_flash: Option<Instant>,
    presence_at: Option<Instant>,
    extra: Extra,
    pack_done: bool,
    in_world: bool,
}

/// The player's health as the health rows see it (health hook), and the screen's GUI size then.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub health: f32,
    pub max: f32,
    pub absorption: f32,
    pub food: i32,
    pub saturation: f32,
    pub width: f32,
    pub height: f32,
}

/// The health rows were laid out this frame (they aren't under F1, on death screens or in spectator mode).
pub fn set_health(health: Health) {
    STATE.with_borrow_mut(|s| s.health = Some((health, Instant::now())));
}

/// The last health seen, while it's fresh (the rows stop being drawn under F1 and then it hides too).
pub fn health() -> Option<Health> {
    STATE.with_borrow(|s| s.health.filter(|(_, at)| at.elapsed() < Duration::from_millis(250)).map(|(h, _)| h))
}

/// Green when full, yellow at half, red when low (0xRRGGBB).
pub fn health_rgb(fraction: f32) -> u32 {
    let f = fraction.clamp(0.0, 1.0);
    let (r, g) = if f > 0.5 { ((1.0 - f) * 2.0, 1.0) } else { (1.0, f * 2.0) };
    ((r * 255.0).round() as u32) << 16 | ((g * 220.0).round() as u32) << 8 | 0x40
}

/// "7.5", "8", "13/20": one decimal only when it isn't whole.
pub fn health_text(value: f32, max: f32, units_points: bool) -> String {
    let _ = max;
    let v = if units_points { value } else { value / 2.0 };
    let tidy = |x: f32| {
        let r = (x * 2.0).round() / 2.0;
        if (r - r.round()).abs() < 1e-3 { format!("{}", r.round() as i32) } else { format!("{r:.1}") }
    };
    tidy(v)
}

/// A stack as the inventory modules read it: its id, count and damage taken.
#[derive(Clone, Debug, PartialEq)]
pub struct Stack {
    pub id: String,
    pub count: u32,
    pub damage: u32,
}

/// The player's inventory (Arsenal's inventory hook): 36 slots (hotbar first), armour head first, the offhand.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Inventory {
    pub slots: Vec<Option<Stack>>,
    pub armor: [Option<Stack>; 4],
    pub offhand: Option<Stack>,
    pub selected: usize,
}

impl Inventory {
    pub fn main_hand(&self) -> Option<&Stack> {
        self.slots.get(self.selected).and_then(Option::as_ref)
    }

    /// How many of `id` ("minecraft:totem_of_undying") the player carries, the offhand included.
    pub fn count(&self, id: &str) -> u32 {
        self.slots.iter().chain(std::iter::once(&self.offhand)).flatten().filter(|s| s.id == id).map(|s| s.count).sum()
    }
}

/// How much of a stack's durability is left (0..1), or None for an item without durability.
pub fn durability_left(stack: &Stack) -> Option<f32> {
    let max = max_durability(&stack.id)?;
    Some(max.saturating_sub(stack.damage) as f32 / max as f32)
}

const TOTEM: &str = "minecraft:totem_of_undying";
/// Item Counter's items, in the Java mod's order.
pub const COUNTED: [&str; 7] = [TOTEM, "minecraft:golden_apple", "minecraft:enchanted_golden_apple", "minecraft:ender_pearl",
    "minecraft:end_crystal", "minecraft:obsidian", "minecraft:respawn_anchor"];

/// Armor HUD: armour (head first) and the hands' items with durability, in a column right of the hotbar.
pub fn armor_shapes(settings: &Settings, inv: &Inventory, w: f32, h: f32, width: &dyn Fn(&str) -> f32) -> Vec<Shape> {
    let mut out = Vec::new();
    if !settings.on(ARMOR) {
        return out;
    }
    let pieces: Vec<&Stack> = inv.armor.iter().flatten().chain(inv.main_hand().filter(|s| max_durability(&s.id).is_some()))
        .chain(inv.offhand.as_ref().filter(|s| max_durability(&s.id).is_some())).collect();
    let x = (w / 2.0).floor() + 98.0;
    let mut y = h - 18.0 - 17.0 * (pieces.len() as f32 - 1.0);
    for stack in pieces {
        out.push(Shape::Item(stack.id.clone(), [x, y]));
        if let Some(left) = durability_left(stack) {
            let text = match settings.v[ARMOR_SHOW] {
                0 => format!("{}%", (left * 100.0).round() as i32),
                1 => format!("{}", max_durability(&stack.id).unwrap_or(0).saturating_sub(stack.damage)),
                _ => String::new(),
            };
            if !text.is_empty() {
                let _ = width(&text);
                out.push(Shape::Text(text, [x + 18.0, y + 4.0], health_rgb(left), true));
            }
        }
        y += 17.0;
    }
    out
}

/// Item Counter: each counted item the player has, with how many, in a column left of the hotbar.
pub fn counter_shapes(settings: &Settings, inv: &Inventory, w: f32, h: f32, width: &dyn Fn(&str) -> f32) -> Vec<Shape> {
    let mut out = Vec::new();
    if !settings.on(COUNTER) {
        return out;
    }
    let found: Vec<(&str, u32)> = COUNTED.iter().map(|id| (*id, inv.count(id))).filter(|(_, n)| *n > 0).collect();
    let right = (w / 2.0).floor() - 98.0;
    let mut y = h - 18.0 - 17.0 * (found.len() as f32 - 1.0);
    for (id, n) in found {
        let text = n.to_string();
        let tw = width(&text);
        out.push(Shape::Item(id.to_owned(), [right - 16.0, y]));
        out.push(Shape::Text(text, [right - 18.0 - tw, y + 4.0], 0xFFFFFF, true));
        y += 17.0;
    }
    out
}

/// Offhand Totem Warning: no totem in the offhand while there are some in the inventory.
pub fn offhand_warning(settings: &Settings, inv: &Inventory) -> Option<String> {
    let carried = inv.count(TOTEM);
    let held = inv.offhand.as_ref().is_some_and(|s| s.id == TOTEM);
    (settings.on(OFFHAND_WARN) && !held && carried > 0).then(|| format!("No totem in your offhand! ({carried} left)"))
}

/// Durability Warning: the armour pieces (names) at or under the setting's durability left.
pub fn worn_out(settings: &Settings, inv: &Inventory) -> Vec<&'static str> {
    if !settings.on(DURA_WARN) {
        return Vec::new();
    }
    let at: f32 = DURA_AT[settings.v[DURA_WARN_AT]].0.parse::<f32>().unwrap_or(10.0) / 100.0;
    ["helmet", "chestplate", "leggings", "boots"].iter().zip(&inv.armor)
        .filter(|(_, s)| s.as_ref().and_then(durability_left).is_some_and(|left| left <= at))
        .map(|(name, _)| *name).collect()
}

/// Whether the durability warning's sound is due: once for each piece that newly wore out.
pub fn durability_sound_due(worn: &[&'static str]) -> bool {
    STATE.with_borrow_mut(|s| {
        let new = worn.iter().any(|p| !s.warned.contains(p));
        s.warned = worn.to_vec();
        new
    })
}

/// The warnings above the hotbar (offhand totem, durability), flashing, centred.
pub fn warning_shapes(settings: &Settings, inv: &Inventory, w: f32, h: f32, millis: u64, width: &dyn Fn(&str) -> f32) -> Vec<Shape> {
    let mut lines: Vec<(String, u32)> = Vec::new();
    if let Some(text) = offhand_warning(settings, inv) {
        lines.push((text, 0xFF5A5A));
    }
    let worn = worn_out(settings, inv);
    if !worn.is_empty() {
        let flash = (millis / 400) % 2 == 0;
        lines.push((format!("Your {} {} about to break!", worn.join(" and "), if worn.len() == 1 { "is" } else { "are" }),
                    if flash { 0xFFC83D } else { 0xFF5A5A }));
    }
    let mut y = h - 72.0;
    let mut out = Vec::new();
    for (text, rgb) in lines.into_iter().rev() {
        let tw = width(&text);
        out.push(Shape::Text(text, [((w - tw) / 2.0).floor(), y], rgb, true));
        y -= 11.0;
    }
    out
}

/// Ready Check: (what, ready) for each check, in the Java mod's order.
pub fn ready_checks(inv: &Inventory) -> Vec<(&'static str, bool)> {
    let armor_ok = inv.armor.iter().all(|s| s.as_ref().is_some_and(|s| durability_left(s).is_none_or(|l| l > 0.25)));
    vec![
        ("Offhand totem", inv.offhand.as_ref().is_some_and(|s| s.id == TOTEM)),
        ("Full armour", armor_ok),
        ("Golden apples", inv.count("minecraft:golden_apple") + inv.count("minecraft:enchanted_golden_apple") > 0),
        ("Ender pearls", inv.count("minecraft:ender_pearl") > 0),
    ]
}

/// Ready Check's panel, at the left edge in the middle: a tick or cross per check, green when all are ready.
pub fn ready_shapes(settings: &Settings, inv: &Inventory, h: f32, width: &dyn Fn(&str) -> f32) -> Vec<Shape> {
    let mut out = Vec::new();
    if !settings.on(READY) {
        return out;
    }
    let checks = ready_checks(inv);
    let all = checks.iter().all(|c| c.1);
    let title = if all { "Ready!" } else { "Not ready" };
    let rows: Vec<String> = checks.iter().map(|(what, ok)| format!("{} {what}", if *ok { "+" } else { "-" })).collect();
    let wide = rows.iter().map(|r| width(r)).fold(width(title), f32::max) + 10.0;
    let tall = 14.0 + 10.0 * rows.len() as f32 + 4.0;
    let (x, y) = (4.0, (h / 2.0 - tall / 2.0).floor());
    out.push(Shape::Fill([x, y, wide, tall], BACKGROUND));
    out.push(Shape::Fill([x, y, 2.0, tall], if all { 0xFF5C_E08A } else { 0xFFFF_5A5A }));
    out.push(Shape::Text(title.to_owned(), [x + 6.0, y + 4.0], if all { 0x5CE08A } else { 0xFF5A5A }, true));
    for (i, (row, (_, ok))) in rows.into_iter().zip(&checks).enumerate() {
        out.push(Shape::Text(row, [x + 6.0, y + 16.0 + 10.0 * i as f32], if *ok { 0xCFE8D4 } else { 0x9A9A9A }, false));
    }
    out
}

/// A death the server's chat told of: who died and, when it says, who killed them.
#[derive(Clone, Debug, PartialEq)]
pub struct Death {
    pub victim: String,
    pub killer: Option<String>,
}

/// The vanilla death messages' wordings (death.attack.*), after the victim's name.
const DEATH_VERBS: &[&str] = &["was slain by", "was shot by", "was blown up by", "was killed", "was pummeled by", "was fireballed by",
    "was impaled by", "was obliterated by", "was skewered by", "was speared by", "was squashed", "was struck by lightning", "was frozen",
    "was stung", "was poked", "was doomed to fall", "was burnt", "burned to death", "went up in flames", "walked into fire", "tried to swim in lava",
    "drowned", "died", "fell", "hit the ground too hard", "experienced kinetic energy", "suffocated", "starved", "blew up", "withered away",
    "was roasted", "was squished", "discovered the floor was lava", "went off with a bang", "left the confines", "didn't want to live"];

fn name_token(word: &str) -> Option<String> {
    let w: String = word.chars().filter(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
    (2..=16).contains(&w.len()).then_some(w)
}

/// A vanilla death message ("Steve was slain by Alex using Sword"), also with a server's prefix before the name.
pub fn parse_death(line: &str) -> Option<Death> {
    let (at, verb) = DEATH_VERBS.iter().filter_map(|v| line.find(&format!(" {v}")).map(|i| (i, *v))).min_by_key(|(i, _)| *i)?;
    // A player's chat line ("<Alex> I died lol", "Alex: I fell") is no death message.
    let before = &line[..at];
    if before.contains(['<', '>', ':', '\u{bb}']) || before.split_whitespace().count() > 3 {
        return None;
    }
    let victim = name_token(before.split_whitespace().last()?)?;
    let rest = &line[at + 1 + verb.len()..];
    let killer = if verb.ends_with(" by") {
        rest.split_whitespace().next().and_then(name_token)
    } else {
        rest.find(" by ").and_then(|i| rest[i + 4..].split_whitespace().next()).and_then(name_token)
    };
    Some(Death { killer: killer.filter(|k| *k != victim), victim })
}

/// A chat line someone else wrote that names you (case does not matter, the whole name only).
pub fn mentions(line: &str, own: &str) -> bool {
    if own.len() < 3 || parse_death(line).is_some() {
        return false;
    }
    let lower = line.to_ascii_lowercase();
    let own = own.to_ascii_lowercase();
    // Your own message ("<Name> hi", "[VIP] Name: hi", "Name » hi") isn't a mention: your name is before the
    // first ">", ":" or "»".
    if let Some(cut) = lower.find(['>', ':', '\u{bb}']) {
        if lower[..cut].contains(&own) {
            return false;
        }
    }
    lower.match_indices(&own).any(|(i, _)| {
        let before = lower[..i].chars().last().is_none_or(|c| !c.is_ascii_alphanumeric() && c != '_');
        let after = lower[i + own.len()..].chars().next().is_none_or(|c| !c.is_ascii_alphanumeric() && c != '_');
        before && after
    })
}

/// Combat Tag Timer: a server line that starts (Some(seconds)) or ends (Some(0)) a combat tag.
pub fn combat_line(line: &str) -> Option<u32> {
    let l = line.to_ascii_lowercase();
    if ["no longer in combat", "out of combat", "combat has ended", "you are now safe", "combat tag expired"].iter().any(|p| l.contains(p)) {
        return Some(0);
    }
    if ["now in combat", "you are in combat", "combat tagged", "you have been tagged", "entered combat"].iter().any(|p| l.contains(p)) {
        let seconds = l.split(|c: char| !c.is_ascii_digit()).filter_map(|d| d.parse::<u32>().ok()).find(|n| (3..=120).contains(n));
        return Some(seconds.unwrap_or(15));
    }
    None
}

/// "Double Kill", "Triple Kill"... for the `count`th kill in a row (none for the first).
pub fn streak_name(count: usize) -> Option<&'static str> {
    Some(match count {
        0 | 1 => return None,
        2 => "Double Kill",
        3 => "Triple Kill",
        4 => "Quadra Kill",
        5 => "Penta Kill",
        _ => "Rampage",
    })
}

/// The message a kill or death sends, `{name}` filled in (an empty name drops it and its spaces).
pub fn fill_name(text: &str, name: Option<&str>) -> String {
    text.replace("{name}", name.unwrap_or("")).split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Another player as the combat modules read them (Arsenal's players hook).
#[derive(Clone, Debug, PartialEq)]
pub struct Other {
    pub id: u32,
    /// The account id (0 when unknown).
    pub uuid: u128,
    pub name: String,
    pub pos: [f64; 3],
    pub health: f32,
    pub absorption: f32,
    pub hurt_time: i32,
    /// Ticks using an item (eating...), and whether with the off hand.
    pub using: i32,
    pub using_off_hand: bool,
    /// Main hand, off hand, feet, legs, chest, head.
    pub equipment: Vec<Option<Stack>>,
}

/// The world camera of this frame, for drawing at places in the world.
#[derive(Clone, Debug, PartialEq)]
pub struct Cam {
    pub pos: [f64; 3],
    pub view: [f32; 16],
    pub projection: [f32; 16],
}

fn mul(m: &[f32; 16], v: [f32; 4]) -> [f32; 4] {
    let mut out = [0.0; 4];
    for (row, o) in out.iter_mut().enumerate() {
        *o = (0..4).map(|col| m[col * 4 + row] * v[col]).sum();
    }
    out
}

/// Where a world point is on the screen (GUI pixels), or none behind the camera.
pub fn project(cam: &Cam, p: [f64; 3], w: f32, h: f32) -> Option<[f32; 2]> {
    let rel = [(p[0] - cam.pos[0]) as f32, (p[1] - cam.pos[1]) as f32, (p[2] - cam.pos[2]) as f32, 1.0];
    let clip = mul(&cam.projection, mul(&cam.view, rel));
    if clip[3] <= 0.01 {
        return None;
    }
    let (x, y) = (clip[0] / clip[3], clip[1] / clip[3]);
    if !(-1.5..=1.5).contains(&x) || !(-1.5..=1.5).contains(&y) {
        return None;
    }
    Some([(x + 1.0) / 2.0 * w, (1.0 - y) / 2.0 * h])
}

/// The combat modules' state: kept between frames and ticks.
#[derive(Default)]
struct Pvp {
    own_name: Option<String>,
    /// Messages to send (Kill / Death Message) and when.
    outbox: Vec<(Instant, String)>,
    last_sent: Option<Instant>,
    combat_until: Option<Instant>,
    mention: Option<(String, Instant)>,
    pops: std::collections::HashMap<String, u32>,
    health: std::collections::HashMap<u32, f32>,
    hurt: std::collections::HashMap<u32, i32>,
    damage: Vec<([f64; 3], f32, Instant)>,
    clicked: Option<Instant>,
    hitmarker: Option<Instant>,
    kills: Vec<Instant>,
    streak: Option<(&'static str, Instant)>,
    target: Option<(u32, Instant)>,
    rings: Vec<([f64; 3], Instant)>,
    last_y: Option<f64>,
    last_dy: f64,
    /// Sounds to play (the hooks with a mutable game play them): (sound, volume, pitch).
    sounds: Vec<(&'static str, f32, f32)>,
    /// Lines to show in your own chat only.
    notes: Vec<String>,
    // Session Stats, Combo & Reach, the fight and death cards, the feed.
    session_start: Option<Instant>,
    kills_total: u32,
    deaths_total: u32,
    combo: u32,
    best_combo: u32,
    reach: Option<f32>,
    own_pos: Option<[f64; 3]>,
    combo_shown: Option<(u32, Instant)>,
    fight: Option<Fight>,
    card: Option<(Vec<String>, Instant)>,
    taken: Vec<(Instant, f32)>,
    own_health: Option<f32>,
    killer: Option<String>,
    feed: Vec<(String, Instant)>,
    warned_effects: Vec<String>,
    armor_warned: Vec<String>,
    low_alerted: Option<u32>,
    /// Fall Damage Preview: how far you've fallen since you last stood.
    fallen: f64,
    /// Kills chat told of since Kill Effects last looked.
    new_kills: Vec<String>,
}

/// The fight going on: who with, since when, your hits, the damage you dealt, their pops.
#[derive(Clone, Debug)]
struct Fight {
    with: String,
    id: u32,
    start: Instant,
    last: Instant,
    hits: u32,
    dealt: f32,
    pops: u32,
}

/// Your status (Arsenal's status hook): attack charge, effects and pings.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Status {
    pub attack: f32,
    /// (id, level from 0, ticks left).
    pub effects: Vec<(String, i32, i32)>,
    pub ping: Option<i32>,
    pub latencies: Vec<(String, i32)>,
}

/// An effect's name for the HUD: "minecraft:fire_resistance" -> "Fire Resistance", with its level ("Strength II").
pub fn effect_name(id: &str, level: i32) -> String {
    let base: String = id.strip_prefix("minecraft:").unwrap_or(id).split('_').map(|w| {
        let mut c = w.chars();
        c.next().map(|f| f.to_ascii_uppercase().to_string() + c.as_str()).unwrap_or_default()
    }).collect::<Vec<_>>().join(" ");
    const ROMAN: [&str; 10] = ["", " II", " III", " IV", " V", " VI", " VII", " VIII", " IX", " X"];
    format!("{base}{}", ROMAN.get(level.max(0) as usize).copied().unwrap_or(""))
}

/// "1:05", "0:09", "**:**" for an effect that lasts for ever.
pub fn ticks_text(ticks: i32) -> String {
    if ticks < 0 || ticks > 20 * 60 * 60 * 9 {
        return "**:**".into();
    }
    let secs = ticks / 20;
    format!("{}:{:02}", secs / 60, secs % 60)
}

/// The good effects Potion Run-out Warning watches.
const GOOD: [&str; 9] = ["strength", "speed", "regeneration", "resistance", "fire_resistance", "absorption", "haste", "jump_boost", "invisibility"];

/// A ping's colour: green under 80 ms, yellow under 150, then red.
pub fn ping_rgb(ms: i32) -> u32 {
    if ms < 80 { 0x5CE08A } else if ms < 150 { 0xF5D33D } else { 0xFF5A5A }
}

/// The left mouse button went down: a hit may land (Hitmarker).
pub fn clicked() {
    STATE.with_borrow_mut(|s| s.pvp.clicked = Some(Instant::now()));
}

/// Takes the sounds and own-chat lines the combat modules asked for.
pub fn take_effects() -> (Vec<(&'static str, f32, f32)>, Vec<String>) {
    STATE.with_borrow_mut(|s| (std::mem::take(&mut s.pvp.sounds), std::mem::take(&mut s.pvp.notes)))
}

/// Takes the chat messages due to be sent now (at most one every 2 seconds).
pub fn take_outbox() -> Vec<String> {
    STATE.with_borrow_mut(|s| {
        let now = Instant::now();
        if s.pvp.last_sent.is_some_and(|t| now.duration_since(t) < Duration::from_secs(2)) {
            return Vec::new();
        }
        let Some(i) = s.pvp.outbox.iter().position(|(due, _)| *due <= now) else { return Vec::new() };
        s.pvp.last_sent = Some(now);
        vec![s.pvp.outbox.remove(i).1]
    })
}

/// New chat lines (Arsenal's chat hook): kills and deaths (Kill / Death Message, Kill Streaks), mentions, combat tags.
pub fn chat_lines(settings: &Settings, own: Option<&str>, lines: &[String]) {
    STATE.with_borrow_mut(|s| {
        let p = &mut s.pvp;
        if let Some(own) = own {
            p.own_name = Some(own.to_owned());
        }
        let Some(own) = p.own_name.clone() else { return };
        let now = Instant::now();
        for line in lines {
            if let Some(d) = parse_death(line) {
                if d.killer.as_deref().is_some_and(|k| k.eq_ignore_ascii_case(&own)) && !d.victim.eq_ignore_ascii_case(&own) {
                    if settings.on(KILL_MSG) {
                        p.outbox.push((now + Duration::from_millis(800), fill_name(KILL_TEXTS[settings.v[KILL_TEXT]].0, Some(&d.victim))));
                    }
                    p.kills.retain(|t| now.duration_since(*t) < Duration::from_secs(10));
                    p.kills.push(now);
                    p.kills_total += 1;
                    p.new_kills.push(d.victim.clone());
                    p.feed.push((format!("You killed {}", d.victim), now));
                    if settings.on(STREAKS) {
                        if let Some(name) = streak_name(p.kills.len()) {
                            p.streak = Some((name, now));
                            p.sounds.push(("minecraft:entity.player.levelup", 0.5, 1.2));
                        }
                    }
                } else if d.victim.eq_ignore_ascii_case(&own) {
                    p.kills.clear();
                    p.deaths_total += 1;
                    p.combo = 0;
                    p.killer = d.killer.clone();
                    p.feed.push((match &d.killer { Some(k) => format!("{k} killed you"), None => "You died".into() }, now));
                    if settings.on(DEATH_RECAP) {
                        let taken: f32 = p.taken.iter().filter(|t| now.duration_since(t.0) < Duration::from_secs(10)).map(|t| t.1).sum();
                        let hits = p.taken.iter().filter(|t| now.duration_since(t.0) < Duration::from_secs(10)).count();
                        p.card = Some((vec![
                            match &d.killer { Some(k) => format!("Killed by {k}"), None => "You died".into() },
                            format!("{hits} hits, {:.1} hearts in the last 10s", taken / 2.0),
                            format!("Best combo this life: {}", p.best_combo),
                        ], now));
                    }
                    p.best_combo = 0;
                    if settings.on(DEATH_MSG) {
                        p.outbox.push((now + Duration::from_millis(800), fill_name(DEATH_TEXTS[settings.v[DEATH_TEXT]].0, d.killer.as_deref())));
                    }
                }
                p.pops.remove(&d.victim);
                continue;
            }
            if settings.on(COMBAT_TAG) {
                match combat_line(line) {
                    Some(0) => p.combat_until = None,
                    Some(seconds) => p.combat_until = Some(now + Duration::from_secs(seconds as u64)),
                    None => {}
                }
            }
            if settings.on(MENTION) && mentions(line, &own) {
                p.mention = Some((line.chars().take(60).collect(), now));
                p.sounds.push(("minecraft:block.note_block.pling", 0.6, 1.4));
            }
        }
    });
}

/// The other players this tick and the entity events since the last: totem pops (35), hurts, deaths (3).
pub fn players_tick(settings: &Settings, others: &[Other], events: &[(u32, i32)]) {
    STATE.with_borrow_mut(|s| {
        let p = &mut s.pvp;
        let now = Instant::now();
        for (id, event) in events {
            let Some(o) = others.iter().find(|o| o.id == *id) else { continue };
            match event {
                35 => {
                    let n = p.pops.entry(o.name.clone()).or_default();
                    *n += 1;
                    let n = *n;
                    if settings.on(POP_ALERTS) {
                        p.notes.push(format!("{} popped a totem ({})", o.name, n));
                        p.sounds.push(("minecraft:block.note_block.bell", 0.4, 1.2));
                    }
                    p.feed.push((format!("{} popped ({n})", o.name), now));
                    if let Some(f) = p.fight.as_mut().filter(|f| f.id == o.id) {
                        f.pops += 1;
                    }
                }
                30 if settings.on(SHIELD_SOUND) => {
                    p.sounds.push(("minecraft:item.shield.break", 0.8, 1.0));
                    p.feed.push((format!("{}'s shield broke", o.name), now));
                }
                3 => {
                    p.pops.remove(&o.name);
                }
                _ => {}
            }
        }
        for o in others {
            // Damage Numbers: health that went down since the last tick.
            if let Some(before) = p.health.insert(o.id, o.health + o.absorption) {
                let lost = before - (o.health + o.absorption);
                if lost >= 0.5 && settings.on(DAMAGE_NUMS) {
                    p.damage.push(([o.pos[0], o.pos[1] + 2.0, o.pos[2]], lost, now));
                }
                if lost > 0.0 {
                    if let Some(f) = p.fight.as_mut().filter(|f| f.id == o.id) {
                        f.dealt += lost;
                    }
                }
            }
            // Enemy Armor Alert: a piece at 10% or less, once each.
            if settings.on(ENEMY_ARMOR) {
                for (slot, piece) in [(5usize, "helmet"), (4, "chestplate"), (3, "leggings"), (2, "boots")] {
                    let key = format!("{}:{piece}", o.name);
                    let worn = o.equipment.get(slot).and_then(|s| s.as_ref()).and_then(durability_left).is_some_and(|l| l <= 0.1);
                    if worn && !p.armor_warned.contains(&key) {
                        p.armor_warned.push(key);
                        p.feed.push((format!("{}'s {piece} is about to break", o.name), now));
                        p.sounds.push(("minecraft:block.note_block.chime", 0.5, 1.6));
                    }
                }
            }
            // Target Low HP Alert: the player you fight at 4 hearts or less, once until they heal.
            if settings.on(LOW_HP_ALERT) && p.fight.as_ref().is_some_and(|f| f.id == o.id) {
                if o.health <= 8.0 && p.low_alerted != Some(o.id) {
                    p.low_alerted = Some(o.id);
                    p.sounds.push(("minecraft:entity.experience_orb.pickup", 0.6, 0.8));
                    p.feed.push((format!("{} is low!", o.name), now));
                } else if o.health > 12.0 && p.low_alerted == Some(o.id) {
                    p.low_alerted = None;
                }
            }
            // Hitmarker: a player just hurt (hurt_time jumped) right after you clicked.
            let before = p.hurt.insert(o.id, o.hurt_time).unwrap_or(0);
            if o.hurt_time > before && p.clicked.is_some_and(|c| now.duration_since(c) < Duration::from_millis(400)) {
                p.hitmarker = Some(now);
                p.target = Some((o.id, now));
                p.combo += 1;
                p.best_combo = p.best_combo.max(p.combo);
                if let Some(me) = p.own_pos {
                    let d = [o.pos[0] - me[0], o.pos[1] - me[1], o.pos[2] - me[2]];
                    p.reach = Some(((d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() as f32 - 0.3).max(0.0));
                }
                p.combo_shown = Some((p.combo, now));
                let fight = p.fight.get_or_insert_with(|| Fight { with: o.name.clone(), id: o.id, start: now, last: now, hits: 0, dealt: 0.0, pops: 0 });
                if fight.id != o.id {
                    *fight = Fight { with: o.name.clone(), id: o.id, start: now, last: now, hits: 0, dealt: 0.0, pops: 0 };
                }
                fight.hits += 1;
                fight.last = now;
            }
        }
        p.damage.retain(|d| now.duration_since(d.2) < Duration::from_millis(1200));
        p.feed.retain(|f| now.duration_since(f.1) < Duration::from_secs(8));
        // A fight ends 10 seconds after the last hit: Fight Summary shows its card.
        if p.fight.as_ref().is_some_and(|f| now.duration_since(f.last) > Duration::from_secs(10)) {
            let f = p.fight.take().unwrap();
            if settings.on(FIGHT_SUMMARY) && f.hits > 0 {
                p.card = Some((vec![format!("Fight with {}", f.with), format!("{} hits, {:.1} hearts dealt", f.hits, f.dealt / 2.0),
                    format!("{} totems popped", f.pops), format!("{}s", f.last.duration_since(f.start).as_secs().max(1))], now));
            }
        }
        p.health.retain(|id, _| others.iter().any(|o| o.id == *id));
    });
}

/// Jump Circles: your position each tick; a jump (rising fast from standing) starts a ring at your feet.
pub fn own_tick(settings: &Settings, pos: Option<[f64; 3]>) {
    STATE.with_borrow_mut(|s| {
        let p = &mut s.pvp;
        let Some(pos) = pos else {
            p.last_y = None;
            p.own_pos = None;
            return;
        };
        p.own_pos = Some(pos);
        let dy = p.last_y.map_or(0.0, |y| pos[1] - y);
        if settings.on(JUMP_RINGS) && dy > 0.25 && p.last_dy.abs() < 0.05 {
            p.rings.push(([pos[0], pos[1] - dy, pos[2]], Instant::now()));
        }
        if dy < -0.01 {
            p.fallen += -dy;
        } else if dy.abs() < 0.001 || dy > 0.0 {
            p.fallen = 0.0;
        }
        p.last_dy = dy;
        p.last_y = Some(pos[1]);
        p.rings.retain(|r| r.1.elapsed() < Duration::from_millis(700));
    });
}

/// Your status each tick: being hit breaks your combo (Combo & Reach) and counts for Death Recap; effects about to
/// run out warn (Potion Run-out Warning).
pub fn status_tick(settings: &Settings, status: &Status, health: Option<f32>) {
    STATE.with_borrow_mut(|s| {
        let p = &mut s.pvp;
        let now = Instant::now();
        p.session_start.get_or_insert(now);
        if let Some(h) = health {
            if let Some(before) = p.own_health.replace(h) {
                if h < before - 0.01 {
                    p.combo = 0;
                    p.taken.push((now, before - h));
                }
            }
        }
        p.taken.retain(|t| now.duration_since(t.0) < Duration::from_secs(12));
        if settings.on(POTION_WARN) {
            for (id, level, ticks) in &status.effects {
                let name = id.strip_prefix("minecraft:").unwrap_or(id);
                if GOOD.contains(&name) && (0..=100).contains(ticks) && !p.warned_effects.contains(id) {
                    p.warned_effects.push(id.clone());
                    p.feed.push((format!("{} runs out in 5s", effect_name(id, *level)), now));
                    p.sounds.push(("minecraft:block.note_block.pling", 0.5, 0.7));
                }
            }
            p.warned_effects.retain(|w| status.effects.iter().any(|e| &e.0 == w && e.2 <= 100));
        }
    });
}

/// The status modules' shapes: Potion Timers, the attack bar, Crit Indicator, Lag Meter, the clock, Session Stats,
/// Fight Timer, Combo & Reach, Inventory Totem, the feed and the fight / death card, Dynamic Crosshair, nametag pings.
pub fn status_shapes(settings: &Settings, status: &Status, others: &[Other], aiming: bool, totems: u32, falling: bool, cam: Option<&Cam>, w: f32, h: f32, width: &dyn Fn(&str) -> f32, clock: &str) -> Vec<Shape> {
    let mut out = Vec::new();
    let (cx, cy) = ((w / 2.0).floor(), (h / 2.0).floor());
    let now = Instant::now();
    if settings.on(POTIONS) && !status.effects.is_empty() {
        let mut y = (h / 2.0 - status.effects.len() as f32 * 6.0).floor();
        for (id, level, ticks) in &status.effects {
            let text = format!("{} {}", effect_name(id, *level), ticks_text(*ticks));
            let tw = width(&text);
            out.push(Shape::Fill([2.0, y - 2.0, tw + 6.0, 11.0], BACKGROUND));
            out.push(Shape::Text(text, [5.0, y], if (0..=100).contains(ticks) { 0xFF5A5A } else { 0xFFFFFF }, false));
            y += 12.0;
        }
    }
    if settings.on(ATTACK_BAR) && status.attack < 1.0 {
        out.push(Shape::Fill([cx - 8.0, cy + 9.0, 16.0, 2.0], 0xA000_0000));
        out.push(Shape::Fill([cx - 8.0, cy + 9.0, (16.0 * status.attack).round(), 2.0], 0xFF00_0000 | settings.accent()));
    }
    if settings.on(CRIT_IND) && falling && status.attack >= 0.9 {
        out.push(Shape::Quad([[cx, cy - 6.0], [cx + 2.0, cy - 4.0], [cx, cy - 2.0], [cx - 2.0, cy - 4.0]], 0xFFFF_C83D));
    }
    if settings.on(DYN_CROSSHAIR) && aiming {
        for (dx, dy) in [(-6.0, -1.0), (5.0, -1.0), (-1.0, -6.0), (-1.0, 5.0)] {
            out.push(Shape::Fill([cx + dx, cy + dy, 2.0, 2.0], 0xFFFF_5A5A));
        }
    }
    let mut corner: Vec<(String, u32)> = Vec::new();
    if settings.on(CLOCK) {
        corner.push((clock.to_owned(), 0xFFFFFF));
    }
    if settings.on(LAG) {
        match status.ping {
            Some(ms) => corner.push((format!("Ping {ms} ms"), ping_rgb(ms))),
            None => corner.push(("Ping -".into(), 0xAAAAAA)),
        }
    }
    STATE.with_borrow(|st| {
        let p = &st.pvp;
        if settings.on(SESSION_STATS) {
            let kd = p.kills_total as f32 / p.deaths_total.max(1) as f32;
            let mins = p.session_start.map_or(0, |t| t.elapsed().as_secs() / 60);
            let popped: u32 = p.pops.values().sum();
            corner.push((format!("K {}  D {}  K/D {kd:.2}", p.kills_total, p.deaths_total), 0xFFFFFF));
            corner.push((format!("Pops {popped}  {}h {}m", mins / 60, mins % 60), 0xCCCCCC));
        }
        if settings.on(COMBO_REACH) {
            let reach = p.reach.map_or("-".to_string(), |r| format!("{r:.2}"));
            corner.push((format!("Combo {}  Best {}  Reach {reach}", p.combo, p.best_combo), 0xFFFFFF));
        }
        if settings.on(FIGHT_TIMER) {
            if let Some(f) = &p.fight {
                corner.push((format!("Fight {}s vs {}", f.start.elapsed().as_secs(), f.with), 0xFFC83D));
            }
        }
        // Combo Counter Pop: the number rises and fades by the crosshair.
        if settings.on(COMBO_POP) {
            if let Some((n, at)) = p.combo_shown.filter(|c| c.0 >= 2 && c.1.elapsed() < Duration::from_millis(700)) {
                let t = at.elapsed().as_secs_f32();
                out.push(Shape::Text(format!("{n}x"), [cx + 10.0, cy - 12.0 - t * 14.0], 0xFFC83D, true));
            }
        }
        if settings.on(FEED) {
            let mut y = h - 70.0;
            for (line, _) in p.feed.iter().rev().take(5) {
                let tw = width(line);
                out.push(Shape::Text(line.clone(), [w - tw - 4.0, y], 0xFFFFFF, true));
                y -= 10.0;
            }
        }
        if let Some((lines, at)) = p.card.as_ref().filter(|c| c.1.elapsed() < Duration::from_secs(6)) {
            let _ = at;
            let wide = lines.iter().map(|l| width(l)).fold(0.0, f32::max) + 12.0;
            let tall = lines.len() as f32 * 11.0 + 8.0;
            let (x, y) = (cx - wide / 2.0, h * 0.28);
            out.push(Shape::Fill([x, y, wide, tall], BACKGROUND));
            out.push(Shape::Fill([x, y, wide, 2.0], 0xFF00_0000 | settings.accent()));
            for (i, l) in lines.iter().enumerate() {
                out.push(Shape::Text(l.clone(), [x + 6.0, y + 5.0 + i as f32 * 11.0], if i == 0 { 0xFFC83D } else { 0xFFFFFF }, true));
            }
        }
        let _ = now;
    });
    if settings.on(INV_TOTEM) && totems > 0 {
        out.push(Shape::Item("minecraft:totem_of_undying".into(), [cx - 30.0, cy - 8.0]));
        out.push(Shape::Text(totems.to_string(), [cx - 13.0, cy - 4.0], 0xFFFFFF, true));
    }
    let mut y = 4.0;
    for (text, rgb) in corner {
        let tw = width(&text);
        out.push(Shape::Fill([2.0, y - 2.0, tw + 6.0, 11.0], BACKGROUND));
        out.push(Shape::Text(text, [5.0, y], rgb, false));
        y += 12.0;
    }
    if let (true, Some(cam)) = (settings.on(NAMETAG_PING), cam) {
        for o in others {
            let Some(ms) = status.latencies.iter().find(|l| l.0 == o.name).map(|l| l.1).filter(|ms| *ms > 0) else { continue };
            if let Some([x, y]) = project(cam, [o.pos[0], o.pos[1] + 2.75, o.pos[2]], w, h) {
                let text = format!("{ms} ms");
                let tw = width(&text);
                out.push(Shape::Text(text, [x - tw / 2.0, y], ping_rgb(ms), true));
            }
        }
    }
    out
}

/// What the game should look like (Arsenal's visuals hook): None leaves Horizon's own setting alone.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Visuals {
    pub gamma: Option<f32>,
    pub fire_lower: Option<u8>,
    pub shield_size: Option<u8>,
    pub totem_size: Option<u8>,
    pub glint: Option<String>,
    pub time: Option<String>,
    pub weather: Option<String>,
    pub hit: Option<(String, u8)>,
    pub no_hurt_cam: bool,
    pub no_pumpkin: bool,
    pub particles: u32,
    pub chat_hide: Vec<String>,
    pub hide_particles: u32,
    pub capture: bool,
    pub crystal_optimizer: bool,
}

/// The lines Chat Filter hides (lower case pieces of them).
fn chat_hidden(settings: &Settings) -> Vec<String> {
    if !settings.on(CHAT_FILTER) {
        return Vec::new();
    }
    let joins = ["joined the game", "left the game", "joined the server", "left the server"];
    let ads = ["discord.gg", "http://", "https://", "[ad]", "/vote", "store.", ".tebex"];
    let mut out: Vec<String> = Vec::new();
    match settings.v[FILTER_KIND] {
        1 => out.extend(joins.iter().map(|s| s.to_string())),
        2 => out.extend(ads.iter().map(|s| s.to_string())),
        _ => out.extend(joins.iter().chain(ads.iter()).map(|s| s.to_string())),
    }
    out
}

/// The visuals to apply now. A module switched off gives Horizon's default back once, then leaves the setting to the
/// player (Horizon's own PvP options keep working). `gamma` is the brightness now, saved before Fullbright raises it.
pub fn visuals(settings: &Settings, gamma: f32) -> Visuals {
    STATE.with_borrow_mut(|s| {
        let a = &mut s.applied;
        let (hide_particles, capture) = particle_rules(settings);
        let mut v = Visuals { particles: 100, chat_hide: chat_hidden(settings), hide_particles, capture, crystal_optimizer: settings.on(CRYSTAL_OPT), ..Default::default() };
        let mut pick = |on: bool, slot: usize, value: Option<String>, off: String| -> Option<String> {
            if on {
                a[slot] = true;
                value
            } else if std::mem::take(&mut a[slot]) {
                Some(off)
            } else {
                None
            }
        };
        let num = |list: &[(&str, &str)], i: usize| list[i].0.to_owned();
        v.fire_lower = pick(settings.on(LOW_FIRE), 0, Some(num(FIRE_LOWERS, settings.v[FIRE_LOWER])), "0".into()).and_then(|x| x.parse().ok());
        v.shield_size = pick(settings.on(SMALLER), 1, Some(num(SIZES, settings.v[SHIELD_SIZE])), "100".into()).and_then(|x| x.parse().ok());
        v.totem_size = pick(settings.on(SMALLER), 2, Some(num(SIZES, settings.v[TOTEM_SIZE])), "100".into()).and_then(|x| x.parse().ok());
        v.glint = pick(settings.on(GLINT), 3, Some(num(GLINTS, settings.v[GLINT_COLOR])), "vanilla".into());
        v.time = pick(settings.on(WEATHER_TIME), 4, Some(num(VIEW_TIMES, settings.v[VIEW_TIME])), "world".into());
        v.weather = pick(settings.on(WEATHER_TIME), 5, Some(num(VIEW_WEATHERS, settings.v[VIEW_WEATHER])), "world".into());
        v.hit = pick(settings.on(HIT_FX), 6, Some(num(HIT_COLORS, settings.v[HIT_COLOR])), "vanilla".into()).map(|c| (c, 40));
        v.no_hurt_cam = settings.on(NO_HURT);
        v.no_pumpkin = settings.on(NO_PUMPKIN);
        if settings.on(PARTICLES) {
            v.particles = num(PARTICLE_KEEPS, settings.v[PARTICLE_KEEP]).parse().unwrap_or(100);
        }
        // Fullbright: brightness far past the slider, the player's own back when it's off.
        if settings.on(FULLBRIGHT) {
            if s.saved_gamma.is_none() && gamma < 2.0 {
                s.saved_gamma = Some(gamma);
            }
            v.gamma = Some(15.0);
        } else if let Some(g) = s.saved_gamma.take() {
            v.gamma = Some(g);
        }
        v
    })
}

/// End crystals, lit TNT and cooldowns (Arsenal's world hook).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct World {
    pub crystals: Vec<[f64; 3]>,
    pub tnts: Vec<([f64; 3], i32)>,
    pub cooldowns: Vec<(String, f32)>,
    pub dropped: Vec<([f64; 3], String, u32)>,
}

/// About how much an end crystal at `crystal` hurts someone standing at `at` (health points), after armour points
/// and toughness: vanilla's explosion (power 6) as if nothing blocked it, on Normal.
pub fn crystal_damage(crystal: [f64; 3], at: [f64; 3], armor: f32, toughness: f32) -> f32 {
    let d = ((crystal[0] - at[0]).powi(2) + (crystal[1] - at[1]).powi(2) + (crystal[2] - at[2]).powi(2)).sqrt();
    let impact = (1.0 - d / 12.0).max(0.0) as f32;
    if impact <= 0.0 {
        return 0.0;
    }
    let raw = (impact * impact + impact) / 2.0 * 7.0 * 12.0 + 1.0;
    let reduced = (armor - raw / (2.0 + toughness / 4.0)).max(armor / 5.0).min(20.0);
    raw * (1.0 - reduced / 25.0)
}

/// Armour points and toughness of worn pieces (head, chest, legs, feet).
pub fn armor_of(pieces: &[Option<Stack>]) -> (f32, f32) {
    let mut points = 0.0;
    let mut tough = 0.0;
    for (i, piece) in pieces.iter().enumerate() {
        let Some(p) = piece else { continue };
        let id = p.id.strip_prefix("minecraft:").unwrap_or(&p.id);
        let material = id.split('_').next().unwrap_or("");
        let table: [f32; 4] = match material {
            "leather" => [1.0, 3.0, 2.0, 1.0],
            "golden" | "chainmail" => [2.0, 5.0, 3.0 + if material == "chainmail" { 1.0 } else { 0.0 }, 1.0],
            "iron" | "copper" => [2.0, 6.0, 5.0, 2.0],
            "diamond" => [3.0, 8.0, 6.0, 3.0],
            "netherite" => [3.0, 8.0, 6.0, 3.0],
            "turtle" => [2.0, 0.0, 0.0, 0.0],
            _ => [0.0; 4],
        };
        points += table[i.min(3)];
        tough += match material { "diamond" => 2.0, "netherite" => 3.0, _ => 0.0 };
    }
    (points, tough)
}

/// The respawn anchor's charges from a block's name and state ("minecraft:respawn_anchor[charges=2]").
pub fn anchor_charges(block: &str) -> Option<u32> {
    if !block.contains("respawn_anchor") {
        return None;
    }
    let at = block.find("charges=")?;
    block[at + 8..].chars().next()?.to_digit(10)
}

/// The fall damage landing now would deal (health points), from the distance fallen.
pub fn fall_damage(fallen: f64) -> f32 {
    ((fallen - 3.0).ceil()).max(0.0) as f32
}

/// The world modules' shapes: Cooldown HUD, TNT Timer, Crystal Damage Preview, Fall Damage Preview, Anchor Helper.
pub fn world_shapes(settings: &Settings, world: &World, me: Option<[f64; 3]>, my_armor: (f32, f32), target: Option<&Other>, block: Option<&str>,
                    cam: Option<&Cam>, w: f32, h: f32, width: &dyn Fn(&str) -> f32) -> Vec<Shape> {
    let mut out = Vec::new();
    let (cx, cy) = ((w / 2.0).floor(), (h / 2.0).floor());
    if settings.on(COOLDOWN) {
        let mut x = cx - 91.0;
        for (id, left) in &world.cooldowns {
            out.push(Shape::Item(id.clone(), [x, h - 58.0]));
            out.push(Shape::Fill([x, h - 41.0, 16.0, 2.0], 0xA000_0000));
            out.push(Shape::Fill([x, h - 41.0, (16.0 * left).round(), 2.0], 0xFFFF_5A5A));
            x += 20.0;
        }
    }
    if let Some(cam) = cam {
        if settings.on(TNT) {
            for (pos, fuse) in &world.tnts {
                if let Some([x, y]) = project(cam, [pos[0], pos[1] + 1.3, pos[2]], w, h) {
                    let text = format!("{:.1}s", *fuse as f32 / 20.0);
                    let tw = width(&text);
                    out.push(Shape::Text(text, [x - tw / 2.0, y], if *fuse < 20 { 0xFF5A5A } else { 0xFFC83D }, true));
                }
            }
        }
        if let (true, Some(me)) = (settings.on(CRYSTAL_DMG), me) {
            for c in &world.crystals {
                let d = ((c[0] - me[0]).powi(2) + (c[1] - me[1]).powi(2) + (c[2] - me[2]).powi(2)).sqrt();
                if d > 12.0 {
                    continue;
                }
                let mine = crystal_damage(*c, me, my_armor.0, my_armor.1);
                let mut text = format!("You {:.1}", mine / 2.0);
                if let Some(t) = target {
                    let theirs = crystal_damage(*c, t.pos, 20.0, 8.0);
                    text.push_str(&format!("  Them {:.1}", theirs / 2.0));
                }
                if let Some([x, y]) = project(cam, [c[0], c[1] + 2.2, c[2]], w, h) {
                    let tw = width(&text);
                    out.push(Shape::Fill([x - tw / 2.0 - 2.0, y - 1.0, tw + 4.0, 10.0], 0x8000_0000));
                    out.push(Shape::Text(text, [x - tw / 2.0, y], if mine >= 20.0 { 0xFF5A5A } else { 0xFFFFFF }, false));
                }
            }
        }
    }
    STATE.with_borrow(|s| {
        if settings.on(FALL) && s.pvp.fallen > 3.5 {
            let text = format!("Fall {:.1}", fall_damage(s.pvp.fallen) / 2.0);
            let tw = width(&text);
            out.push(Shape::Text(text, [cx - tw / 2.0, cy + 26.0], 0xFF5A5A, true));
        }
    });
    if let (true, Some(n)) = (settings.on(ANCHOR), block.and_then(anchor_charges)) {
        let text = format!("Anchor {n}/4");
        let tw = width(&text);
        out.push(Shape::Text(text, [cx - tw / 2.0, cy - 34.0], if n == 0 { 0xAAAAAA } else { 0xFFC83D }, true));
    }
    out
}

/// SHA-256 of `data` as lower-case hex (Arsenal Users' account hashes).
pub fn sha256_hex(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
    let mut h: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
    let mut msg = data.to_vec();
    let bits = (data.len() as u64).wrapping_mul(8);
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bits.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[i * 4], chunk[i * 4 + 1], chunk[i * 4 + 2], chunk[i * 4 + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let mut v = h;
        for i in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let ch = (v[4] & v[5]) ^ (!v[4] & v[6]);
            let t1 = v[7].wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = v[0].rotate_right(2) ^ v[0].rotate_right(13) ^ v[0].rotate_right(22);
            let maj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let t2 = s0.wrapping_add(maj);
            v = [t1.wrapping_add(t2), v[0], v[1], v[2], v[3].wrapping_add(t1), v[4], v[5], v[6]];
        }
        for i in 0..8 {
            h[i] = h[i].wrapping_add(v[i]);
        }
    }
    h.iter().map(|x| format!("{x:08x}")).collect()
}

/// A uuid as Minecraft writes it ("069a79f4-44e9-4726-a5be-fca90e38aaf5").
pub fn dashed(uuid: u128) -> String {
    let h = format!("{uuid:032x}");
    format!("{}-{}-{}-{}-{}", &h[0..8], &h[8..12], &h[12..16], &h[16..20], &h[20..32])
}

/// Arsenal Users' hash of an account: SHA-256 of "arsenal-users-v1:" and its uuid.
pub fn user_hash(uuid: u128) -> String {
    sha256_hex(format!("arsenal-users-v1:{}", dashed(uuid)).as_bytes())
}

/// A player's best PvP tier from pvptiers.com's profile answer: ("HT1", colour), none without a current one.
pub fn best_tier(json: &str) -> Option<(String, u32)> {
    let v: serde_json::Value = serde_json::from_str(json).ok()?;
    let rankings = v.get("rankings")?.as_object()?;
    let mut best: Option<(u64, u64)> = None;
    for r in rankings.values() {
        let tier = r.get("tier")?.as_u64()?;
        let pos = r.get("pos").and_then(|p| p.as_u64()).unwrap_or(0);
        if r.get("retired").and_then(|x| x.as_bool()).unwrap_or(false) || !(1..=5).contains(&tier) || pos > 1 {
            continue;
        }
        if best.is_none_or(|b| tier * 2 + pos < b.0 * 2 + b.1) {
            best = Some((tier, pos));
        }
    }
    let (tier, pos) = best?;
    let rgb = match (tier, pos == 0) {
        (1, true) => 0xFFC93C, (1, false) => 0xFFE08A, (2, true) => 0xD4DCE8, (2, false) => 0xAEB8C6,
        (3, true) => 0xE08A4A, (3, false) => 0xC49270, (4, _) => 0xA8A8A8, _ => 0x868686,
    };
    Some((format!("{}T{tier}", if pos == 0 { "H" } else { "L" }), rgb))
}

/// What the network told about players (filled off the game's thread): their tiers and who uses Arsenal.
#[derive(Default)]
struct Net {
    tiers: std::collections::HashMap<u128, Option<(String, u32)>>,
    users: std::collections::HashSet<u128>,
    asked_users: std::collections::HashSet<u128>,
    asking: usize,
    last_lookup: Option<Instant>,
}

static NET: Mutex<Option<Net>> = Mutex::new(None);

fn net<R>(f: impl FnOnce(&mut Net) -> R) -> R {
    let mut guard = NET.lock().unwrap_or_else(|e| e.into_inner());
    f(guard.get_or_insert_with(Net::default))
}

/// Asks pvptiers.com about players not asked yet (one at a time) and Arsenal's website which ones use Arsenal (in
/// batches, at most every 20 seconds). Only the uuid is sent: pvptiers.com gets it for its public profile, Arsenal a
/// one-way hash.
pub fn ask_network(settings: &Settings, others: &[Other]) {
    if settings.on(TIERS) {
        let next = net(|n| {
            if n.asking > 0 {
                return None;
            }
            let id = others.iter().map(|o| o.uuid).find(|u| *u != 0 && !n.tiers.contains_key(u))?;
            n.asking += 1;
            Some(id)
        });
        if let Some(id) = next {
            fetch_tier(id);
        }
    }
    if settings.on(USERS) {
        let batch: Vec<u128> = net(|n| {
            if n.last_lookup.is_some_and(|t| t.elapsed() < Duration::from_secs(20)) {
                return Vec::new();
            }
            let ids: Vec<u128> = others.iter().map(|o| o.uuid).filter(|u| *u != 0 && !n.asked_users.contains(u)).take(50).collect();
            if !ids.is_empty() {
                n.last_lookup = Some(Instant::now());
                n.asked_users.extend(ids.iter().copied());
            }
            ids
        });
        if !batch.is_empty() {
            lookup_users(batch);
        }
    }
}

#[cfg(not(test))]
fn fetch_tier(id: u128) {
    std::thread::spawn(move || {
        let url = format!("https://pvptiers.com/api/profile/{id:032x}");
        let answer = ureq::get(&url).timeout(Duration::from_secs(10)).set("User-Agent", "Arsenal-Horizon").call().ok().and_then(|r| r.into_string().ok());
        net(|n| {
            n.tiers.insert(id, answer.as_deref().and_then(best_tier));
            n.asking = n.asking.saturating_sub(1);
        });
    });
}

#[cfg(test)]
fn fetch_tier(id: u128) {
    net(|n| {
        n.tiers.insert(id, None);
        n.asking = 0;
    });
}

#[cfg(not(test))]
fn lookup_users(ids: Vec<u128>) {
    std::thread::spawn(move || {
        let hashes: Vec<String> = ids.iter().map(|id| user_hash(*id)).collect();
        let body = serde_json::json!({ "hashes": hashes }).to_string();
        let answer = ureq::post(&format!("{WEBSITE}/v1/lookup")).timeout(Duration::from_secs(15)).set("User-Agent", "Arsenal-Horizon")
            .set("Content-Type", "application/json").send_string(&body).ok().and_then(|r| r.into_string().ok());
        let Some(answer) = answer else { return };
        let found = users_in(&answer);
        net(|n| {
            for (id, hash) in ids.iter().zip(&hashes) {
                if found.contains(hash) {
                    n.users.insert(*id);
                }
            }
        });
    });
}

#[cfg(test)]
fn lookup_users(_ids: Vec<u128>) {}

/// The hashes the website's lookup answer names (as plain strings or entries with a hash).
pub fn users_in(json: &str) -> Vec<String> {
    let Ok(v) = serde_json::from_str::<serde_json::Value>(json) else { return Vec::new() };
    v.get("users").and_then(|u| u.as_array()).map_or_else(Vec::new, |list| {
        list.iter().filter_map(|e| e.as_str().map(str::to_owned).or_else(|| ["hash", "id", "user"].iter().find_map(|k| e.get(*k)?.as_str().map(str::to_owned)))).collect()
    })
}

/// Discord Rich Presence: the status Arsenal hands the launcher (`ARSENAL_PRESENCE_RELAY`), as the Java mod's
/// PresenceRelay writes it: {pid, at, activity}.
pub fn presence_json(settings: &Settings, playing: bool, at_ms: u64, pid: u32) -> String {
    if !settings.on(DISCORD) {
        return serde_json::json!({ "pid": pid, "at": at_ms }).to_string();
    }
    let (kills, deaths, fight) = STATE.with_borrow(|s| (s.pvp.kills_total, s.pvp.deaths_total, s.pvp.fight.as_ref().map(|f| f.with.clone())));
    let details = if playing { "Playing Horizon with Arsenal" } else { "In the menus" };
    let state = match fight {
        Some(with) => format!("Fighting {with}"),
        None if kills + deaths > 0 => format!("{kills} kills, {deaths} deaths"),
        None => "Minecraft 26.3, in Rust".to_owned(),
    };
    serde_json::json!({
        "pid": pid,
        "at": at_ms,
        "activity": {
            "details": details,
            "state": state,
            "assets": { "large_image": "logo", "large_text": "Arsenal" },
            "buttons": [{ "label": "Get Arsenal", "url": WEBSITE }],
        },
    }).to_string()
}

/// Where the launcher wants the status written (none when Horizon wasn't started by it or Show on Discord is off).
pub fn presence_file() -> Option<std::path::PathBuf> {
    std::env::var_os("ARSENAL_PRESENCE_RELAY").map(std::path::PathBuf::from)
}

/// Writes the status for the launcher every few seconds (whole files only, never half of one).
pub fn write_presence(settings: &Settings, playing: bool) {
    let Some(path) = presence_file() else { return };
    let due = STATE.with_borrow_mut(|s| {
        let due = s.presence_at.is_none_or(|t| t.elapsed() >= Duration::from_secs(5));
        if due {
            s.presence_at = Some(Instant::now());
        }
        due
    });
    if !due {
        return;
    }
    let at = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as u64);
    let text = presence_json(settings, playing, at, std::process::id());
    let tmp = path.with_extension("tmp");
    if std::fs::write(&tmp, text).is_ok() {
        let _ = std::fs::rename(&tmp, &path);
    }
}

fn fx_rgb(settings: &Settings, choice: usize, t: f32) -> u32 {
    match FX_COLORS[choice].0 {
        "accent" => settings.accent(),
        "red" => 0xFF5A5A,
        "white" => 0xFFFFFF,
        "rainbow" => {
            let h = (t.rem_euclid(1.0)) * 6.0;
            let x = 1.0 - ((h % 2.0) - 1.0).abs();
            let (r, g, b) = match h as u32 { 0 => (1.0, x, 0.0), 1 => (x, 1.0, 0.0), 2 => (0.0, 1.0, x), 3 => (0.0, x, 1.0), 4 => (x, 0.0, 1.0), _ => (1.0, 0.0, x) };
            ((r * 255.0) as u32) << 16 | ((g * 255.0) as u32) << 8 | (b * 255.0) as u32
        }
        _ => 0xFFC83D,
    }
}

/// Particles Arsenal draws itself (captured where the game made them): crits and sweeps, then they fade.
pub fn particles_tick(settings: &Settings, caught: &[(&str, [f64; 3])], own_pop: bool) {
    STATE.with_borrow_mut(|s| {
        let now = Instant::now();
        for (name, at) in caught {
            match *name {
                "crit" | "enchanted_hit" if settings.on(CRITS) => {
                    if s.fx.iter().filter(|f| f.1 == 1).count() < 120 {
                        let n = s.fx.len() as f64;
                        // A little burst out from where the vanilla particle would have been.
                        let dir = [(n * 2.3).sin() * 0.6, 0.3 + (n * 1.7).cos().abs() * 0.5, (n * 3.1).cos() * 0.6];
                        s.fx.push(([at[0], at[1], at[2]], 1, now, dir));
                    }
                }
                "sweep_attack" if settings.on(SWEEP) => s.fx.push(([at[0], at[1], at[2]], 2, now, [0.0; 3])),
                _ => {}
            }
        }
        if own_pop && settings.on(TOTEM_STYLE) {
            s.totem_flash = Some(now);
        }
        s.fx.retain(|f| f.2.elapsed() < Duration::from_millis(700));
        s.kill_fx.retain(|f| f.1.elapsed() < Duration::from_millis(1200));
    });
}

/// Kill Effects: where a player you killed was (their last place), from Kill Message's chat check.
pub fn killed_at(settings: &Settings, others: &[Other], victim: &str) {
    if !settings.on(KILL_FX) {
        return;
    }
    if let Some(o) = others.iter().find(|o| o.name.eq_ignore_ascii_case(victim)) {
        STATE.with_borrow_mut(|s| s.kill_fx.push((o.pos, Instant::now())));
    }
}

/// The names of the kills chat told of since the last call (Kill Effects reads them).
pub fn take_kills() -> Vec<String> {
    STATE.with_borrow_mut(|s| std::mem::take(&mut s.pvp.new_kills))
}

/// Which particles to hide (`ARSENAL_HIDE`'s bits) and whether to keep crit and sweep places.
pub fn particle_rules(settings: &Settings) -> (u32, bool) {
    let mut hide = 0;
    if settings.on(CRITS) { hide |= 1; }
    if settings.on(SWEEP) { hide |= 2; }
    if settings.on(TOTEM_STYLE) && settings.on(TOTEM_HIDE) { hide |= 4; }
    if settings.on(HIDE_OWN_FX) { hide |= 8; }
    (hide, settings.on(CRITS) || settings.on(SWEEP))
}

fn shape_points(shape: usize, c: [f32; 2], r: f32, turn: f32) -> Vec<[[f32; 2]; 4]> {
    let at = |a: f32, d: f32| [c[0] + (a + turn).cos() * d, c[1] + (a + turn).sin() * d];
    match CRIT_SHAPES[shape].0 {
        "heart" => {
            let mut out = Vec::new();
            for side in [-1.0f32, 1.0] {
                let lobe = [c[0] + side * r * 0.5, c[1] - r * 0.25];
                let p = |a: f32| [lobe[0] + a.cos() * r * 0.5, lobe[1] + a.sin() * r * 0.5];
                for i in 0..6 {
                    let (a0, a1) = (i as f32 / 6.0 * std::f32::consts::TAU, (i + 1) as f32 / 6.0 * std::f32::consts::TAU);
                    out.push([lobe, p(a0), p(a1), lobe]);
                }
            }
            out.push([[c[0] - r, c[1] - r * 0.1], [c[0] + r, c[1] - r * 0.1], [c[0], c[1] + r], [c[0], c[1] + r]]);
            out
        }
        "diamond" => vec![[at(0.0, r), at(1.5708, r * 0.7), at(3.1416, r), at(4.7124, r * 0.7)]],
        "circle" => (0..8).map(|i| {
            let (a0, a1) = (i as f32 / 8.0 * std::f32::consts::TAU, (i + 1) as f32 / 8.0 * std::f32::consts::TAU);
            [c, at(a0, r), at(a1, r), c]
        }).collect(),
        _ => (0..4).map(|i| {
            let a = i as f32 * 1.5708;
            [c, at(a - 0.35, r * 0.35), at(a, r), at(a + 0.35, r * 0.35)]
        }).collect(),
    }
}

/// The effects Arsenal draws: Crit Particles, Sweep Effect, Swing Trails, Totem Pop Style's flash, Kill Effects,
/// and PvP Tiers and Arsenal Users above heads.
pub fn effect_shapes(settings: &Settings, others: &[Other], cam: Option<&Cam>, w: f32, h: f32, width: &dyn Fn(&str) -> f32) -> Vec<Shape> {
    let mut out = Vec::new();
    let now = Instant::now();
    STATE.with_borrow(|s| {
        if let Some(cam) = cam {
            for (i, (at, kind, start, dir)) in s.fx.iter().enumerate() {
                let t = start.elapsed().as_secs_f32();
                let alpha = (((1.0 - t / 0.7).max(0.0)) * 255.0) as u32;
                if *kind == 1 {
                    let p = [at[0] + dir[0] * t as f64, at[1] + dir[1] * t as f64 - 0.3 * (t * t) as f64, at[2] + dir[2] * t as f64];
                    if let Some(c) = project(cam, p, w, h) {
                        let rgb = fx_rgb(settings, settings.v[CRIT_COLOR], i as f32 * 0.07 + t);
                        for q in shape_points(settings.v[CRIT_SHAPE], c, 5.0, t * 3.0) {
                            out.push(Shape::Quad(q, alpha << 24 | rgb));
                        }
                    }
                } else {
                    // Sweep: a flat arc round the place, opening as it fades.
                    let n = 12;
                    let radius = 0.9 + t as f64 * 0.6;
                    for k in 0..n {
                        let a0 = -1.2 + 2.4 * k as f64 / n as f64;
                        let a1 = -1.2 + 2.4 * (k + 1) as f64 / n as f64;
                        let pt = |a: f64, r: f64| [at[0] + a.sin() * r, at[1], at[2] + a.cos() * r];
                        let rgb = fx_rgb(settings, settings.v[SWEEP_COLOR], k as f32 / n as f32);
                        let quad = [pt(a0, radius), pt(a1, radius), pt(a1, radius - 0.18), pt(a0, radius - 0.18)].map(|p| project(cam, p, w, h));
                        if let [Some(a), Some(b), Some(c), Some(d)] = quad {
                            out.push(Shape::Quad([a, b, c, d], (alpha * 3 / 4) << 24 | rgb));
                        }
                    }
                }
            }
            for (at, start) in &s.kill_fx {
                let t = start.elapsed().as_secs_f32();
                let alpha = (((1.0 - t / 1.2).max(0.0)) * 255.0) as u32;
                let Some(c) = project(cam, [at[0], at[1] + 1.0, at[2]], w, h) else { continue };
                match KILL_STYLES[settings.v[KILL_STYLE]].0 {
                    "ring" => {
                        let r = 6.0 + t * 60.0;
                        for i in 0..16 {
                            let (a0, a1) = (i as f32 / 16.0 * std::f32::consts::TAU, (i + 1) as f32 / 16.0 * std::f32::consts::TAU);
                            let p = |a: f32, d: f32| [c[0] + a.cos() * d, c[1] + a.sin() * d * 0.4];
                            out.push(Shape::Quad([p(a0, r), p(a1, r), p(a1, r - 3.0), p(a0, r - 3.0)], alpha << 24 | settings.accent()));
                        }
                    }
                    "lightning" => {
                        if let Some(top) = project(cam, [at[0], at[1] + 30.0, at[2]], w, h) {
                            let mut prev = top;
                            for k in 1..=6 {
                                let f = k as f32 / 6.0;
                                let next = [top[0] + (c[0] - top[0]) * f + if k % 2 == 0 { 6.0 } else { -6.0 } * (1.0 - f), top[1] + (c[1] - top[1]) * f];
                                out.push(Shape::Quad([[prev[0] - 1.5, prev[1]], [prev[0] + 1.5, prev[1]], [next[0] + 1.5, next[1]], [next[0] - 1.5, next[1]]], alpha << 24 | 0xDDE8FF));
                                prev = next;
                            }
                        }
                    }
                    _ => {
                        for i in 0..12 {
                            let a = i as f32 / 12.0 * std::f32::consts::TAU;
                            let (d0, d1) = (6.0 + t * 30.0, 14.0 + t * 50.0);
                            let n = [-a.sin() * 1.2, a.cos() * 1.2];
                            let p0 = [c[0] + a.cos() * d0, c[1] + a.sin() * d0];
                            let p1 = [c[0] + a.cos() * d1, c[1] + a.sin() * d1];
                            out.push(Shape::Quad([[p0[0] - n[0], p0[1] - n[1]], [p1[0] - n[0], p1[1] - n[1]], [p1[0] + n[0], p1[1] + n[1]], [p0[0] + n[0], p0[1] + n[1]]], alpha << 24 | 0xFFC83D));
                        }
                    }
                }
            }
            // PvP Tiers and Arsenal Users above heads (higher than Nametag Health).
            if settings.on(TIERS) || settings.on(USERS) {
                net(|n| {
                    for o in others {
                        let tier = if settings.on(TIERS) { n.tiers.get(&o.uuid).cloned().flatten() } else { None };
                        let user = settings.on(USERS) && n.users.contains(&o.uuid);
                        if tier.is_none() && !user {
                            continue;
                        }
                        let Some([x, y]) = project(cam, [o.pos[0], o.pos[1] + 3.05, o.pos[2]], w, h) else { continue };
                        let label = match (&tier, user) {
                            (Some((t, _)), true) => format!("A {t}"),
                            (Some((t, _)), false) => t.clone(),
                            (None, _) => "A".to_owned(),
                        };
                        let tw = width(&label);
                        out.push(Shape::Fill([x - tw / 2.0 - 2.0, y - 1.0, tw + 4.0, 10.0], 0x9000_0000));
                        out.push(Shape::Text(label, [x - tw / 2.0, y], tier.map_or(0xFFC83D, |t| t.1), false));
                    }
                });
            }
        }
        // Swing Trails: an arc sweeping across the lower right for a fifth of a second after each swing.
        if settings.on(SWING) {
            if let Some(c) = s.pvp.clicked.filter(|c| c.elapsed() < Duration::from_millis(220)) {
                let t = c.elapsed().as_secs_f32() / 0.22;
                let (cx, cy) = (w * 0.62, h * 0.62);
                for k in 0..10 {
                    let f0 = k as f32 / 10.0 * t;
                    let f1 = (k + 1) as f32 / 10.0 * t;
                    let p = |f: f32, r: f32| [cx + (2.6 - f * 2.2).cos() * r, cy - (2.6 - f * 2.2).sin() * r];
                    let alpha = (((k as f32 / 10.0) * 180.0) as u32) << 24;
                    out.push(Shape::Quad([p(f0, 70.0), p(f1, 70.0), p(f1, 62.0), p(f0, 62.0)], alpha | settings.accent()));
                }
            }
        }
        // Totem Pop Style: the screen flashes your colour when your totem pops.
        if let Some(at) = s.totem_flash.filter(|t| t.elapsed() < Duration::from_millis(500)) {
            let rgb = match FLASH_COLORS[settings.v[TOTEM_FLASH]].0 { "green" => Some(0x5CE08A), "red" => Some(0xFF5A5A), "blue" => Some(0x5AA9FF), "none" => None, _ => Some(0xFFC83D) };
            if let Some(rgb) = rgb {
                let alpha = ((1.0 - at.elapsed().as_secs_f32() / 0.5) * 90.0) as u32;
                out.push(Shape::Fill([0.0, 0.0, w, h], alpha << 24 | rgb));
            }
        }
    });
    let _ = now;
    out
}

/// The values Arsenal hands its shader pack each frame (`frame.mods`, see shaderpack/arsenal/common.wgsl).
pub fn shader_params(settings: &Settings) -> [[f32; 4]; 8] {
    let mut p = [[0.0f32; 4]; 8];
    let amount = |i: usize| AMOUNTS[settings.v[i]].0.parse::<f32>().unwrap_or(0.5);
    if settings.on(BLOOM) {
        p[0][0] = amount(BLOOM_AMT);
    }
    if settings.on(MOTION) {
        p[0][1] = amount(MOTION_AMT) * 0.8;
    }
    if settings.on(MIST) {
        p[0][2] = 1.0;
        let rgb = match MIST_COLORS[settings.v[MIST_COLOR]].0 { "blue" => [0.6, 0.72, 0.9], "purple" => [0.7, 0.6, 0.9], "green" => [0.62, 0.85, 0.66], _ => [0.85, 0.87, 0.9] };
        p[5] = [rgb[0], rgb[1], rgb[2], 72.0];
    }
    p[0][3] = 1.0;
    if settings.on(LOOK) {
        p[1] = match LOOKS[settings.v[LOOK_KIND]].0 {
            "cinematic" => [0.9, 1.15, 0.95, 0.35],
            "faded" => [0.75, 0.85, 1.05, 0.15],
            "noir" => [0.001, 1.25, 1.0, 0.4],
            "warm" => [1.1, 1.05, 1.02, 0.2],
            "cold" => [0.95, 1.05, 1.0, 0.2],
            _ => [1.3, 1.08, 1.03, 0.2],
        };
        let warmth = match LOOKS[settings.v[LOOK_KIND]].0 { "warm" => Some([1.0, 0.85, 0.65]), "cold" => Some([0.7, 0.85, 1.0]), _ => None };
        if let Some(rgb) = warmth {
            p[2] = [rgb[0], rgb[1], rgb[2], 0.25];
        }
    }
    if settings.on(COLORS) {
        let rgb = match TINTS[settings.v[WORLD_TINT]].0 {
            "blue" => [0.55, 0.7, 1.0], "pink" => [1.0, 0.6, 0.85], "green" => [0.6, 1.0, 0.65], "orange" => [1.0, 0.7, 0.45], "red" => [1.0, 0.5, 0.5],
            "accent" => { let a = settings.accent(); [((a >> 16) & 255) as f32 / 255.0, ((a >> 8) & 255) as f32 / 255.0, (a & 255) as f32 / 255.0] }
            _ => [0.75, 0.55, 1.0],
        };
        p[2] = [rgb[0], rgb[1], rgb[2], 0.3];
        p[3] = [rgb[0], rgb[1], rgb[2], 0.55];
    }
    if settings.on(SKY_FX) {
        p[4] = [1.0, 0.8, 0.6, 0.0];
    }
    p
}

/// The pack's files (shaderpack/arsenal, put in by scripts/embed_pack.py), written into the game's shaderpacks/.
// BEGIN PACK
pub const PACK: &[(&str, &str)] = &[
    ("common.wgsl", r####"// What Arsenal hands the pack each frame (frame.mods, set by the mod's native_shader_params hook):
//   mods[0]: x Bloom strength, y Motion Blur strength, z Mist density, w 1 while Arsenal runs
//   mods[1]: World Look: x saturation, y contrast, z exposure, w vignette (all 0 = leave the picture)
//   mods[2]: world tint: rgb colour, w how much
//   mods[3]: sky tint: rgb colour, w how much
//   mods[4]: Sky Effects: x aurora, y stars, z sky glow
//   mods[5]: Mist colour: rgb, w height it lies under (blocks)

fn luminance(c: vec3<f32>) -> f32 {
    return dot(c, vec3<f32>(0.2126, 0.7152, 0.0722));
}

// 1 at night in the Overworld, 0 by day.
fn night() -> f32 {
    return select(0.0, 1.0 - smoothstep(-0.15, 0.05, frame.sun_direction.y), frame.dimension == 0);
}

fn hash2(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}
"####),
    ("horizon-shaders.toml", r####"schema = 1
id = "arsenal"
name = "Arsenal"
version = "1.0.0"
api = "0.1"
description = "Arsenal's visuals on Horizon: Bloom, World Look, Sky & World Colors, Mist, Sky Effects and Motion Blur, switched on and set in Arsenal's menu (Right Shift). With them all off it leaves the picture as it is."
authors = ["Arsenal"]
license = "MIT"
shared = ["common.wgsl"]

[sky]
file = "sky.wgsl"
name = "Sky Effects and sky colours"

[[post]]
file = "post/grade.wgsl"
name = "World Look, world colours and Mist"

[[post]]
file = "post/bloom.wgsl"
name = "Bloom"

[[post]]
file = "post/motion.wgsl"
name = "Motion Blur"

[budget]
gpu_ms = 2.0
"####),
    ("post/bloom.wgsl", r####"// Bloom: bright parts glow out softly (a ring of samples round each pixel, only what's brighter than the threshold).
fn post(input: PostInput) -> vec4<f32> {
    let strength = frame.mods[0].x;
    if strength <= 0.0 {
        return input.color;
    }
    let px = 1.0 / frame.resolution;
    var glow = vec3<f32>(0.0);
    var total = 0.0;
    for (var ring = 1; ring <= 3; ring++) {
        for (var k = 0; k < 8; k++) {
            let a = f32(k) * 0.7853982 + f32(ring) * 0.4;
            let offset = vec2<f32>(cos(a), sin(a)) * px * f32(ring) * 4.0;
            let s = textureSampleLevel(previous, linear_sampler, input.uv + offset, 0.0).rgb;
            let bright = max(luminance(s) - 0.65, 0.0);
            let w = 1.0 / f32(ring);
            glow += s * bright * w;
            total += w;
        }
    }
    let c = input.color.rgb + glow / max(total, 1.0) * strength * 4.0;
    return vec4<f32>(c, input.color.a);
}
"####),
    ("post/grade.wgsl", r####"// World Look (saturation, contrast, exposure, vignette), Sky & World Colors' world tint, and Mist low over the ground.
fn post(input: PostInput) -> vec4<f32> {
    var c = input.color.rgb;
    let look = frame.mods[1];
    if look.x > 0.0 {
        c = mix(vec3<f32>(luminance(c)), c, look.x);
    }
    if look.y > 0.0 {
        c = (c - 0.5) * look.y + 0.5;
    }
    if look.z > 0.0 {
        c *= look.z;
    }
    let tint = frame.mods[2];
    c = mix(c, c * tint.rgb * 1.5, clamp(tint.w, 0.0, 1.0));
    let mist = frame.mods[5];
    let density = frame.mods[0].z;
    if density > 0.0 && input.depth > 0.0 {
        let p = view_position(input.uv, input.depth);
        let height = p.y + frame.camera_position.y;
        let low = 1.0 - smoothstep(mist.w - 6.0, mist.w + 4.0, height);
        let far = 1.0 - exp(-length(p) * 0.03 * density);
        c = mix(c, mist.rgb, clamp(far * low, 0.0, 0.85));
    }
    if look.w > 0.0 {
        let aspect = frame.resolution.x / frame.resolution.y;
        let d = length((input.uv - 0.5) * vec2<f32>(aspect, 1.0)) / length(vec2<f32>(aspect, 1.0) * 0.5);
        c *= 1.0 - smoothstep(0.5, 1.15, d) * look.w;
    }
    return vec4<f32>(clamp(c, vec3<f32>(0.0), vec3<f32>(1.0)), input.color.a);
}
"####),
    ("post/motion.wgsl", r####"// Motion Blur: a little of the last frame stays on, so fast turns smear (none on the first frame or after a pause).
fn post(input: PostInput) -> vec4<f32> {
    let k = clamp(frame.mods[0].y, 0.0, 0.85);
    if k <= 0.0 || frame.delta_time <= 0.0 {
        return input.color;
    }
    let last = textureSampleLevel(history, linear_sampler, input.uv, 0.0).rgb;
    return vec4<f32>(mix(input.color.rgb, last, k), input.color.a);
}
"####),
    ("sky.wgsl", r####"// The sky: Sky & World Colors' sky tint, and Sky Effects' aurora, extra stars and glow at night.
fn sky(input: SkyInput) -> vec4<f32> {
    var c = input.color.rgb;
    let tint = frame.mods[3];
    c = mix(c, c * tint.rgb * 1.6, clamp(tint.w, 0.0, 1.0));
    let fx = frame.mods[4];
    let up = input.direction.y;
    let n = night();
    if fx.x > 0.0 && up > 0.0 {
        // Aurora: soft green and violet bands drifting across the northern sky.
        let band = sin(input.direction.x * 6.0 + frame.time * 0.15) * 0.5 + 0.5;
        let wave = sin(input.direction.z * 9.0 + band * 3.0 + frame.time * 0.3) * 0.5 + 0.5;
        let height = smoothstep(0.05, 0.35, up) * (1.0 - smoothstep(0.55, 0.95, up));
        let colour = mix(vec3<f32>(0.15, 0.95, 0.55), vec3<f32>(0.55, 0.3, 0.95), band);
        c += colour * wave * height * fx.x * n * 0.6;
    }
    if fx.y > 0.0 && up > 0.0 {
        // More stars: tiny points from a hash of the direction.
        let cell = floor(input.direction.xz / max(up, 0.05) * 220.0);
        let star = step(0.997, hash2(cell)) * smoothstep(0.0, 0.2, up);
        c += vec3<f32>(star * fx.y * n);
    }
    if fx.z > 0.0 {
        c += vec3<f32>(0.25, 0.2, 0.45) * fx.z * n * (1.0 - smoothstep(0.0, 0.5, abs(up))) * 0.4;
    }
    return vec4<f32>(c, input.color.a);
}
"####),
];
// END PACK

/// Writes Arsenal's shader pack into `instance/shaderpacks/arsenal` when it isn't there as it should be.
pub fn install_pack(instance: &std::path::Path) {
    let dir = instance.join("shaderpacks").join("arsenal");
    for (name, text) in PACK {
        let path = dir.join(name);
        if std::fs::read_to_string(&path).ok().as_deref() != Some(*text) {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(&path, text);
        }
    }
}

/// Installs the pack once per start.
pub fn install_pack_once(instance: &std::path::Path) {
    let first = STATE.with_borrow_mut(|s| !std::mem::replace(&mut s.pack_done, true));
    if first {
        install_pack(instance);
    }
}

/// Session Summary: entering (true) or leaving (false) a world; leaving ends the session.
pub fn world_change(in_world: bool) {
    let was = STATE.with_borrow_mut(|s| std::mem::replace(&mut s.in_world, in_world));
    if was && !in_world {
        session_ended();
        STATE.with_borrow_mut(|s| s.extra.summary_shown = None);
    }
}

/// A Minecraft item id as a name: "minecraft:enchanted_golden_apple" -> "Enchanted Golden Apple".
pub fn item_name(id: &str) -> String {
    id.strip_prefix("minecraft:").unwrap_or(id).split('_').map(|w| {
        let mut c = w.chars();
        c.next().map(|f| f.to_ascii_uppercase().to_string() + c.as_str()).unwrap_or_default()
    }).collect::<Vec<_>>().join(" ")
}

const VALUABLE: [&str; 9] = ["totem_of_undying", "enchanted_golden_apple", "golden_apple", "netherite", "diamond", "elytra", "mace", "end_crystal", "ender_pearl"];

fn is_weapon(id: &str) -> bool {
    ["_sword", "_axe", "mace", "trident", "_spear"].iter().any(|w| id.ends_with(w))
}

/// Bow Charge: how drawn the held bow, crossbow or trident is, 0..1 (none for anything else).
pub fn bow_charge(item: &str, ticks: u32) -> Option<f32> {
    let full = if item.ends_with("crossbow") { 25.0 } else if item.ends_with("bow") { 20.0 } else if item.ends_with("trident") { 10.0 } else { return None };
    Some((ticks as f32 / full).min(1.0))
}

/// The mace's bonus damage for a fall of `fallen` blocks (vanilla's: 4 a block for 3, 2 a block for the next 5, then 1).
pub fn mace_bonus(fallen: f64) -> f32 {
    if fallen <= 1.5 {
        return 0.0;
    }
    let f = fallen as f32;
    (f.min(3.0) * 4.0) + ((f - 3.0).clamp(0.0, 5.0) * 2.0) + (f - 8.0).max(0.0)
}

/// Mace Optimizer: a player just hurt (hurt_time 10) while you fall with a mace in hand gets the smash sound now,
/// not when the server's comes back.
pub fn mace_hit(settings: &Settings, others: &[Other], holding_mace: bool) -> bool {
    if !settings.on(MACE_OPT) || !holding_mace {
        return false;
    }
    STATE.with_borrow_mut(|s| {
        let hit = s.pvp.fallen > 1.5 && others.iter().any(|o| o.hurt_time == 10);
        if hit {
            let heavy = s.pvp.fallen > 5.0;
            s.pvp.sounds.push((if heavy { "minecraft:item.mace.smash_ground_heavy" } else { "minecraft:item.mace.smash_ground" }, 1.0, 1.0));
        }
        hit
    })
}

/// The keys Arsenal adds besides the menu and zoom: Chat Hotkeys' four and Friends' (all unbound to start but Friend).
pub const HOTKEY_KEYS: [(&str, &str, &str); 4] = [("hotkey_gg", "Chat Hotkey: gg", "gg"), ("hotkey_glhf", "Chat Hotkey: gl hf", "gl hf"),
    ("hotkey_spawn", "Chat Hotkey: /spawn", "/spawn"), ("hotkey_kit", "Chat Hotkey: /kit", "/kit")];
pub const FRIEND_KEY: &str = "friend";
pub const DEFAULT_FRIEND_KEY: &str = "key.mouse.middle";

/// A key went down: Chat Hotkeys queue their message, the Friend key marks the player you look at.
pub fn hotkey(settings: &Settings, key: &str, bound: &dyn Fn(&str) -> String, target: Option<&str>) -> Option<String> {
    if settings.on(HOTKEYS) {
        for (id, _, message) in HOTKEY_KEYS {
            if bound(id) == key {
                STATE.with_borrow_mut(|s| s.pvp.outbox.push((Instant::now(), message.to_owned())));
                return None;
            }
        }
    }
    if settings.on(FRIENDS) && bound(FRIEND_KEY) == key {
        return target.map(str::to_owned);
    }
    None
}

/// The friends list as kept in its setting ("Alex,Steve").
pub fn friends_of(stored: Option<String>) -> Vec<String> {
    stored.unwrap_or_default().split(',').map(str::trim).filter(|s| !s.is_empty()).map(str::to_owned).collect()
}

/// Adds or removes `name`: the new list to keep, and whether they're a friend now.
pub fn toggle_friend(list: &[String], name: &str) -> (String, bool) {
    let mut out: Vec<String> = list.iter().filter(|f| !f.eq_ignore_ascii_case(name)).cloned().collect();
    let added = out.len() == list.len();
    if added {
        out.push(name.to_owned());
    }
    (out.join(","), added)
}

/// The rest of the combat modules' per-tick reading: who's eating, pearls (a player whose pearl just left their hand),
/// your re-totem time, and each fight's opponent intel.
pub fn more_tick(settings: &Settings, others: &[Other], events: &[(u32, i32)], own_id: u32, offhand_totem: bool) {
    STATE.with_borrow_mut(|s| {
        let now = Instant::now();
        let x = &mut s.extra;
        for o in others {
            // A golden apple eaten: using went back to 0 after 32 ticks with one in hand.
            let was = x.using.insert(o.id, (o.using, o.equipment.get(if o.using_off_hand { 1 } else { 0 }).cloned().flatten().map(|s| s.id)));
            if let Some((ticks, Some(item))) = was {
                if ticks >= 30 && o.using == 0 && item.contains("golden_apple") {
                    *x.gapples.entry(o.name.clone()).or_default() += 1;
                }
                // A pearl thrown: the held stack of pearls got smaller.
                let _ = item;
            }
            let pearls = o.equipment.iter().flatten().filter(|s| s.id == "minecraft:ender_pearl").map(|s| s.count).sum::<u32>();
            if let Some(before) = x.pearls.insert(o.id, pearls) {
                if pearls < before && settings.on(PEARLS) {
                    x.thrown.insert(o.id, now);
                }
            }
        }
        for (id, event) in events {
            if *event == 30 {
                x.shield_down.insert(*id, now);
            }
            if *event == 35 && *id == own_id && own_id != 0 {
                x.popped_at = Some(now);
                x.retotem = None;
            }
        }
        if let Some(at) = x.popped_at {
            if offhand_totem && x.retotem.is_none() {
                x.retotem = Some(now.duration_since(at));
                x.popped_at = None;
            }
        }
        x.thrown.retain(|_, t| t.elapsed() < Duration::from_secs(15));
        x.shield_down.retain(|_, t| t.elapsed() < Duration::from_secs(5));
        let _ = settings;
    });
}

/// Session Summary: the session that just ended, shown when you're in a world again.
pub fn session_ended() {
    STATE.with_borrow_mut(|s| {
        let p = &s.pvp;
        let Some(start) = p.session_start else { return };
        let mins = start.elapsed().as_secs() / 60;
        let kd = p.kills_total as f32 / p.deaths_total.max(1) as f32;
        let pops: u32 = p.pops.values().sum();
        s.extra.summary = Some(vec![
            "Last session".into(),
            format!("{}h {}m played", mins / 60, mins % 60),
            format!("{} kills, {} deaths, K/D {kd:.2}", p.kills_total, p.deaths_total),
            format!("{pops} totems popped, best combo {}", p.best_combo),
        ]);
        s.pvp.session_start = None;
        s.pvp.kills_total = 0;
        s.pvp.deaths_total = 0;
        s.pvp.best_combo = 0;
        s.pvp.pops.clear();
    });
}

/// Ambience: a few soft specks drifting round you (fireflies at night, embers in the Nether, dust by day), as places
/// in the world that follow you.
fn ambience_points(me: [f64; 3], t: f32) -> Vec<([f64; 3], f32)> {
    (0..24).map(|i| {
        let f = i as f32;
        let a = f * 2.399 + t * 0.05 * (1.0 + (f * 0.37).sin());
        let r = 3.0 + (f * 1.7).sin().abs() * 9.0;
        let y = 0.6 + (f * 0.9).cos().abs() * 3.0 + (t * 0.4 + f).sin() * 0.4;
        let glow = 0.5 + 0.5 * (t * 2.0 + f * 1.3).sin();
        ([me[0] + (a.cos() * r) as f64, me[1] + y as f64, me[2] + (a.sin() * r) as f64], glow)
    }).collect()
}

/// A box's twelve edges, drawn as thin quads through the camera.
fn box_edges(cam: &Cam, min: [f64; 3], max: [f64; 3], w: f32, h: f32, argb: u32, out: &mut Vec<Shape>) {
    let c = |x: usize, y: usize, z: usize| [if x == 0 { min[0] } else { max[0] }, if y == 0 { min[1] } else { max[1] }, if z == 0 { min[2] } else { max[2] }];
    let edges = [(c(0,0,0), c(1,0,0)), (c(0,0,1), c(1,0,1)), (c(0,1,0), c(1,1,0)), (c(0,1,1), c(1,1,1)),
                 (c(0,0,0), c(0,1,0)), (c(1,0,0), c(1,1,0)), (c(0,0,1), c(0,1,1)), (c(1,0,1), c(1,1,1)),
                 (c(0,0,0), c(0,0,1)), (c(1,0,0), c(1,0,1)), (c(0,1,0), c(0,1,1)), (c(1,1,0), c(1,1,1))];
    for (a, b) in edges {
        if let (Some(a), Some(b)) = (project(cam, a, w, h), project(cam, b, w, h)) {
            let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
            let len = (dx * dx + dy * dy).sqrt().max(0.001);
            let n = [-dy / len * 0.6, dx / len * 0.6];
            out.push(Shape::Quad([[a[0] - n[0], a[1] - n[1]], [b[0] - n[0], b[1] - n[1]], [b[0] + n[0], b[1] + n[1]], [a[0] + n[0], a[1] + n[1]]], argb));
        }
    }
}

/// The last group's shapes.
#[allow(clippy::too_many_arguments)]
pub fn more_shapes(settings: &Settings, others: &[Other], target: Option<&Other>, inv: Option<&Inventory>, world: &World, using: Option<(&str, u32)>,
                   attack: f32, block: Option<&str>, aimed: Option<[i32; 3]>, me: Option<[f64; 3]>, friends: &[String], cam: Option<&Cam>,
                   w: f32, h: f32, width: &dyn Fn(&str) -> f32) -> Vec<Shape> {
    let mut out = Vec::new();
    let (cx, cy) = ((w / 2.0).floor(), (h / 2.0).floor());
    let held = inv.and_then(|i| i.main_hand()).map(|s| s.id.clone()).unwrap_or_default();
    let label = |out: &mut Vec<Shape>, text: String, at: [f32; 2], rgb: u32| {
        let tw = width(&text);
        out.push(Shape::Fill([at[0] - tw / 2.0 - 2.0, at[1] - 1.0, tw + 4.0, 10.0], 0x9000_0000));
        out.push(Shape::Text(text, [at[0] - tw / 2.0, at[1]], rgb, false));
    };
    STATE.with_borrow(|s| {
        let x = &s.extra;
        if let Some(cam) = cam {
            let t = s.pvp.session_start.map_or(0.0, |t| t.elapsed().as_secs_f32());
            if let (true, Some(me)) = (settings.on(AMBIENCE), me) {
                for (p, glow) in ambience_points(me, t) {
                    if let Some([px, py]) = project(cam, p, w, h) {
                        let a = (60.0 + glow * 150.0) as u32;
                        out.push(Shape::Fill([px - 1.0, py - 1.0, 2.0, 2.0], a << 24 | 0xE8F57A));
                    }
                }
            }
            let show_boxes = settings.on(HITBOXES) && (!settings.on(AUTO_HITBOX) || is_weapon(&held));
            for o in others {
                let friend = friends.iter().any(|f| f.eq_ignore_ascii_case(&o.name));
                if show_boxes && !friend {
                    let aimed_full = target.is_some_and(|t| t.id == o.id) && attack >= 0.95;
                    box_edges(cam, [o.pos[0] - 0.3, o.pos[1], o.pos[2] - 0.3], [o.pos[0] + 0.3, o.pos[1] + 1.8, o.pos[2] + 0.3], w, h,
                              if aimed_full { 0xFFFF_4040 } else { 0xB0FF_FFFF }, &mut out);
                }
                let mut tags: Vec<(String, u32)> = Vec::new();
                if settings.on(FRIENDS) && friend {
                    tags.push(("F".into(), 0x5CE08A));
                }
                if settings.on(EATING) && o.using > 0 {
                    let item = o.equipment.get(if o.using_off_hand { 1 } else { 0 }).cloned().flatten().map(|s| s.id).unwrap_or_default();
                    let what = if item.contains("golden_apple") { Some("EATING GAPPLE") } else if item.contains("potion") || item.ends_with("milk_bucket") { Some("DRINKING") }
                        else if food(&item).is_some() { Some("EATING") } else { None };
                    if let Some(what) = what {
                        tags.push((what.into(), 0xFF5A5A));
                    }
                }
                if settings.on(SHIELD_HELP) {
                    if let Some(at) = x.shield_down.get(&o.id) {
                        tags.push((format!("Shield {:.1}s", 5.0 - at.elapsed().as_secs_f32()), 0xFF5A5A));
                    } else if o.equipment.iter().take(2).flatten().any(|s| s.id == "minecraft:shield") {
                        tags.push(("Shield".into(), 0x5CE08A));
                    }
                }
                if let (true, Some(at)) = (settings.on(PEARLS), x.thrown.get(&o.id)) {
                    tags.push((format!("Pearl {:.1}s", (15.0 - at.elapsed().as_secs_f32()).max(0.0)), 0xB884FF));
                }
                let mut y_off = 3.35;
                for (text, rgb) in tags {
                    if let Some(at) = project(cam, [o.pos[0], o.pos[1] + y_off, o.pos[2]], w, h) {
                        label(&mut out, text, at, rgb);
                    }
                    y_off += 0.3;
                }
            }
            if settings.on(ITEM_NAMES) {
                if let Some(me) = me {
                    for (p, id, count) in &world.dropped {
                        let d2 = (p[0] - me[0]).powi(2) + (p[1] - me[1]).powi(2) + (p[2] - me[2]).powi(2);
                        if d2 > 24.0 * 24.0 {
                            continue;
                        }
                        if let Some(at) = project(cam, [p[0], p[1] + 0.6, p[2]], w, h) {
                            let gold = VALUABLE.iter().any(|v| id.contains(v));
                            let text = if *count > 1 { format!("{} x{count}", item_name(id)) } else { item_name(id) };
                            label(&mut out, text, at, if gold { 0xFFC83D } else { 0xFFFFFF });
                        }
                    }
                }
            }
            if let (true, Some(pos)) = (settings.on(CRYSTAL_PREV), aimed) {
                if held == "minecraft:end_crystal" {
                    let base = block.is_some_and(|b| b.contains("obsidian") || b.contains("bedrock"));
                    let min = [pos[0] as f64, pos[1] as f64 + 1.0, pos[2] as f64];
                    box_edges(cam, min, [min[0] + 1.0, min[1] + 2.0, min[2] + 1.0], w, h, if base { 0xFF5C_E08A } else { 0xFFFF_5A5A }, &mut out);
                }
            }
            // Mace Aim: falling with a mace, a line to the player below and the bonus damage.
            if let (true, Some(me)) = (settings.on(MACE_AIM), me) {
                if held == "minecraft:mace" && s.pvp.fallen > 1.5 {
                    let below = others.iter().filter(|o| o.pos[1] < me[1] && ((o.pos[0] - me[0]).powi(2) + (o.pos[2] - me[2]).powi(2)).sqrt() < 6.0)
                        .min_by(|a, b| a.pos[1].total_cmp(&b.pos[1]).reverse());
                    if let Some(o) = below {
                        if let Some(at) = project(cam, [o.pos[0], o.pos[1] + 1.0, o.pos[2]], w, h) {
                            let (dx, dy) = (at[0] - cx, at[1] - cy);
                            let len = (dx * dx + dy * dy).sqrt().max(0.001);
                            let n = [-dy / len, dx / len];
                            out.push(Shape::Quad([[cx - n[0], cy - n[1]], [at[0] - n[0], at[1] - n[1]], [at[0] + n[0], at[1] + n[1]], [cx + n[0], cy + n[1]]], 0xFFFF_C83D));
                        }
                    }
                    label(&mut out, format!("+{:.1} mace", mace_bonus(s.pvp.fallen) / 2.0), [cx, cy + 38.0], 0xFFC83D);
                }
            }
        }
        if let (true, Some((item, ticks))) = (settings.on(BOW), using) {
            if let Some(c) = bow_charge(item, ticks) {
                label(&mut out, format!("{}%", (c * 100.0).round() as i32), [cx, cy + 14.0], if c >= 1.0 { 0x5CE08A } else { 0xFFFFFF });
            }
        }
        if settings.on(RETOTEM) {
            if let Some(d) = x.retotem {
                label(&mut out, format!("Re-totem {:.2}s", d.as_secs_f32()), [cx, cy + 48.0], if d.as_millis() < 300 { 0x5CE08A } else { 0xFFC83D });
            } else if x.popped_at.is_some() {
                label(&mut out, "Re-totem!".into(), [cx, cy + 48.0], 0xFF5A5A);
            }
        }
        if settings.on(GOAL) {
            let goal: u32 = GOALS[settings.v[GOAL_KILLS]].0.parse().unwrap_or(10);
            let done = s.pvp.kills_total;
            let text = if done >= goal { format!("Goal reached: {done}/{goal} kills") } else { format!("Goal: {done}/{goal} kills") };
            out.push(Shape::Text(text, [4.0, h - 12.0], if done >= goal { 0x5CE08A } else { 0xFFFFFF }, true));
        }
        if settings.on(INTEL) {
            if let Some(f) = &s.pvp.fight {
                let gapples = x.gapples.get(&f.with).copied().unwrap_or(0);
                let lines = [format!("{}: {} pops, {} gapples", f.with, f.pops, gapples), format!("Your hits {}", f.hits)];
                let mut y = 40.0;
                for l in lines {
                    out.push(Shape::Text(l, [4.0, y], 0xFFC83D, true));
                    y += 10.0;
                }
            }
        }
        if let (true, Some(lines)) = (settings.on(SUMMARY), &x.summary) {
            if x.summary_shown.is_none_or(|t| t.elapsed() < Duration::from_secs(8)) {
                let wide = lines.iter().map(|l| width(l)).fold(0.0, f32::max) + 12.0;
                let (bx, by) = (cx - wide / 2.0, h * 0.3);
                out.push(Shape::Fill([bx, by, wide, lines.len() as f32 * 11.0 + 8.0], BACKGROUND));
                for (i, l) in lines.iter().enumerate() {
                    out.push(Shape::Text(l.clone(), [bx + 6.0, by + 5.0 + i as f32 * 11.0], if i == 0 { 0xFFC83D } else { 0xFFFFFF }, true));
                }
            }
        }
    });
    STATE.with_borrow_mut(|s| {
        if s.extra.summary.is_some() && s.extra.summary_shown.is_none() {
            s.extra.summary_shown = Some(Instant::now());
        }
    });
    out
}

/// The last group's state.
#[derive(Default)]
struct Extra {
    using: std::collections::HashMap<u32, (i32, Option<String>)>,
    gapples: std::collections::HashMap<String, u32>,
    pearls: std::collections::HashMap<u32, u32>,
    thrown: std::collections::HashMap<u32, Instant>,
    shield_down: std::collections::HashMap<u32, Instant>,
    popped_at: Option<Instant>,
    retotem: Option<Duration>,
    summary: Option<Vec<String>>,
    summary_shown: Option<Instant>,
}

/// Whether you're falling now (Crit Indicator): going down since the last tick.
pub fn falling() -> bool {
    STATE.with_borrow(|s| s.pvp.last_dy < -0.04)
}

/// The player you look at most directly within 6 blocks (or the last one you hit, for 3 seconds).
pub fn target<'a>(others: &'a [Other], eye: [f64; 3], yaw: f32, pitch: f32) -> Option<&'a Other> {
    let (y, p) = ((yaw as f64).to_radians(), (pitch as f64).to_radians());
    let look = [-y.sin() * p.cos(), -p.sin(), y.cos() * p.cos()];
    let remembered = STATE.with_borrow(|s| s.pvp.target.filter(|t| t.1.elapsed() < Duration::from_secs(3)).map(|t| t.0));
    let best = others.iter().filter_map(|o| {
        let d = [o.pos[0] - eye[0], o.pos[1] + 1.0 - eye[1], o.pos[2] - eye[2]];
        let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        if len > 6.0 || len < 0.01 {
            return None;
        }
        let cos = (d[0] * look[0] + d[1] * look[1] + d[2] * look[2]) / len;
        (cos > 0.95).then_some((o, cos))
    }).max_by(|a, b| a.1.total_cmp(&b.1)).map(|(o, _)| o);
    best.or_else(|| remembered.and_then(|id| others.iter().find(|o| o.id == id)))
}

fn ring_rgb(settings: &Settings, t: f32) -> u32 {
    match settings.v[JUMP_COLOR] {
        1 => 0xFFFFFF,
        2 => {
            let h = (t * 6.0) % 6.0;
            let x = 1.0 - ((h % 2.0) - 1.0).abs();
            let (r, g, b) = match h as u32 { 0 => (1.0, x, 0.0), 1 => (x, 1.0, 0.0), 2 => (0.0, 1.0, x), 3 => (0.0, x, 1.0), 4 => (x, 0.0, 1.0), _ => (1.0, 0.0, x) };
            ((r * 255.0) as u32) << 16 | ((g * 255.0) as u32) << 8 | (b * 255.0) as u32
        }
        _ => settings.accent(),
    }
}

/// Everything the combat modules draw this frame: in the world (through the camera) and on the HUD.
pub fn pvp_shapes(settings: &Settings, others: &[Other], target: Option<&Other>, cam: Option<&Cam>, w: f32, h: f32, width: &dyn Fn(&str) -> f32) -> Vec<Shape> {
    let mut out = Vec::new();
    let now = Instant::now();
    STATE.with_borrow(|s| {
        let p = &s.pvp;
        if let Some(cam) = cam {
            // Jump Circles: a ring on the ground, growing and fading.
            for (at, start) in &p.rings {
                let t = start.elapsed().as_secs_f32() / 0.7;
                let radius = 0.2 + 1.1 * t;
                let alpha = (((1.0 - t) * 200.0).max(0.0) as u32) << 24;
                let n = 24;
                for i in 0..n {
                    let a0 = i as f64 / n as f64 * std::f64::consts::TAU;
                    let a1 = (i + 1) as f64 / n as f64 * std::f64::consts::TAU;
                    let pt = |a: f64, r: f64| [at[0] + a.cos() * r, at[1] + 0.05, at[2] + a.sin() * r];
                    let (r0, r1) = (radius as f64, (radius - 0.12) as f64);
                    let corners = [pt(a0, r0), pt(a1, r0), pt(a1, r1), pt(a0, r1)].map(|c| project(cam, c, w, h));
                    if let [Some(a), Some(b), Some(c), Some(d)] = corners {
                        out.push(Shape::Quad([a, b, c, d], alpha | ring_rgb(settings, i as f32 / n as f32)));
                    }
                }
            }
            // Nametag Health: above each player's head.
            if settings.on(NAMETAG_HP) {
                for o in others {
                    if let Some([x, y]) = project(cam, [o.pos[0], o.pos[1] + 2.45, o.pos[2]], w, h) {
                        let text = format!("{:.1}", (o.health + o.absorption) / 2.0);
                        let tw = width(&text);
                        out.push(Shape::Fill([x - tw / 2.0 - 2.0, y - 1.0, tw + 4.0, 10.0], 0x8000_0000));
                        out.push(Shape::Text(text, [x - tw / 2.0, y], if o.absorption > 0.0 { 0xFFC83D } else { health_rgb(o.health / 20.0) }, false));
                    }
                }
            }
            // Damage Numbers: rising and fading over a second.
            for (at, amount, start) in &p.damage {
                let t = start.elapsed().as_secs_f32();
                if let Some([x, y]) = project(cam, [at[0], at[1] + t as f64 * 0.8, at[2]], w, h) {
                    let text = format!("-{:.1}", amount / 2.0);
                    let tw = width(&text);
                    out.push(Shape::Text(text, [x - tw / 2.0, y], 0xFF5A5A, true));
                }
            }
        }
        let (cx, cy) = ((w / 2.0).floor(), (h / 2.0).floor());
        // Hitmarker: an X round the crosshair for a moment.
        if settings.on(HITMARKER) && p.hitmarker.is_some_and(|t| t.elapsed() < Duration::from_millis(180)) {
            for (dx, dy) in [(1.0, 1.0), (-1.0, 1.0), (1.0, -1.0), (-1.0, -1.0)] {
                let (a, b) = ([cx + 3.0 * dx, cy + 3.0 * dy], [cx + 7.0 * dx, cy + 7.0 * dy]);
                let n = [0.7 * dy, -0.7 * dx];
                out.push(Shape::Quad([[a[0] - n[0], a[1] - n[1]], [b[0] - n[0], b[1] - n[1]], [b[0] + n[0], b[1] + n[1]], [a[0] + n[0], a[1] + n[1]]], 0xFFFF_FFFF));
            }
        }
        // Kill Streaks: big text at the top for two seconds.
        if let Some((name, at)) = p.streak.filter(|s| s.1.elapsed() < Duration::from_secs(2)) {
            let _ = at;
            let tw = width(name);
            out.push(Shape::Text(name.to_owned(), [cx - tw / 2.0, h * 0.22], 0xFFC83D, true));
        }
        // Combat Tag Timer: under the crosshair's left.
        if let Some(until) = p.combat_until.filter(|u| *u > now) {
            let text = format!("Combat {}s", (until - now).as_secs() + 1);
            let tw = width(&text);
            out.push(Shape::Text(text, [cx - tw / 2.0, cy - 22.0], 0xFF5A5A, true));
        }
        // Mention Ping: who wrote your name, for three seconds.
        if let Some((line, at)) = p.mention.as_ref().filter(|m| m.1.elapsed() < Duration::from_secs(3)) {
            let _ = at;
            let tw = width(line);
            out.push(Shape::Fill([cx - tw / 2.0 - 4.0, 4.0, tw + 8.0, 12.0], BACKGROUND));
            out.push(Shape::Text(line.clone(), [cx - tw / 2.0, 6.0], 0xFFC83D, true));
        }
        // Totem Pop Counter: top right.
        if settings.on(POPS) && !p.pops.is_empty() {
            let mut rows: Vec<(&String, &u32)> = p.pops.iter().collect();
            rows.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
            let mut y = 4.0;
            for (name, n) in rows.into_iter().take(8) {
                let text = format!("{name}: {n}");
                let tw = width(&text);
                out.push(Shape::Fill([w - tw - 10.0, y - 2.0, tw + 6.0, 11.0], BACKGROUND));
                out.push(Shape::Text(text, [w - tw - 7.0, y], 0xFFFFFF, false));
                y += 12.0;
            }
        }
        // Target HUD: right of the crosshair.
        if settings.on(TARGET) {
            if let Some(t) = target {
                let (x, y) = (cx + 24.0, cy - 30.0);
                let pops = p.pops.get(&t.name).copied().unwrap_or(0);
                let name = if pops > 0 { format!("{} ({pops} pops)", t.name) } else { t.name.clone() };
                let wide = (width(&name) + 12.0).max(110.0);
                out.push(Shape::Fill([x, y, wide, 48.0], BACKGROUND));
                out.push(Shape::Fill([x, y, 2.0, 48.0], 0xFF00_0000 | settings.accent()));
                out.push(Shape::Text(name, [x + 6.0, y + 4.0], 0xFFFFFF, true));
                let hp = (t.health / 20.0).clamp(0.0, 1.0);
                out.push(Shape::Fill([x + 6.0, y + 15.0, wide - 12.0, 4.0], 0xA000_0000));
                out.push(Shape::Fill([x + 6.0, y + 15.0, ((wide - 12.0) * hp).round(), 4.0], 0xFF00_0000 | health_rgb(hp)));
                if t.absorption > 0.0 {
                    out.push(Shape::Fill([x + 6.0, y + 19.0, ((wide - 12.0) * (t.absorption / 20.0).clamp(0.0, 1.0)).round(), 1.0], 0xFFFF_C83D));
                }
                let hearts = format!("{:.1}", (t.health + t.absorption) / 2.0);
                let hw = width(&hearts);
                out.push(Shape::Text(hearts, [x + wide - 6.0 - hw, y + 4.0], health_rgb(hp), true));
                // Armour (head first) and the held item.
                let mut ix = x + 6.0;
                for slot in [5usize, 4, 3, 2, 0] {
                    if let Some(Some(st)) = t.equipment.get(slot) {
                        out.push(Shape::Item(st.id.clone(), [ix, y + 26.0]));
                        ix += 18.0;
                    }
                }
            }
        }
    });
    out
}

/// Low Health Warning: red edges round the screen, pulsing about once a second, when health is at or under the
/// setting (hearts). `millis` is any running clock.
pub fn low_health_shapes(settings: &Settings, h: &Health, millis: u64) -> Vec<Shape> {
    let mut out = Vec::new();
    let at: f32 = LOW_AT[settings.v[LOW_HEALTH_AT]].0.parse().unwrap_or(4.0);
    if !settings.on(LOW_HEALTH) || h.health <= 0.0 || h.health > at * 2.0 {
        return out;
    }
    let pulse = 0.5 + 0.5 * ((millis % 1000) as f32 / 1000.0 * std::f32::consts::TAU).sin();
    // Lower health, stronger red.
    let strength = (1.0 - h.health / (at * 2.0)).clamp(0.0, 1.0) * 0.5 + 0.5;
    let (w, ht) = (h.width, h.height);
    for (i, band) in [24.0_f32, 14.0, 6.0].iter().enumerate() {
        let alpha = ((0.12 + 0.16 * pulse) * strength * (i as f32 + 1.0) / 2.0 * 255.0).round().clamp(0.0, 255.0) as u32;
        let c = alpha << 24 | 0xD0_1010;
        out.push(Shape::Fill([0.0, 0.0, w, *band], c));
        out.push(Shape::Fill([0.0, ht - band, w, *band], c));
        out.push(Shape::Fill([0.0, *band, *band, ht - 2.0 * band], c));
        out.push(Shape::Fill([w - band, *band, *band, ht - 2.0 * band], c));
    }
    out
}

/// Whether the heartbeat should play now: about once a second while the warning shows (faster when lower).
pub fn heartbeat_due(settings: &Settings, h: &Health) -> bool {
    let at: f32 = LOW_AT[settings.v[LOW_HEALTH_AT]].0.parse().unwrap_or(4.0);
    if !settings.on(LOW_HEALTH) || !settings.on(HEARTBEAT) || h.health <= 0.0 || h.health > at * 2.0 {
        return false;
    }
    let gap = Duration::from_millis(600 + (h.health / (at * 2.0) * 600.0) as u64);
    STATE.with_borrow_mut(|s| {
        let due = s.heartbeat.is_none_or(|at| at.elapsed() >= gap);
        if due {
            s.heartbeat = Some(Instant::now());
        }
        due
    })
}

/// Food Preview on Horizon: the saturation as a gold number just right of the hunger bar.
pub fn saturation_shapes(settings: &Settings, h: &Health) -> Vec<Shape> {
    if !settings.on(SATURATION) {
        return Vec::new();
    }
    let text = if (h.saturation - h.saturation.round()).abs() < 0.05 { format!("{}", h.saturation.round() as i32) } else { format!("{:.1}", h.saturation) };
    vec![Shape::Text(text, [(h.width / 2.0).floor() + 93.0, h.height - 38.0], 0xFFC83D, true)]
}

/// Vanilla items' durability (`max_damage`): Horizon doesn't hand mods the item's own yet.
pub fn max_durability(item: &str) -> Option<u32> {
    let id = item.strip_prefix("minecraft:").unwrap_or(item);
    let tier = |material: &str| -> Option<u32> {
        Some(match material { "wooden" => 59, "stone" => 131, "copper" => 190, "iron" => 250, "golden" => 32, "diamond" => 1561, "netherite" => 2031, _ => return None })
    };
    for tool in ["_sword", "_pickaxe", "_axe", "_shovel", "_hoe", "_spear"] {
        if let Some(material) = id.strip_suffix(tool) {
            return tier(material);
        }
    }
    let armor = |material: &str, piece: usize| -> Option<u32> {
        let base = match material { "leather" => 5, "copper" => 11, "chainmail" | "iron" => 15, "golden" => 7, "diamond" => 33, "netherite" => 37, _ => return None };
        Some(base * [11, 16, 15, 13][piece])
    };
    for (i, piece) in ["_helmet", "_chestplate", "_leggings", "_boots"].iter().enumerate() {
        if let Some(material) = id.strip_suffix(piece) {
            return armor(material, i);
        }
    }
    Some(match id {
        "turtle_helmet" => 275, "elytra" => 432, "trident" => 250, "mace" => 500, "shield" => 336, "bow" => 384, "crossbow" => 465,
        "fishing_rod" => 64, "shears" => 238, "flint_and_steel" => 64, "carrot_on_a_stick" => 25, "warped_fungus_on_a_stick" => 100,
        "brush" => 64, "wolf_armor" => 64,
        _ => return None,
    })
}

/// Vanilla foods: (hunger, saturation, effects).
pub fn food(item: &str) -> Option<(u32, f32, &'static str)> {
    let id = item.strip_prefix("minecraft:").unwrap_or(item);
    Some(match id {
        "apple" | "chorus_fruit" => (4, 2.4, ""), "baked_potato" | "bread" | "cooked_cod" | "cooked_rabbit" => (5, 6.0, ""),
        "beef" | "porkchop" | "rabbit" => (3, 1.8, ""), "beetroot" => (1, 1.2, ""), "beetroot_soup" | "mushroom_stew" | "cooked_chicken" => (6, 7.2, ""),
        "carrot" => (3, 3.6, ""), "cooked_beef" | "cooked_porkchop" => (8, 12.8, ""), "cooked_mutton" | "cooked_salmon" => (6, 9.6, ""),
        "cookie" => (2, 0.4, ""), "dried_kelp" | "potato" => (1, 0.6, ""), "golden_carrot" => (6, 14.4, ""), "honey_bottle" => (6, 1.2, ""),
        "melon_slice" | "chicken" | "mutton" => (2, 1.2, ""), "poisonous_potato" => (2, 1.2, "Poison (60%)"), "pufferfish" => (1, 0.2, "Poison II, Nausea, Hunger III"),
        "pumpkin_pie" => (8, 4.8, ""), "rabbit_stew" => (10, 12.0, ""), "rotten_flesh" => (4, 0.8, "Hunger (80%)"), "salmon" | "cod" | "sweet_berries" | "glow_berries" => (2, 0.4, ""),
        "spider_eye" => (2, 3.2, "Poison"), "suspicious_stew" => (6, 7.2, ""), "tropical_fish" => (1, 0.2, ""),
        "golden_apple" => (4, 9.6, "Regeneration II (5s), Absorption (2:00)"),
        "enchanted_golden_apple" => (4, 9.6, "Regeneration II (20s), Absorption IV (2:00), Resistance (5:00), Fire Resistance (5:00)"),
        _ => return None,
    })
}

/// Better Tooltips' lines for an item: (text, 0xRRGGBB), added under the game's own.
pub fn tooltip_lines(settings: &Settings, item: &str, damage: u32) -> Vec<(String, u32)> {
    let mut out = Vec::new();
    if !settings.on(TOOLTIPS) {
        return out;
    }
    if let Some(max) = max_durability(item) {
        let left = max.saturating_sub(damage);
        out.push((format!("Durability: {left} / {max}"), health_rgb(left as f32 / max as f32)));
    }
    if let Some((hunger, saturation, effects)) = food(item) {
        out.push((format!("Restores {hunger} hunger, {saturation:.1} saturation"), 0xFFC83D));
        if !effects.is_empty() {
            out.push((effects.to_owned(), 0x9C6BFF));
        }
    }
    out
}

/// Shulker Preview: whether a shulker box's tooltip gets the grid, and its size (9 x 3 slots of 18 pixels).
pub fn shulker_size(settings: &Settings, item: &str, has_contents: bool) -> Option<[f32; 2]> {
    (settings.on(SHULKER) && has_contents && item.ends_with("shulker_box")).then_some([9.0 * 18.0 + 4.0, 3.0 * 18.0 + 4.0])
}

/// The shulker box's colour (ARGB, see-through) from its id ("minecraft:red_shulker_box"; plain is purple).
pub fn shulker_color(item: &str) -> u32 {
    let id = item.strip_prefix("minecraft:").unwrap_or(item);
    let rgb = match id.strip_suffix("_shulker_box").unwrap_or("") {
        "white" => 0xE8ECEC, "orange" => 0xF07613, "magenta" => 0xBD44B3, "light_blue" => 0x3AAFD9, "yellow" => 0xF8C627,
        "lime" => 0x70B919, "pink" => 0xED8DAC, "gray" => 0x3E4447, "light_gray" => 0x8E8E86, "cyan" => 0x158991,
        "purple" => 0x792AAC, "blue" => 0x35399D, "brown" => 0x724728, "green" => 0x546D1B, "red" => 0xA12722, "black" => 0x141519,
        _ => 0x8B5E8B,
    };
    0xC000_0000 | rgb
}

/// Where slot `slot` (0..27) of the grid is drawn, from the component's top left.
pub fn shulker_slot(origin: [f32; 2], slot: u8) -> [f32; 2] {
    let (col, row) = ((slot % 9) as f32, (slot / 9) as f32);
    [origin[0] + 2.0 + col * 18.0, origin[1] + 2.0 + row * 18.0]
}

/// HUD Theme on tooltips: (background, top border, bottom border) as ARGB, or None to leave vanilla's.
pub fn tooltip_colors(settings: &Settings) -> Option<(u32, u32, u32)> {
    if !settings.on(THEME) || !settings.on(THEME_TOOLTIPS) {
        return None;
    }
    let (background, _) = settings.panel_colors();
    let accent = settings.accent();
    Some((0xF000_0000 | (background & 0xFF_FFFF), 0xFF00_0000 | accent, 0x8000_0000 | accent))
}

/// Health HUD's shapes, in screen (GUI) coordinates: the number and/or bar centred under the crosshair.
pub fn health_shapes(settings: &Settings, h: &Health, text_width: &dyn Fn(&str) -> f32) -> Vec<Shape> {
    let mut out = Vec::new();
    if !settings.on(HEALTH) || h.max <= 0.0 {
        return out;
    }
    let show = settings.v[HEALTH_SHOW];
    let points = settings.v[HEALTH_UNIT] == 1;
    let fraction = h.health / h.max;
    let cx = (h.width / 2.0).floor();
    let mut y = (h.height / 2.0).floor() + 10.0;
    if show != 2 {
        let mut text = health_text(h.health, h.max, points);
        if h.absorption > 0.0 {
            text.push_str(&format!(" +{}", health_text(h.absorption, h.max, points)));
        }
        let w = text_width(&text);
        out.push(Shape::Text(text, [cx - (w / 2.0).floor(), y], health_rgb(fraction), true));
        y += 10.0;
    }
    if show != 1 {
        let width = 40.0;
        let x = cx - width / 2.0;
        out.push(Shape::Fill([x - 1.0, y - 1.0, width + 2.0, 5.0], 0xA000_0000));
        let filled = (width * fraction.clamp(0.0, 1.0)).round();
        out.push(Shape::Fill([x, y, filled, 3.0], 0xFF00_0000 | health_rgb(fraction)));
        if h.absorption > 0.0 {
            let gold = (width * (h.absorption / h.max).clamp(0.0, 1.0)).round();
            out.push(Shape::Fill([x, y + 3.0, gold, 1.0], 0xFFFF_C83D));
        }
    }
    out
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

/// The settings, read once through `get`.
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    STATE.with_borrow_mut(|s| s.settings.get_or_insert_with(|| Settings::read(get)).clone())
}

/// Replaces the settings (a settings page changed them).
pub fn set_settings(settings: &Settings) {
    STATE.with_borrow_mut(|s| s.settings = Some(settings.clone()));
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
        let smooth = s.settings.as_ref().is_none_or(|st| st.on(SMOOTH));
        Some((factor, smooth, 1.0 / factor))
    })
}

/// Toggle Sneak: the sneak key as the game read it this tick (`down`); true when the player sneaks.
pub fn sneak(on: bool, down: bool) -> bool {
    STATE.with_borrow_mut(|s| {
        if !on {
            s.sneaking = false;
            s.sneak_down = down;
            return down;
        }
        if down && !s.sneak_down {
            s.sneaking = !s.sneaking;
        }
        s.sneak_down = down;
        s.sneaking
    })
}

/// Whether Toggle Sneak keeps the player sneaking.
pub fn sneaking() -> bool {
    STATE.with_borrow(|s| s.sneaking)
}

/// The block you aim at this frame (the highlight hook, whether Block Outline is on or not).
pub fn set_aimed(pos: [i32; 3]) {
    STATE.with_borrow_mut(|s| s.aimed = Some(pos));
}

/// Takes the block aimed at this frame (Anchor Helper): none when you aim at no block.
pub fn take_aimed() -> Option<[i32; 3]> {
    STATE.with_borrow_mut(|s| s.aimed.take())
}

/// Block Outline: the block the game is about to outline this frame, or none.
pub fn set_outlined(pos: Option<[i32; 3]>) {
    STATE.with_borrow_mut(|s| s.outlined = pos);
}

pub fn outlined() -> Option<[i32; 3]> {
    STATE.with_borrow(|s| s.outlined)
}

/// Follows the player for Speed Display, Session Clock and FPS Graph: their position (None out of a world) and the
/// frame rate at `now`. Returns the horizontal speed in blocks per second (measured over a quarter second) and the
/// seconds since they joined.
pub fn track(player: Option<[f64; 3]>, fps: u32, now: Instant) -> (f64, u64) {
    STATE.with_borrow_mut(|s| {
        if s.fps_at.is_none_or(|at| now.duration_since(at) >= Duration::from_millis(250)) {
            s.fps_at = Some(now);
            if s.fps.len() >= GRAPH_SAMPLES {
                s.fps.pop_front();
            }
            s.fps.push_back(fps);
        }
        let Some(pos) = player else {
            s.sample = None;
            s.speed = 0.0;
            s.joined = None;
            return (0.0, 0);
        };
        let joined = *s.joined.get_or_insert(now);
        match s.sample {
            Some((at, last)) => {
                let dt = now.duration_since(at).as_secs_f64();
                if dt >= 0.25 {
                    let (dx, dz) = (pos[0] - last[0], pos[2] - last[2]);
                    s.speed = (dx * dx + dz * dz).sqrt() / dt;
                    s.sample = Some((now, pos));
                }
            }
            None => s.sample = Some((now, pos)),
        }
        (s.speed, now.duration_since(joined).as_secs())
    })
}

/// FPS Graph's readings, oldest first.
pub fn fps_history() -> Vec<u32> {
    STATE.with_borrow(|s| s.fps.iter().copied().collect())
}

/// "4:05", or "1:02:09" past an hour.
pub fn clock(secs: u64) -> String {
    let (h, m, s) = (secs / 3600, secs / 60 % 60, secs % 60);
    if h > 0 { format!("{h}:{m:02}:{s:02}") } else { format!("{m}:{s:02}") }
}

/// What the panel shows this frame: the readings its lines need.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Readings {
    pub fps: u32,
    /// Attack and use clicks in the last second.
    pub cps: (u32, u32),
    /// The player's position and yaw, in a world.
    pub player: Option<([f64; 3], f32)>,
    /// Blocks per second, and seconds since joining this world.
    pub speed: f64,
    pub session: u64,
    /// Toggle Sneak keeps the player sneaking.
    pub sneaking: bool,
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
        if s.on(SPEED) {
            out.push(("Speed", format!("{:.2} b/s", r.speed)));
        }
        if s.on(SESSION) {
            out.push(("Session", clock(r.session)));
        }
        if s.on(SNEAK) && r.sneaking {
            out.push(("Sneak", "Toggled".to_owned()));
        }
    }
    out
}

/// One thing to draw: a rectangle `[x, y, w, h]` in 0xAARRGGBB, or text at `[x, y]` in 0xRRGGBB with a shadow or not.
#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Fill([f32; 4], u32),
    Text(String, [f32; 2], u32, bool),
    /// An item's icon (its id), 16 pixels square, top left at the point.
    Item(String, [f32; 2]),
    /// Four corners in order (a slanted bar, a wedge): what a rectangle can't draw.
    Quad([[f32; 2]; 4], u32),
}

/// The panel's background, as the HUD's own boxes are drawn.
pub const BACKGROUND: u32 = 0x9010_1014;
pub const WHITE: u32 = 0xFFFFFF;
const PAD: f32 = 4.0;
const STRIPE: f32 = 2.0;
const LINE: f32 = 10.0;
const GAP: f32 = 5.0;
const GRAPH_SAMPLES: usize = 60;

/// The panel for `lines` in `s`'s colours, measured with `width`: its shapes (from its top left) and its size. None
/// when there is nothing to show.
pub fn panel(lines: &[(&str, String)], s: &Settings, width: &dyn Fn(&str) -> f32) -> Option<(Vec<Shape>, f32, f32)> {
    if lines.is_empty() {
        return None;
    }
    let accent = s.accent();
    let (background, text) = s.panel_colors();
    let labels = lines.iter().map(|(l, _)| width(l)).fold(0.0, f32::max);
    let values = lines.iter().map(|(_, v)| width(v)).fold(0.0, f32::max);
    let w = STRIPE + PAD + labels + GAP + values + PAD;
    let h = PAD * 2.0 + lines.len() as f32 * LINE - 2.0;
    let mut out = vec![Shape::Fill([0.0, 0.0, w, h], background), Shape::Fill([0.0, 0.0, STRIPE, h], 0xFF00_0000 | accent)];
    for (i, (label, value)) in lines.iter().enumerate() {
        let y = PAD + i as f32 * LINE;
        out.push(Shape::Text((*label).to_owned(), [STRIPE + PAD, y], accent, true));
        out.push(Shape::Text(value.clone(), [STRIPE + PAD + labels + GAP, y], text, true));
    }
    Some((out, w, h))
}

/// FPS Graph's box: a bar per reading, scaled to the highest, with the newest number. None without readings.
pub fn graph(readings: &[u32], s: &Settings) -> Option<(Vec<Shape>, f32, f32)> {
    let newest = *readings.last()?;
    let (w, h, bars) = (GRAPH_SAMPLES as f32 + 8.0, 34.0, 20.0);
    let top = readings.iter().copied().max().unwrap_or(1).max(1) as f32;
    let accent = s.accent();
    let (background, text) = s.panel_colors();
    let mut out = vec![Shape::Fill([0.0, 0.0, w, h], background), Shape::Text(format!("{newest} FPS"), [4.0, 3.0], text, true)];
    let x0 = w - 4.0 - readings.len() as f32;
    for (i, &v) in readings.iter().enumerate() {
        let bh = (v as f32 / top * bars).max(1.0);
        out.push(Shape::Fill([x0 + i as f32, h - 3.0 - bh, 1.0, bh], 0xFF00_0000 | accent));
    }
    Some((out, w, h))
}

// ---- Browse: the website's gallery ----

/// Browse's tabs: (label, the website's kind). Presets are whole Java setups, so they're listed but not applied here.
pub const BROWSE_TABS: [(&str, &str); 3] = [("Crosshairs", "crosshair"), ("Themes", "theme"), ("Presets", "preset")];

#[derive(Clone, Debug, PartialEq)]
pub enum Look {
    Crosshair(Crosshair),
    Theme(Theme),
    Preset,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GalleryItem {
    pub title: String,
    pub author: String,
    pub look: Look,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Fetch {
    Loading,
    Ready(Vec<GalleryItem>),
    Failed,
}

static GALLERY: Mutex<[Option<Fetch>; 3]> = Mutex::new([None, None, None]);

/// The website's list (`/v1/gallery`) as Browse shows it: crosshairs with their grid, themes with their colours.
pub fn parse_gallery(text: &str) -> Vec<GalleryItem> {
    let Ok(v) = serde_json::from_str::<serde_json::Value>(text) else {
        return Vec::new();
    };
    let s = |x: &serde_json::Value| x.as_str().unwrap_or("").to_owned();
    let mut out = Vec::new();
    for item in v["items"].as_array().into_iter().flatten() {
        let p = &item["preview"];
        let look = match item["kind"].as_str() {
            Some("crosshair") => {
                let rows: Vec<String> = p["rows"].as_array().into_iter().flatten().filter_map(|r| r.as_str().map(str::to_owned)).collect();
                let Some(rgb) = p["color"].as_str().and_then(parse_color) else { continue };
                if !crosshair_ok(&rows) {
                    continue;
                }
                Look::Crosshair(Crosshair { rows, rgb: rgb & 0xFF_FFFF, size: p["size"].as_u64().unwrap_or(2).clamp(1, 8) as u32 })
            }
            Some("theme") => {
                let c = |k: &str| p["look"][k].as_str().or(p[k].as_str()).and_then(parse_color);
                let (Some(background), Some(text), Some(accent)) = (c("background"), c("text"), c("accent")) else { continue };
                Look::Theme(Theme { background, text: text & 0xFF_FFFF, accent: accent & 0xFF_FFFF })
            }
            Some("preset") => Look::Preset,
            _ => continue,
        };
        out.push(GalleryItem { title: s(&item["title"]), author: s(&item["author"]), look });
    }
    out
}

/// Browse's tab `tab`: what is loaded, starting the download when it isn't (once, in the background).
pub fn gallery(tab: usize) -> Fetch {
    let mut g = GALLERY.lock().unwrap_or_else(|e| e.into_inner());
    match &g[tab] {
        Some(f) => f.clone(),
        None => {
            g[tab] = Some(Fetch::Loading);
            download(tab);
            Fetch::Loading
        }
    }
}

/// Downloads tab `tab`'s list off the game's thread.
#[cfg(not(test))]
fn download(tab: usize) {
    let url = format!("{WEBSITE}/v1/gallery?kind={}&sort=popular&limit=100", BROWSE_TABS[tab].1);
    std::thread::spawn(move || {
        let result = ureq::get(&url)
            .timeout(Duration::from_secs(15))
            .set("User-Agent", "Arsenal-Horizon")
            .call()
            .ok()
            .and_then(|r| r.into_string().ok());
        let fetched = match result {
            Some(text) => Fetch::Ready(parse_gallery(&text)),
            None => Fetch::Failed,
        };
        GALLERY.lock().unwrap_or_else(|e| e.into_inner())[tab] = Some(fetched);
    });
}

#[cfg(test)]
fn download(_tab: usize) {}

/// Forgets tab `tab`'s list, so the next look downloads it again (after a failure).
pub fn retry(tab: usize) {
    GALLERY.lock().unwrap_or_else(|e| e.into_inner())[tab] = None;
}

/// Applies `item` from Browse: a crosshair turns on Crosshair Editor with it, a theme turns on HUD Theme with it.
pub fn apply_item(item: &GalleryItem, s: &mut Settings) -> bool {
    match &item.look {
        Look::Crosshair(c) => {
            s.crosshair = Some(c.clone());
            s.v[CROSSHAIR] = 1;
            true
        }
        Look::Theme(t) => {
            s.theme = Some(*t);
            s.v[THEME] = 1;
            s.v[ACCENT] = ACCENTS.len() - 1;
            true
        }
        Look::Preset => false,
    }
}

// ---- The menu ----

/// What a click on the menu does.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    Close,
    Go(Page),
    Category(usize),
    /// Switch the Horizon module (index in `JAVA_MODULES`) on or off.
    Toggle(usize),
    /// Open the module's settings.
    Open(usize),
    Back,
    /// A click on setting `i` (index in `FIELDS`).
    Setting(usize),
    Tab(usize),
    /// Apply card `i` of the current Browse tab.
    Apply(usize),
    Retry,
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
const CARD_W: f32 = 92.0;
const CARD_H: f32 = 70.0;

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

/// Browse's card grid in a menu of size `w`×`h`: (columns, rows shown).
fn grid(w: f32, h: f32) -> (usize, usize) {
    let cols = (((w - 16.0) / (CARD_W + 4.0)).floor().max(1.0)) as usize;
    let rows = (((h - HEADER - 26.0) / (CARD_H + 4.0)).floor().max(1.0)) as usize;
    (cols, rows)
}

/// How many Horizon modules are on.
pub fn modules_on(s: &Settings) -> usize {
    ON_HORIZON.iter().filter(|m| s.v[m.1] == 1).count()
}

fn button(out: &mut Vec<Shape>, hits: &mut Vec<([f32; 4], Action)>, r: [f32; 4], label: &str, action: Action, mouse: [f32; 2], width: &dyn Fn(&str) -> f32) {
    out.push(Shape::Fill(r, if inside(r, mouse) { ROW_HOVER } else { ROW_BG }));
    out.push(Shape::Text(label.to_owned(), [r[0] + (r[2] - width(label)) / 2.0, r[1] + (r[3] - 8.0) / 2.0], WHITE, false));
    hits.push((r, action));
}

/// The menu in `screen` (`[x, y, w, h]`, GUI pixels) with the pointer at `mouse`: what to draw, and where a click does
/// what (the last one under the pointer wins). `browse` is the current Browse tab's list.
pub fn menu_layout(
    screen: [f32; 4],
    mouse: [f32; 2],
    m: &Menu,
    s: &Settings,
    browse: &Fetch,
    width: &dyn Fn(&str) -> f32,
) -> (Vec<Shape>, Vec<([f32; 4], Action)>) {
    let accent = s.accent();
    let (w, h) = menu_size(screen);
    let x0 = (screen[0] + (screen[2] - w) / 2.0).round();
    let y0 = (screen[1] + (screen[3] - h) / 2.0).round();
    let mut out = vec![
        Shape::Fill([0.0, 0.0, screen[2] + screen[0], screen[3] + screen[1]], 0x7000_0000),
        Shape::Fill([x0, y0, w, h], MENU_BG),
        Shape::Fill([x0, y0, w, 2.0], 0xFF00_0000 | accent),
    ];
    let mut hits = Vec::new();
    let title = [x0 + 6.0, y0 + 5.0, width("ARSENAL") + 8.0, 14.0];
    if m.page != Page::Home && inside(title, mouse) {
        out.push(Shape::Fill(title, ROW_BG));
    }
    out.push(Shape::Text("ARSENAL".into(), [x0 + 10.0, y0 + 9.0], accent, true));
    if m.page != Page::Home {
        hits.push((title, Action::Go(Page::Home)));
        let page = match m.page {
            Page::Modules => "Modules",
            Page::Browse => "Browse",
            Page::Cosmetics => "Cosmetics",
            Page::Home => "",
        };
        out.push(Shape::Text(format!("> {page}"), [title[0] + title[2] + 4.0, y0 + 9.0], MUTED, false));
    }
    let close = [x0 + w - 18.0, y0 + 5.0, 14.0, 14.0];
    button(&mut out, &mut hits, close, "x", Action::Close, mouse, width);
    let top = y0 + HEADER;

    match m.page {
        Page::Home => {
            out.push(Shape::Text("on Horizon".into(), [x0 + 14.0 + width("ARSENAL"), y0 + 9.0], GREY, false));
            let tiles: [(&str, String, Option<Page>); 3] = [
                ("Modules", format!("{} on · {} of {} run on Horizon", modules_on(s), ON_HORIZON.len(), JAVA_MODULES.len()), Some(Page::Modules)),
                ("Browse", "Crosshairs and themes from the website".into(), Some(Page::Browse)),
                ("Cosmetics", "Not on Horizon yet".into(), Some(Page::Cosmetics)),
            ];
            let tw = ((w - 32.0) / 3.0).floor();
            let ty = top + (h - HEADER - 80.0) / 2.0;
            for (i, (name, line, page)) in tiles.iter().enumerate() {
                let r = [x0 + 12.0 + i as f32 * (tw + 4.0), ty, tw, 54.0];
                let greyed = i == 2;
                out.push(Shape::Fill(r, if inside(r, mouse) { ROW_HOVER } else { ROW_BG }));
                out.push(Shape::Fill([r[0], r[1], r[2], 2.0], if greyed { 0xFF5E_5E6A } else { 0xFF00_0000 | accent }));
                out.push(Shape::Text((*name).into(), [r[0] + 8.0, r[1] + 12.0], if greyed { MUTED } else { WHITE }, true));
                out.push(Shape::Text(fit(line, r[2] - 16.0, width), [r[0] + 8.0, r[1] + 30.0], if greyed { GREY } else { MUTED }, false));
                if let Some(p) = page {
                    hits.push((r, Action::Go(*p)));
                }
            }
            let done = [x0 + (w - 80.0) / 2.0, y0 + h - 26.0, 80.0, 16.0];
            button(&mut out, &mut hits, done, "Done", Action::Close, mouse, width);
        }
        Page::Cosmetics => {
            let lines = [
                "Your Arsenal cosmetics still show to players on the Java version.",
                "Horizon can't draw capes, hats or other cosmetics on players yet,",
                "and doesn't pass your account to mods, so the Wardrobe isn't here yet.",
                "Change what you wear on the launcher's Cosmetics page.",
            ];
            for (i, l) in lines.iter().enumerate() {
                out.push(Shape::Text(fit(l, w - 28.0, width), [x0 + 14.0, top + 10.0 + i as f32 * 14.0], if i == 0 { WHITE } else { MUTED }, false));
            }
            let back = [x0 + 14.0, top + 74.0, 70.0, 16.0];
            button(&mut out, &mut hits, back, "< Home", Action::Go(Page::Home), mouse, width);
        }
        Page::Browse => {
            let mut tx = x0 + 10.0;
            for (i, (label, _)) in BROWSE_TABS.iter().enumerate() {
                let r = [tx, top, width(label) + 14.0, 15.0];
                let selected = m.tab == i;
                out.push(Shape::Fill(r, if selected { ROW_HOVER } else if inside(r, mouse) { ROW_BG } else { 0 }));
                if selected {
                    out.push(Shape::Fill([r[0], r[1] + r[3] - 1.0, r[2], 1.0], 0xFF00_0000 | accent));
                }
                out.push(Shape::Text((*label).into(), [r[0] + 7.0, r[1] + 4.0], if selected { WHITE } else { MUTED }, false));
                hits.push((r, Action::Tab(i)));
                tx += r[2] + 4.0;
            }
            if let Some(n) = &m.notice {
                out.push(Shape::Text(fit(n, x0 + w - tx - 12.0, width), [tx + 6.0, top + 4.0], accent, false));
            }
            let gy = top + 22.0;
            match browse {
                Fetch::Loading => out.push(Shape::Text("Loading from the Arsenal website...".into(), [x0 + 12.0, gy + 6.0], MUTED, false)),
                Fetch::Failed => {
                    out.push(Shape::Text("Couldn't reach the Arsenal website.".into(), [x0 + 12.0, gy + 6.0], MUTED, false));
                    button(&mut out, &mut hits, [x0 + 12.0, gy + 22.0, 70.0, 16.0], "Try again", Action::Retry, mouse, width);
                }
                Fetch::Ready(items) if items.is_empty() => out.push(Shape::Text("Nothing here yet.".into(), [x0 + 12.0, gy + 6.0], MUTED, false)),
                Fetch::Ready(items) => {
                    let (cols, rows) = grid(w, h);
                    let first_row = m.browse_scroll.min(items.len().div_ceil(cols).saturating_sub(rows));
                    for (n, item) in items.iter().enumerate().skip(first_row * cols).take(cols * rows) {
                        let k = n - first_row * cols;
                        let r = [x0 + 10.0 + (k % cols) as f32 * (CARD_W + 4.0), gy + (k / cols) as f32 * (CARD_H + 4.0), CARD_W, CARD_H];
                        let usable = !matches!(item.look, Look::Preset);
                        out.push(Shape::Fill(r, if usable && inside(r, mouse) { ROW_HOVER } else { ROW_BG }));
                        let preview = [r[0] + 4.0, r[1] + 4.0, r[2] - 8.0, 36.0];
                        out.push(Shape::Fill(preview, 0xFF0A_0A0E));
                        match &item.look {
                            Look::Crosshair(c) => {
                                let span = c.rows.len().max(c.rows.iter().map(|x| x.len()).max().unwrap_or(1)) as f32;
                                let cell = (32.0 / span).min(3.0);
                                out.extend(crosshair_shapes(c, [preview[0] + preview[2] / 2.0, preview[1] + preview[3] / 2.0], cell));
                            }
                            Look::Theme(t) => {
                                let p = [preview[0] + 8.0, preview[1] + 8.0, preview[2] - 16.0, 20.0];
                                out.push(Shape::Fill(p, t.background));
                                out.push(Shape::Fill([p[0], p[1], 2.0, p[3]], 0xFF00_0000 | t.accent));
                                out.push(Shape::Text("FPS".into(), [p[0] + 6.0, p[1] + 6.0], t.accent, true));
                                out.push(Shape::Text("144".into(), [p[0] + 8.0 + width("FPS"), p[1] + 6.0], t.text, true));
                            }
                            Look::Preset => out.push(Shape::Text("Java only".into(), [preview[0] + 6.0, preview[1] + 14.0], GREY, false)),
                        }
                        out.push(Shape::Text(fit(&item.title, r[2] - 8.0, width), [r[0] + 4.0, r[1] + 44.0], if usable { WHITE } else { GREY }, false));
                        out.push(Shape::Text(fit(&item.author, r[2] - 8.0, width), [r[0] + 4.0, r[1] + 56.0], GREY, false));
                        if usable {
                            hits.push((r, Action::Apply(n)));
                        }
                    }
                }
            }
        }
        Page::Modules => {
            out.push(Shape::Fill([x0, top, SIDE, h - HEADER], SIDEBAR_BG));
            for (i, name) in CATEGORIES.iter().enumerate() {
                let r = [x0, top + 4.0 + i as f32 * 16.0, SIDE, 16.0];
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
            match m.detail.and_then(|d| JAVA_MODULES.get(d)) {
                Some(module) => {
                    let back = [cx, top, width("< Back") + 10.0, 14.0];
                    button(&mut out, &mut hits, back, "< Back", Action::Back, mouse, width);
                    out.push(Shape::Text(module.name.into(), [back[0] + back[2] + 8.0, top + 3.0], accent, true));
                    out.push(Shape::Text(fit(module.description, cw, width), [cx, top + 20.0], MUTED, false));
                    let mut y = top + 34.0;
                    match on_horizon(module.id) {
                        Some((toggle, fields)) => {
                            if let Some(note) = horizon_note(module.id) {
                                out.push(Shape::Text(fit(note, cw, width), [cx, y], GREY, false));
                                y += 14.0;
                            }
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
                            if module.id == "custom_crosshair" || module.id == "hud_theme" {
                                let r = [cx, y + 4.0, width("Open Browse") + 14.0, 16.0];
                                button(&mut out, &mut hits, r, "Open Browse", Action::Go(Page::Browse), mouse, width);
                            }
                        }
                        None => {
                            out.push(Shape::Text("Only in the Java version of Arsenal for now:".into(), [cx, y], GREY, false));
                            out.push(Shape::Text(fit(why_not(module.id), cw, width), [cx, y + 12.0], GREY, false));
                        }
                    }
                }
                None => {
                    let list = category_modules(m.category);
                    let shown = rows_shown(screen);
                    let first = m.scroll.min(list.len().saturating_sub(shown));
                    for (row, &i) in list.iter().skip(first).take(shown).enumerate() {
                        let module = &JAVA_MODULES[i];
                        let r = [cx, top + 4.0 + row as f32 * ROW, cw, ROW - 3.0];
                        let here = on_horizon(module.id);
                        out.push(Shape::Fill(r, if inside(r, mouse) { ROW_HOVER } else { ROW_BG }));
                        let right = 44.0;
                        let (name_rgb, desc_rgb) = if here.is_some() { (WHITE, MUTED) } else { (GREY, GREY) };
                        out.push(Shape::Text(fit(module.name, cw - right - 12.0, width), [r[0] + 6.0, r[1] + 3.0], name_rgb, false));
                        out.push(Shape::Text(fit(module.description, cw - right - 12.0, width), [r[0] + 6.0, r[1] + 13.0], desc_rgb, false));
                        // Every row opens: the greyed-out ones say why they don't run here.
                        hits.push((r, Action::Open(i)));
                        match here {
                            Some((toggle, _)) => {
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
                        let track = [cx + cw + 3.0, top + 4.0, 2.0, shown as f32 * ROW - 3.0];
                        out.push(Shape::Fill(track, ROW_BG));
                        let part = shown as f32 / list.len() as f32;
                        let at = first as f32 / list.len() as f32;
                        out.push(Shape::Fill([track[0], track[1] + track[3] * at, 2.0, (track[3] * part).max(6.0)], 0xFF00_0000 | accent));
                    }
                }
            }
        }
    }
    (out, hits)
}

/// What a click at `mouse` does, from the same layout as the drawing.
pub fn menu_click(screen: [f32; 4], mouse: [f32; 2], m: &Menu, s: &Settings, browse: &Fetch, width: &dyn Fn(&str) -> f32) -> Option<Action> {
    let (_, hits) = menu_layout(screen, mouse, m, s, browse, width);
    hits.iter().rev().find(|(r, _)| inside(*r, mouse)).map(|(_, a)| *a)
}

/// Applies `action` to the menu and the settings; true when the settings changed (to be stored). `browse` is the
/// current Browse tab's list.
pub fn apply(action: Action, m: &mut Menu, s: &mut Settings, browse: &Fetch) -> bool {
    match action {
        Action::Close => {
            request_close();
            false
        }
        Action::Go(page) => {
            *m = Menu { page, tab: m.tab, ..Menu::default() };
            false
        }
        Action::Category(c) => {
            *m = Menu { page: Page::Modules, category: c, tab: m.tab, ..Menu::default() };
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
        Action::Tab(t) => {
            m.tab = t;
            m.browse_scroll = 0;
            m.notice = None;
            false
        }
        Action::Retry => {
            retry(m.tab);
            false
        }
        Action::Apply(i) => match browse {
            Fetch::Ready(items) => match items.get(i) {
                Some(item) if apply_item(item, s) => {
                    m.notice = Some(format!("Applied {}", item.title));
                    true
                }
                _ => false,
            },
            _ => false,
        },
    }
}

/// The wheel over the menu: `steps` up (positive) or down moves the list.
pub fn menu_scroll(m: &mut Menu, steps: f64, screen: [f32; 4], browse: &Fetch) {
    let step = -(steps.signum() as i64);
    match m.page {
        Page::Modules if m.detail.is_none() => {
            let max = category_modules(m.category).len().saturating_sub(rows_shown(screen));
            m.scroll = (m.scroll.min(max) as i64 + step).clamp(0, max as i64) as usize;
        }
        Page::Browse => {
            if let Fetch::Ready(items) = browse {
                let (w, h) = menu_size(screen);
                let (cols, rows) = grid(w, h);
                let max = items.len().div_ceil(cols).saturating_sub(rows);
                m.browse_scroll = (m.browse_scroll.min(max) as i64 + step).clamp(0, max as i64) as usize;
            }
        }
        _ => {}
    }
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

    fn cross() -> Crosshair {
        Crosshair { rows: vec!["..#..".into(), "..#..".into(), "#####".into(), "..#..".into(), "..#..".into()], rgb: 0xD277B0, size: 2 }
    }

    fn st(id: &str, count: u32, damage: u32) -> Option<Stack> {
        Some(Stack { id: id.into(), count, damage })
    }

    #[test]
    fn the_embedded_pack_is_the_shaderpack_folder() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("shaderpack").join("arsenal");
        for (name, text) in PACK {
            let file = std::fs::read_to_string(dir.join(name)).unwrap().replace("
", "
");
            assert_eq!(file, *text, "{name} changed: run scripts/embed_pack.py");
        }
        assert!(PACK.iter().any(|(n, _)| *n == "horizon-shaders.toml"));
    }

    #[test]
    fn mace_optimizer_plays_the_smash() {
        let mut s = Settings::default();
        s.v[MACE_OPT] = 1;
        let alex = Other { id: 7, uuid: 0, name: "Alex".into(), pos: [0.0; 3], health: 20.0, absorption: 0.0, hurt_time: 10, using: 0, using_off_hand: false, equipment: vec![] };
        STATE.with_borrow_mut(|st| st.pvp.fallen = 0.0);
        assert!(!mace_hit(&s, std::slice::from_ref(&alex), true));
        STATE.with_borrow_mut(|st| st.pvp.fallen = 3.0);
        assert!(!mace_hit(&s, std::slice::from_ref(&alex), false));
        assert!(mace_hit(&s, std::slice::from_ref(&alex), true));
        STATE.with_borrow_mut(|st| st.pvp.fallen = 0.0);
    }

    #[test]
    fn the_last_group() {
        let s0 = Settings::default();
        assert_eq!(shader_params(&s0)[0], [0.0, 0.0, 0.0, 1.0], "the pack passes the picture through");
        let mut s = Settings::default();
        s.v[BLOOM] = 1;
        s.v[MOTION] = 1;
        s.v[LOOK] = 1;
        s.v[LOOK_KIND] = 3;
        s.v[SKY_FX] = 1;
        let p = shader_params(&s);
        assert!(p[0][0] > 0.0 && p[0][1] > 0.0 && p[1][0] < 0.01 && p[4][0] > 0.0);
        assert_eq!(item_name("minecraft:enchanted_golden_apple"), "Enchanted Golden Apple");
        assert_eq!(bow_charge("minecraft:bow", 10), Some(0.5));
        assert_eq!(bow_charge("minecraft:crossbow", 50), Some(1.0));
        assert_eq!(bow_charge("minecraft:stick", 5), None);
        assert_eq!(mace_bonus(1.0), 0.0);
        assert_eq!(mace_bonus(10.0), 12.0 + 10.0 + 2.0);
        let (list, added) = toggle_friend(&["Alex".into()], "Steve");
        assert_eq!((list.as_str(), added), ("Alex,Steve", true));
        let (list, added) = toggle_friend(&friends_of(Some(list)), "alex");
        assert_eq!((list.as_str(), added), ("Steve", false));
        s.v[HOTKEYS] = 1;
        let bound = |id: &str| if id == "hotkey_gg" { "key.keyboard.f6".to_string() } else { "key.keyboard.unknown".to_string() };
        assert_eq!(hotkey(&s, "key.keyboard.f6", &bound, None), None);
        STATE.with_borrow(|st| assert_eq!(st.pvp.outbox.last().map(|o| o.1.as_str()), Some("gg")));
        s.v[PEARLS] = 1;
        let mut alex = Other { id: 7, uuid: 0, name: "Alex".into(), pos: [0.0; 3], health: 20.0, absorption: 0.0, hurt_time: 0, using: 32, using_off_hand: false,
            equipment: vec![st("minecraft:golden_apple", 3, 0), st("minecraft:ender_pearl", 8, 0)] };
        more_tick(&s, std::slice::from_ref(&alex), &[], 1, false);
        alex.using = 0;
        alex.equipment[1] = st("minecraft:ender_pearl", 7, 0);
        more_tick(&s, std::slice::from_ref(&alex), &[(1, 35)], 1, false);
        more_tick(&s, std::slice::from_ref(&alex), &[], 1, true);
        STATE.with_borrow(|st| {
            assert_eq!(st.extra.gapples.get("Alex"), Some(&1));
            assert!(st.extra.thrown.contains_key(&7) && st.extra.retotem.is_some());
        });
        STATE.with_borrow_mut(|st| { st.pvp.session_start = Some(Instant::now()); st.pvp.kills_total = 3; });
        session_ended();
        STATE.with_borrow(|st| assert!(st.extra.summary.as_ref().unwrap()[2].starts_with("3 kills")));
    }

    #[test]
    fn hashes_tiers_presence_and_effects() {
        assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        assert_eq!(sha256_hex(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(dashed(0x069a79f444e94726a5befca90e38aaf5), "069a79f4-44e9-4726-a5be-fca90e38aaf5");
        assert_eq!(user_hash(1).len(), 64);
        let json = r#"{"rankings": {"sword": {"tier": 3, "pos": 0}, "crystal": {"tier": 2, "pos": 1}, "mace": {"tier": 1, "pos": 0, "retired": true}}}"#;
        assert_eq!(best_tier(json), Some(("LT2".into(), 0xAEB8C6)));
        assert_eq!(best_tier("{}"), None);
        assert_eq!(users_in(r#"{"users": ["aa", {"hash": "bb", "vip": true}]}"#), vec!["aa".to_string(), "bb".to_string()]);
        let mut s = Settings::default();
        let p: serde_json::Value = serde_json::from_str(&presence_json(&s, true, 5, 9)).unwrap();
        assert_eq!(p["activity"]["details"], "Playing Horizon with Arsenal", "on by default");
        s.v[DISCORD] = 0;
        let off: serde_json::Value = serde_json::from_str(&presence_json(&s, true, 5, 9)).unwrap();
        assert!(off.get("activity").is_none() && off["pid"] == 9);
        s.v[CRITS] = 1;
        s.v[SWEEP] = 1;
        s.v[TOTEM_STYLE] = 1;
        assert_eq!(particle_rules(&s), (3, true));
        particles_tick(&s, &[("crit", [0.0, 0.0, 5.0]), ("sweep_attack", [0.0, 0.0, 5.0])], true);
        let mut proj = [0.0f32; 16];
        proj[0] = 1.0;
        proj[5] = 1.0;
        proj[11] = 1.0;
        let mut view = [0.0f32; 16];
        for i in 0..4 { view[i * 5] = 1.0; }
        let cam = Cam { pos: [0.0, 0.0, 0.0], view, projection: proj };
        let shapes = effect_shapes(&s, &[], Some(&cam), 400.0, 300.0, &width);
        assert!(shapes.iter().filter(|x| matches!(x, Shape::Quad(..))).count() >= 4, "a star and the sweep arc");
        assert!(shapes.iter().any(|x| matches!(x, Shape::Fill([0.0, 0.0, ..], _))), "the totem flash");
    }

    #[test]
    fn visuals_go_on_and_give_back_once() {
        let mut s = Settings::default();
        assert_eq!(visuals(&s, 0.5), Visuals { particles: 100, ..Default::default() }, "nothing touched");
        s.v[LOW_FIRE] = 1;
        s.v[SMALLER] = 1;
        s.v[FULLBRIGHT] = 1;
        s.v[WEATHER_TIME] = 1;
        s.v[PARTICLES] = 1;
        s.v[CHAT_FILTER] = 1;
        s.v[FILTER_KIND] = 1;
        let v = visuals(&s, 0.5);
        assert_eq!((v.fire_lower, v.shield_size, v.totem_size, v.gamma), (Some(40), Some(80), Some(80), Some(15.0)));
        assert_eq!((v.time.as_deref(), v.weather.as_deref(), v.particles), (Some("world"), Some("clear"), 50));
        assert!(v.chat_hide.contains(&"joined the game".to_string()) && !v.chat_hide.contains(&"discord.gg".to_string()));
        s.v[LOW_FIRE] = 0;
        s.v[FULLBRIGHT] = 0;
        let off = visuals(&s, 15.0);
        assert_eq!((off.fire_lower, off.gamma), (Some(0), Some(0.5)), "given back once, the old brightness");
        let later = visuals(&s, 0.5);
        assert_eq!((later.fire_lower, later.gamma), (None, None), "then left alone");
    }

    #[test]
    fn crystals_anchors_and_falls() {
        let bare = crystal_damage([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0, 0.0);
        let armored = crystal_damage([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 20.0, 12.0);
        assert!(bare > 70.0 && armored < bare && armored > 5.0, "{bare} {armored}");
        assert_eq!(crystal_damage([0.0; 3], [0.0, 0.0, 13.0], 0.0, 0.0), 0.0, "too far");
        let full = [st("minecraft:netherite_helmet", 1, 0), st("minecraft:netherite_chestplate", 1, 0), st("minecraft:netherite_leggings", 1, 0), st("minecraft:netherite_boots", 1, 0)];
        assert_eq!(armor_of(&full), (20.0, 12.0));
        assert_eq!(anchor_charges("minecraft:respawn_anchor[charges=3]"), Some(3));
        assert_eq!(anchor_charges("minecraft:stone"), None);
        assert_eq!(fall_damage(10.0), 7.0);
        let mut s = Settings::default();
        s.v[COOLDOWN] = 1;
        s.v[ANCHOR] = 1;
        let world = World { crystals: vec![], tnts: vec![], cooldowns: vec![("minecraft:ender_pearl".into(), 0.5)], dropped: vec![] };
        let shapes = world_shapes(&s, &world, None, (0.0, 0.0), None, Some("minecraft:respawn_anchor[charges=2]"), None, 400.0, 300.0, &width);
        assert!(shapes.iter().any(|x| matches!(x, Shape::Item(id, _) if id == "minecraft:ender_pearl")));
        assert!(shapes.iter().any(|x| matches!(x, Shape::Text(t, ..) if t == "Anchor 2/4")));
    }

    #[test]
    fn status_modules() {
        assert_eq!(effect_name("minecraft:fire_resistance", 0), "Fire Resistance");
        assert_eq!(effect_name("strength", 1), "Strength II");
        assert_eq!(ticks_text(1300), "1:05");
        assert_eq!(ticks_text(-1), "**:**");
        assert_eq!(ping_rgb(40), 0x5CE08A);
        let mut s = Settings::default();
        let status = Status { attack: 0.5, effects: vec![("minecraft:strength".into(), 1, 90), ("minecraft:speed".into(), 0, 2400)], ping: Some(45), latencies: vec![("Alex".into(), 120)] };
        assert!(status_shapes(&s, &status, &[], false, 0, false, None, 400.0, 300.0, &width, "12:00").is_empty(), "everything off");
        s.v[POTIONS] = 1;
        s.v[ATTACK_BAR] = 1;
        s.v[LAG] = 1;
        s.v[POTION_WARN] = 1;
        s.v[SESSION_STATS] = 1;
        s.v[INV_TOTEM] = 1;
        let shapes = status_shapes(&s, &status, &[], false, 3, false, None, 400.0, 300.0, &width, "12:00");
        let texts: Vec<String> = shapes.iter().filter_map(|x| if let Shape::Text(t, ..) = x { Some(t.clone()) } else { None }).collect();
        assert!(texts.contains(&"Strength II 0:04".to_string()) && texts.contains(&"Speed 2:00".to_string()));
        assert!(texts.contains(&"Ping 45 ms".to_string()) && texts.iter().any(|t| t.starts_with("K 0  D 0")) && texts.contains(&"3".to_string()));
        status_tick(&s, &status, Some(20.0));
        status_tick(&s, &status, Some(14.0));
        let (sounds, _) = take_effects();
        assert_eq!(sounds.len(), 1, "Strength runs out in 5s, once");
        STATE.with_borrow(|st| assert_eq!(st.pvp.taken.len(), 1));
    }

    #[test]
    fn fights_combos_and_recaps() {
        let mut s = Settings::default();
        s.v[FIGHT_SUMMARY] = 1;
        s.v[DEATH_RECAP] = 1;
        s.v[ENEMY_ARMOR] = 1;
        let alex = Other { id: 7, uuid: 0, name: "Alex".into(), pos: [0.0, 64.0, 3.0], health: 20.0, absorption: 0.0, hurt_time: 0, using: 0, using_off_hand: false,
            equipment: vec![None, None, None, None, None, st("minecraft:diamond_helmet", 1, 350)] };
        players_tick(&s, std::slice::from_ref(&alex), &[]);
        clicked();
        players_tick(&s, &[Other { health: 15.0, hurt_time: 10, ..alex.clone() }], &[]);
        players_tick(&s, &[Other { health: 15.0, hurt_time: 9, ..alex.clone() }], &[(7, 35)]);
        STATE.with_borrow(|st| {
            let f = st.pvp.fight.as_ref().unwrap();
            assert_eq!((f.hits, f.pops), (1, 1));
            assert_eq!(st.pvp.combo, 1);
            assert!(st.pvp.feed.iter().any(|l| l.0 == "Alex's helmet is about to break"));
        });
        chat_lines(&s, Some("Steve"), &["Steve was slain by Alex".into()]);
        STATE.with_borrow(|st| {
            assert_eq!(st.pvp.deaths_total, 1);
            assert_eq!(st.pvp.card.as_ref().unwrap().0[0], "Killed by Alex");
        });
    }

    #[test]
    fn death_messages_mentions_and_combat_tags() {
        assert_eq!(parse_death("Steve was slain by Alex using Netherite Sword"), Some(Death { victim: "Steve".into(), killer: Some("Alex".into()) }));
        assert_eq!(parse_death("[mcpvp] Steve was blown up by Alex"), Some(Death { victim: "Steve".into(), killer: Some("Alex".into()) }));
        assert_eq!(parse_death("Steve fell from a high place"), Some(Death { victim: "Steve".into(), killer: None }));
        assert_eq!(parse_death("Steve hit the ground too hard while trying to escape Alex"), Some(Death { victim: "Steve".into(), killer: None }));
        assert_eq!(parse_death("Steve drowned whilst trying to escape Alex"), Some(Death { victim: "Steve".into(), killer: None }));
        assert_eq!(parse_death("<Alex> hello there"), None);
        assert_eq!(parse_death("<Alex> I died lol"), None, "a chat line");
        assert!(mentions("<Alex> hey Steve, duel?", "Steve"));
        assert!(!mentions("<Steve> hey Alex", "Steve"), "your own message");
        assert!(!mentions("<Alex> Steveo is cool", "Steve"), "a longer name");
        assert_eq!(combat_line("You are now in combat for 15 seconds. Do not log out!"), Some(15));
        assert_eq!(combat_line("You are no longer in combat."), Some(0));
        assert_eq!(combat_line("Welcome!"), None);
        assert_eq!(streak_name(1), None);
        assert_eq!(streak_name(3), Some("Triple Kill"));
        assert_eq!(fill_name("gg {name}", Some("Alex")), "gg Alex");
        assert_eq!(fill_name("nice one {name}", None), "nice one");
    }

    #[test]
    fn kill_and_death_messages_go_out_once_each() {
        let mut s = Settings::default();
        s.v[KILL_MSG] = 1;
        s.v[KILL_TEXT] = 3;
        s.v[DEATH_MSG] = 1;
        s.v[STREAKS] = 1;
        chat_lines(&s, Some("Steve"), &["Alex was slain by Steve".into(), "Bob was shot by Steve".into()]);
        std::thread::sleep(Duration::from_millis(850));
        assert_eq!(take_outbox(), vec!["gg Alex".to_string()]);
        assert!(take_outbox().is_empty(), "the next waits 2 seconds");
        let (sounds, _) = take_effects();
        assert_eq!(sounds.len(), 1, "Double Kill's sound");
        chat_lines(&s, None, &["Steve was slain by Bob".into()]);
        assert_eq!(STATE.with_borrow(|st| st.pvp.outbox.last().map(|o| o.1.clone())), Some("GG".into()));
    }

    #[test]
    fn pops_damage_hitmarker_and_projection() {
        let mut s = Settings::default();
        s.v[POPS] = 1;
        s.v[POP_ALERTS] = 1;
        s.v[DAMAGE_NUMS] = 1;
        s.v[HITMARKER] = 1;
        let alex = Other { id: 7, uuid: 0, name: "Alex".into(), pos: [0.0, 64.0, 3.0], health: 20.0, absorption: 0.0, hurt_time: 0, using: 0, using_off_hand: false, equipment: vec![st("minecraft:diamond_sword", 1, 0)] };
        players_tick(&s, std::slice::from_ref(&alex), &[]);
        clicked();
        let hit = Other { health: 14.0, hurt_time: 10, ..alex.clone() };
        players_tick(&s, std::slice::from_ref(&hit), &[(7, 35), (7, 35)]);
        let (sounds, notes) = take_effects();
        assert_eq!(notes, vec!["Alex popped a totem (1)".to_string(), "Alex popped a totem (2)".to_string()]);
        assert_eq!(sounds.len(), 2);
        STATE.with_borrow(|st| {
            assert_eq!(st.pvp.pops.get("Alex"), Some(&2));
            assert_eq!(st.pvp.damage.len(), 1);
            assert!(st.pvp.hitmarker.is_some());
        });
        // A camera at the origin looking down +z (identity view, a simple projection): straight ahead is the middle.
        let mut proj = [0.0f32; 16];
        proj[0] = 1.0;
        proj[5] = 1.0;
        proj[11] = 1.0; // w = z
        let mut view = [0.0f32; 16];
        for i in 0..4 {
            view[i * 5] = 1.0;
        }
        let cam = Cam { pos: [0.0, 0.0, 0.0], view, projection: proj };
        assert_eq!(project(&cam, [0.0, 0.0, 5.0], 400.0, 300.0), Some([200.0, 150.0]));
        assert_eq!(project(&cam, [0.0, 0.0, -5.0], 400.0, 300.0), None, "behind");
        let shapes = pvp_shapes(&s, &[hit.clone()], None, Some(&cam), 400.0, 300.0, &width);
        assert!(shapes.iter().any(|x| matches!(x, Shape::Quad(..))), "the hitmarker");
        assert!(shapes.iter().any(|x| matches!(x, Shape::Text(t, ..) if t == "Alex: 2")), "the pop counter");
        assert!(target(&[hit], [0.0, 63.0, 0.0], 0.0, 0.0).is_some(), "looking at Alex 3 blocks ahead");
    }

    #[test]
    fn inventory_modules() {
        let mut s = Settings::default();
        let mut inv = Inventory { slots: vec![None; 36], selected: 0, ..Default::default() };
        inv.slots[0] = st("minecraft:diamond_sword", 1, 0);
        inv.slots[5] = st("minecraft:totem_of_undying", 1, 0);
        inv.slots[20] = st("minecraft:totem_of_undying", 1, 0);
        inv.slots[7] = st("minecraft:ender_pearl", 16, 0);
        inv.armor = [st("minecraft:diamond_helmet", 1, 330), st("minecraft:diamond_chestplate", 1, 0), st("minecraft:diamond_leggings", 1, 0), st("minecraft:diamond_boots", 1, 0)];
        assert_eq!(inv.count("minecraft:totem_of_undying"), 2);
        for f in [armor_shapes(&s, &inv, 400.0, 300.0, &width), counter_shapes(&s, &inv, 400.0, 300.0, &width), ready_shapes(&s, &inv, 300.0, &width)] {
            assert!(f.is_empty(), "everything is off by default");
        }
        assert_eq!(offhand_warning(&s, &inv), None);
        s.v[ARMOR] = 1;
        let armor = armor_shapes(&s, &inv, 400.0, 300.0, &width);
        assert_eq!(armor.iter().filter(|x| matches!(x, Shape::Item(..))).count(), 5, "4 armour pieces and the sword");
        assert!(armor.iter().any(|x| matches!(x, Shape::Text(t, ..) if t == "9%")), "the helmet: 33 of 363 left");
        s.v[COUNTER] = 1;
        let counted = counter_shapes(&s, &inv, 400.0, 300.0, &width);
        assert!(counted.iter().any(|x| matches!(x, Shape::Text(t, ..) if t == "2")) && counted.iter().any(|x| matches!(x, Shape::Text(t, ..) if t == "16")));
        s.v[OFFHAND_WARN] = 1;
        assert_eq!(offhand_warning(&s, &inv).as_deref(), Some("No totem in your offhand! (2 left)"));
        s.v[DURA_WARN] = 1;
        assert_eq!(worn_out(&s, &inv), vec!["helmet"]);
        assert!(durability_sound_due(&worn_out(&s, &inv)) && !durability_sound_due(&worn_out(&s, &inv)), "once per piece");
        let warnings = warning_shapes(&s, &inv, 400.0, 300.0, 0, &width);
        assert_eq!(warnings.len(), 2);
        assert!(!ready_checks(&inv)[0].1 && !ready_checks(&inv)[1].1, "no offhand totem, a worn helmet");
        inv.offhand = st("minecraft:totem_of_undying", 1, 0);
        inv.armor[0] = st("minecraft:diamond_helmet", 1, 0);
        inv.slots[9] = st("minecraft:golden_apple", 8, 0);
        assert!(ready_checks(&inv).iter().all(|c| c.1));
        s.v[READY] = 1;
        assert!(matches!(&ready_shapes(&s, &inv, 300.0, &width)[2], Shape::Text(t, ..) if t == "Ready!"));
        assert_eq!(offhand_warning(&s, &inv), None);
        for id in ["armor_hud", "item_counter", "offhand_totem_warning", "durability_warning", "ready_check"] {
            assert!(on_horizon(id).is_some(), "{id}");
        }
    }

    #[test]
    fn low_health_saturation_tooltips_and_shulkers() {
        let mut s = Settings::default();
        let low = Health { health: 5.0, max: 20.0, absorption: 0.0, food: 10, saturation: 3.25, width: 400.0, height: 300.0 };
        let fine = Health { health: 12.0, ..low };
        assert!(low_health_shapes(&s, &low, 0).is_empty(), "off by default");
        s.v[LOW_HEALTH] = 1;
        assert_eq!(low_health_shapes(&s, &low, 250).len(), 12, "three bands on four edges");
        assert!(low_health_shapes(&s, &fine, 250).is_empty(), "6 hearts isn't low at 4");
        assert!(heartbeat_due(&s, &low) && !heartbeat_due(&s, &low), "once, then waits");
        s.v[SATURATION] = 1;
        assert!(matches!(&saturation_shapes(&s, &low)[..], [Shape::Text(t, ..)] if t == "3.2" || t == "3.3"));
        assert_eq!(max_durability("minecraft:diamond_sword"), Some(1561));
        assert_eq!(max_durability("minecraft:netherite_chestplate"), Some(592));
        assert_eq!(max_durability("minecraft:leather_boots"), Some(65));
        assert_eq!(max_durability("minecraft:stick"), None);
        assert!(tooltip_lines(&s, "minecraft:diamond_sword", 0).is_empty(), "Better Tooltips off");
        s.v[TOOLTIPS] = 1;
        let lines = tooltip_lines(&s, "minecraft:diamond_sword", 1500);
        assert_eq!(lines[0].0, "Durability: 61 / 1561");
        assert_eq!(lines[0].1 >> 8 & 0xFF, ((61.0 / 1561.0 * 2.0) * 220.0_f32).round() as u32, "red-ish when low");
        let apple = tooltip_lines(&s, "minecraft:enchanted_golden_apple", 0);
        assert!(apple[0].0.starts_with("Restores 4 hunger, 9.6") && apple[1].0.contains("Absorption IV"));
        assert_eq!(shulker_size(&s, "minecraft:red_shulker_box", true), None, "Shulker Preview off");
        s.v[SHULKER] = 1;
        assert_eq!(shulker_size(&s, "minecraft:red_shulker_box", true), Some([166.0, 58.0]));
        assert_eq!(shulker_size(&s, "minecraft:red_shulker_box", false), None, "empty: no grid");
        assert_eq!(shulker_color("minecraft:red_shulker_box") & 0xFF_FFFF, 0xA12722);
        assert_eq!(shulker_slot([10.0, 20.0], 10), [10.0 + 2.0 + 18.0, 20.0 + 2.0 + 18.0]);
        assert!(tooltip_colors(&s).is_some(), "theme on by default");
        for id in ["low_health_warning", "better_tooltips", "shulker_preview", "food_preview"] {
            assert!(on_horizon(id).is_some(), "{id}");
        }
    }

    #[test]
    fn health_hud_number_bar_and_colours() {
        let h = Health { health: 15.0, max: 20.0, absorption: 4.0, food: 20, saturation: 5.0, width: 400.0, height: 300.0 };
        let off = Settings::default();
        assert!(health_shapes(&off, &h, &width).is_empty(), "off by default");
        let mut s = Settings::default();
        s.v[HEALTH] = 1;
        let shapes = health_shapes(&s, &h, &width);
        assert!(matches!(&shapes[0], Shape::Text(t, [_, y], _, true) if t == "7.5 +2" && *y == 160.0));
        assert_eq!(shapes.len(), 4, "number, bar background, bar, absorption");
        s.v[HEALTH_SHOW] = 1;
        s.v[HEALTH_UNIT] = 1;
        let shapes = health_shapes(&s, &h, &width);
        assert!(matches!(&shapes[..], [Shape::Text(t, ..)] if t == "15 +4"));
        s.v[HEALTH_SHOW] = 2;
        assert!(health_shapes(&s, &h, &width).iter().all(|x| matches!(x, Shape::Fill(..))));
        assert_eq!(health_rgb(1.0) >> 16, 0, "full is green");
        assert_eq!(health_rgb(0.0) >> 8 & 0xFF, 0, "empty is red");
        assert_eq!(health_text(13.0, 20.0, false), "6.5");
        assert_eq!(health_text(20.0, 20.0, false), "10");
        assert!(on_horizon("health_hud").is_some());
        assert!(on_horizon("kill_message").is_some() && on_horizon("death_message").is_some());
    }

    #[test]
    fn settings_round_trip_and_fall_back_to_defaults() {
        assert_eq!(Settings::read(stored(&[])), Settings::default());
        let mut s = Settings::read(stored(&[("corner", "bottom_right"), ("zoom", "6\n"), ("fog", "nope"), ("item", "55")]));
        assert_eq!((s.corner(), s.zoom_level(), s.v[FOG], s.v[ITEM]), (3, 6.0, 1, 0));
        s.click(CORNER);
        s.click(INFO);
        s.theme = Some(Theme { background: 0xB0000000, text: 0xFFFFFF, accent: 0xFF0000 });
        s.crosshair = Some(cross());
        let back = s.stored();
        let pairs: Vec<(&str, &str)> = back.iter().map(|(k, v)| (*k, v.as_str())).collect();
        assert_eq!(Settings::read(stored(&pairs)), s);
        assert_eq!(s.label(INFO), "Info HUD: OFF");
        assert_eq!(s.labels().len(), FIELDS.len());
    }

    #[test]
    fn modules_change_things_only_while_on() {
        let mut s = Settings::default();
        s.v[ITEM] = 2;
        s.v[DISTANCE] = 1;
        assert_eq!((s.item_scale(), s.camera_distance(4.0), s.fog_range(10.0, 20.0)), (1.0, 4.0, (10.0, 20.0)));
        s.click(VIEW);
        s.click(CAMERA);
        s.click(NO_FOG);
        assert!((s.item_scale() - 0.8).abs() < 1e-6);
        assert_eq!(s.camera_distance(4.0), 2.0);
        let (a, b) = s.fog_range(10.0, 20.0);
        assert!(b > a && a > 1000.0);
        s.v[FOG] = 0;
        assert_eq!(s.fog_range(10.0, 20.0), (20.0, 40.0));
        assert!(!s.hides_layer("minecraft:boss_overlay") && !s.hides_layer("minecraft:crosshair"));
        s.click(HIDE_BOSS);
        s.click(HIDE_TITLES);
        s.click(CROSSHAIR);
        assert!(s.hides_layer("minecraft:boss_overlay") && s.hides_layer("subtitle_overlay"));
        // No crosshair applied yet: the vanilla one stays.
        assert!(!s.hides_layer("minecraft:crosshair"));
        s.crosshair = Some(cross());
        assert!(s.hides_layer("minecraft:crosshair"));
        assert!(!s.hides_tag("minecraft:player"));
        s.click(HIDE_TAGS);
        assert!(s.hides_tag("minecraft:player") && !s.hides_tag("minecraft:zombie"));
        assert_eq!(s.outline(), None);
        s.click(OUTLINE);
        assert_eq!(s.outline(), Some((0xFF00_0000 | s.accent(), 2.0)));
    }

    #[test]
    fn themes_from_browse_colour_the_panel() {
        let mut s = Settings::default();
        let t = Theme { background: 0xB0102030, text: 0xEEEEEE, accent: 0x00FF88 };
        assert!(apply_item(&GalleryItem { title: "T".into(), author: "A".into(), look: Look::Theme(t) }, &mut s));
        assert_eq!((s.accent(), s.panel_colors()), (0x00FF88, (0xB0102030, 0xEEEEEE)));
        s.click(ACCENT);
        assert_eq!((s.accent(), s.panel_colors()), (ACCENT_RGB[0], (BACKGROUND, WHITE)));
        assert!(!apply_item(&GalleryItem { title: "P".into(), author: "A".into(), look: Look::Preset }, &mut s));
    }

    #[test]
    fn reads_the_websites_gallery() {
        let text = r##"{"items":[
            {"kind":"crosshair","title":"Tiny X pink","author":"Solo","preview":{"rows":["#.#",".#.","#.#"],"color":"#D277B0","size":2}},
            {"kind":"crosshair","title":"Broken","author":"x","preview":{"rows":["abc"],"color":"#FFFFFF"}},
            {"kind":"theme","title":"Dark","author":"Solo","preview":{"look":{"background":"#B0000000","text":"#FFFFFFFF","accent":"#FFFF0000"}}},
            {"kind":"preset","title":"Setup","author":"Solo","preview":{"changed":3}}]}"##;
        let items = parse_gallery(text);
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].look, Look::Crosshair(Crosshair { rows: vec!["#.#".into(), ".#.".into(), "#.#".into()], rgb: 0xD277B0, size: 2 }));
        assert_eq!(items[1].look, Look::Theme(Theme { background: 0xB0000000, text: 0xFFFFFF, accent: 0xFF0000 }));
        assert_eq!(items[2].look, Look::Preset);
        assert!(parse_gallery("not json").is_empty());
    }

    #[test]
    fn crosshairs_draw_one_rectangle_per_run() {
        let shapes = crosshair_shapes(&cross(), [100.0, 50.0], 1.0);
        assert_eq!(shapes.len(), 5);
        assert!(shapes.contains(&Shape::Fill([97.5, 49.5, 5.0, 1.0], 0xFFD277B0)));
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
        let r = Readings { fps: 144, cps: (7, 2), player: Some(([10.7, 64.0, -3.2], 180.0)), ..Readings::default() };
        let s = Settings::default();
        let l = lines(&s, &r);
        assert_eq!(l[2], ("XYZ", "10 64 -4".to_owned()));
        assert_eq!(l[3], ("Facing", "North -Z".to_owned()));
        let (shapes, w, h) = panel(&l, &s, &width).unwrap();
        assert_eq!(w, 2.0 + 4.0 + 36.0 + 5.0 + 48.0 + 4.0);
        assert_eq!(h, 8.0 + 40.0 - 2.0);
        assert_eq!(shapes[1], Shape::Fill([0.0, 0.0, 2.0, h], 0xFFF5B83D));
        assert_eq!(lines(&s, &Readings::default()).len(), 2);
        let mut none = s.clone();
        none.v[INFO] = 0;
        assert!(panel(&lines(&none, &Readings::default()), &none, &width).is_none());
        assert!(graph(&[], &s).is_none());
        let (g, gw, _) = graph(&[60, 120, 30], &s).unwrap();
        assert_eq!(gw, GRAPH_SAMPLES as f32 + 8.0);
        assert!(g.contains(&Shape::Text("30 FPS".into(), [4.0, 3.0], WHITE, true)));
    }

    #[test]
    fn the_wheel_zooms_only_while_the_key_is_held() {
        set_settings(&Settings::default());
        assert!(!wheel(1.0));
        zoom_key(false, true, |_| None);
        assert_eq!(view(), None);
        zoom_key(true, true, |_| None);
        assert_eq!(view(), Some((4.0, true, 0.25)));
        assert!(wheel(1.0));
        assert_eq!(view().unwrap().0, 5.0);
        // Past the old 10x: it keeps zooming in with every step.
        for _ in 0..20 {
            wheel(1.0);
        }
        let deep = view().unwrap().0;
        assert!(deep > 100.0 && deep < ZOOM_MAX, "{deep}");
        assert!((view().unwrap().2 - 1.0 / deep).abs() < 1e-9);
        for _ in 0..200 {
            wheel(1.0);
        }
        assert_eq!(view().unwrap().0, ZOOM_MAX);
        // And back out.
        wheel(-500.0);
        assert_eq!(view().unwrap().0, ZOOM_MIN);
        zoom_key(true, false, |_| None);
        assert_eq!(view(), None);
    }

    #[test]
    fn toggle_sneak_flips_on_each_press() {
        assert!(!sneak(true, false));
        assert!(sneak(true, true));
        assert!(sneak(true, false));
        assert!(!sneak(true, true));
        assert!(!sneak(true, false));
        assert!(sneak(false, true));
        assert!(!sneaking());
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
        let keys: std::collections::HashSet<&str> = FIELDS.iter().map(|f| f.key).collect();
        assert_eq!(keys.len(), FIELDS.len());
    }

    #[test]
    fn the_menu_clicks_what_it_draws() {
        let screen = [0.0, 0.0, 640.0, 360.0];
        let none = Fetch::Loading;
        let mut s = Settings::default();
        let mut m = Menu::default();
        // Home: the Modules tile opens the modules.
        let (shapes, hits) = menu_layout(screen, [0.0, 0.0], &m, &s, &none, &width);
        assert!(shapes.iter().any(|sh| matches!(sh, Shape::Text(t, _, _, _) if t == "ARSENAL")));
        let tile = hits.iter().find(|(_, a)| *a == Action::Go(Page::Modules)).unwrap().0;
        let a = menu_click(screen, [tile[0] + 5.0, tile[1] + 5.0], &m, &s, &none, &width).unwrap();
        apply(a, &mut m, &mut s, &none);
        assert_eq!(m.page, Page::Modules);
        // HUD lists Horizon's modules first: the first row's switch toggles it.
        let list = category_modules(0);
        let first = list[0];
        let (_, hits) = menu_layout(screen, [0.0, 0.0], &m, &s, &none, &width);
        let toggle = hits.iter().find(|(_, a)| *a == Action::Toggle(first)).unwrap().0;
        let (field, _) = on_horizon(JAVA_MODULES[first].id).unwrap();
        let before = s.v[field];
        let action = menu_click(screen, [toggle[0] + 2.0, toggle[1] + 2.0], &m, &s, &none, &width).unwrap();
        assert!(apply(action, &mut m, &mut s, &none));
        assert_ne!(s.v[field], before);
        // A greyed-out module switches nothing but says why.
        let java_only = *list.iter().find(|&&i| on_horizon(JAVA_MODULES[i].id).is_none()).unwrap();
        assert!(!hits.iter().any(|(_, a)| *a == Action::Toggle(java_only)));
        apply(Action::Open(java_only), &mut m, &mut s, &none);
        let (shapes, _) = menu_layout(screen, [0.0, 0.0], &m, &s, &none, &width);
        assert!(shapes.iter().any(|sh| matches!(sh, Shape::Text(t, _, _, _) if t.starts_with("Only in the Java version"))));
        apply(Action::Back, &mut m, &mut s, &none);
        // Settings view.
        let zoom = JAVA_MODULES.iter().position(|j| j.id == "zoom").unwrap();
        apply(Action::Open(zoom), &mut m, &mut s, &none);
        let (_, hits) = menu_layout(screen, [0.0, 0.0], &m, &s, &none, &width);
        assert!(hits.iter().any(|(_, a)| *a == Action::Setting(ZOOM)));
        assert!(apply(Action::Setting(ZOOM), &mut m, &mut s, &none));
        assert_eq!(s.zoom_level(), 6.0);
        // Browse: a card applies its crosshair.
        let ready = Fetch::Ready(vec![GalleryItem { title: "Plus".into(), author: "A".into(), look: Look::Crosshair(cross()) }]);
        apply(Action::Go(Page::Browse), &mut m, &mut s, &ready);
        let (_, hits) = menu_layout(screen, [0.0, 0.0], &m, &s, &ready, &width);
        let card = hits.iter().find(|(_, a)| *a == Action::Apply(0)).unwrap().0;
        let a = menu_click(screen, [card[0] + 5.0, card[1] + 5.0], &m, &s, &ready, &width).unwrap();
        assert!(apply(a, &mut m, &mut s, &ready));
        assert_eq!((s.crosshair.clone(), s.on(CROSSHAIR)), (Some(cross()), true));
        assert_eq!(m.notice.as_deref(), Some("Applied Plus"));
        apply(Action::Close, &mut m, &mut s, &ready);
        assert!(take_close());
        assert!(!take_close());
    }

    #[test]
    fn speed_and_session_follow_the_player() {
        let t0 = Instant::now();
        let ms = |n: u64| t0 + Duration::from_millis(n);
        assert_eq!(track(None, 60, t0), (0.0, 0));
        assert_eq!(track(Some([0.0, 64.0, 0.0]), 60, ms(0)), (0.0, 0));
        assert_eq!(track(Some([1.0, 64.0, 0.0]), 60, ms(100)).0, 0.0);
        let (speed, _) = track(Some([3.0, 70.0, 4.0]), 60, ms(500));
        assert!((speed - 10.0).abs() < 1e-9, "{speed}");
        assert_eq!(track(Some([3.0, 70.0, 4.0]), 60, ms(65_000)).1, 65);
        assert_eq!(track(None, 60, ms(66_000)), (0.0, 0));
        assert!(!fps_history().is_empty());
        assert_eq!((clock(245), clock(3729)), ("4:05".to_owned(), "1:02:09".to_owned()));
    }

    #[test]
    fn the_wheel_scrolls_within_the_list() {
        let screen = [0.0, 0.0, 400.0, 200.0];
        let mut m = Menu { page: Page::Modules, ..Menu::default() };
        let len = category_modules(0).len();
        let max = len.saturating_sub(rows_shown(screen));
        menu_scroll(&mut m, 1.0, screen, &Fetch::Loading);
        assert_eq!(m.scroll, 0);
        for _ in 0..200 {
            menu_scroll(&mut m, -1.0, screen, &Fetch::Loading);
        }
        assert_eq!(m.scroll, max);
        assert_eq!(fit("A long description", 60.0, &width), "A long...");
    }
}
