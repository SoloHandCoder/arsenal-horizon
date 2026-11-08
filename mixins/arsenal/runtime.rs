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
pub const VERSION: &str = "1.16.0";
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
pub const SWING_SPEEDS: &[(&str, &str)] = &[("150", "Slower"), ("70", "Faster"), ("50", "Much faster"), ("200", "Much slower")];
pub const CRYSTAL_SIZES: &[(&str, &str)] = &[("50", "Half"), ("75", "Three quarters"), ("35", "Tiny")];
pub const BIG_WHAT: &[(&str, &str)] = &[("loot", "Valuable loot"), ("all", "All items")];
pub const TAG_SIZES: &[(&str, &str)] = &[("75", "75%"), ("50", "50%"), ("100", "100%"), ("125", "125%")];
pub const TAG_BGS: &[(&str, &str)] = &[("dark", "Dark"), ("none", "None"), ("darker", "Darker")];
pub const BREAK_KINDS: &[(&str, &str)] = &[("shatter", "Shatter"), ("shrink", "Shrink away"), ("flash", "Flash")];
pub const PLACE_KINDS: &[(&str, &str)] = &[("pop", "Pop in"), ("flash", "Flash"), ("ripple", "Ripple")];
pub const HIDE_AS: &[(&str, &str)] = &[("You", "You"), ("Player", "Player"), ("Arsenal user", "Arsenal user"), ("???", "???")];
pub const VOLUME_STEPS: &[(&str, &str)] = &[("50", "50%"), ("25", "25%"), ("0", "Muted"), ("75", "75%"), ("100", "100%"), ("150", "150%")];
pub const MESSAGE_WHERE: &[(&str, &str)] = &[("chat", "Chat"), ("toast", "Small toasts")];
pub const CURSOR_SPEEDS: &[(&str, &str)] = &[("50", "50%"), ("75", "75%"), ("125", "125%"), ("150", "150%"), ("200", "200%")];
pub const BLADE_SIZES: &[(&str, &str)] = &[("100", "100%"), ("75", "75%"), ("125", "125%"), ("50", "50%"), ("150", "150%")];
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
pub const HOTBAR_STYLE: usize = 150;
pub const SMOOTH_HOTBAR: usize = 151;
pub const INV_STYLE: usize = 152;
pub const SMOOTH_INV: usize = 153;
pub const OLD_ANIM: usize = 154;
pub const SWING_ANIM: usize = 155;
pub const SWING_SPEED: usize = 156;
pub const HAND_TINT: usize = 157;
pub const HAND_GLOW: usize = 158;
pub const SMALL_CRYSTALS: usize = 159;
pub const CRYSTAL_SIZE: usize = 160;
pub const CRYSTAL_STILL: usize = 161;
pub const BIG_ITEMS: usize = 162;
pub const BIG_WHICH: usize = 163;
pub const ARMOR_COLOR: usize = 164;
pub const ARMOR_TINT: usize = 165;
pub const TAG_STYLE: usize = 166;
pub const TAG_SIZE: usize = 167;
pub const TAG_BG: usize = 168;
pub const FREE_LOOK: usize = 169;
pub const BREAK_ANIM: usize = 170;
pub const BREAK_KIND: usize = 171;
pub const PLACE_ANIM: usize = 172;
pub const PLACE_KIND: usize = 173;
pub const SLICK: usize = 174;
pub const TAB_PING: usize = 175;
pub const MENTIONS: usize = 176;
pub const CHAT_TWEAKS: usize = 177;
pub const CHAT_COPY: usize = 178;
pub const NAME_HIDER: usize = 179;
pub const HIDE_NAME_AS: usize = 180;
pub const SOUND_TWEAKS: usize = 181;
pub const VOL_EXPLODE: usize = 182;
pub const VOL_TOTEM: usize = 183;
pub const VOL_HIT: usize = 184;
pub const VOL_MACE: usize = 185;
pub const SPRINT_IND: usize = 186;
pub const OBSIDIAN_GUARD: usize = 187;
pub const CRYSTAL_HELPER: usize = 188;
pub const SHOT_CLIP: usize = 189;
pub const QUICK_MENU: usize = 190;
pub const PERF: usize = 191;
pub const MESSAGES: usize = 192;
pub const MESSAGES_WHERE: usize = 193;
pub const PARTY: usize = 194;
pub const WARDROBE_FX: usize = 195;
pub const USAGE: usize = 196;
pub const RECONNECT: usize = 197;
pub const CRASHES: usize = 198;
pub const UPDATES: usize = 199;
pub const NOTES: usize = 200;
pub const PROFILES: usize = 201;
pub const LYRICS: usize = 202;
pub const SKYBLOCK: usize = 203;
pub const SB_PRICES: usize = 204;
pub const SB_SPAM: usize = 205;
pub const ALBUM: usize = 206;
pub const ALBUM_SKY: usize = 207;
pub const CLOUD: usize = 208;
pub const CURSOR: usize = 209;
pub const CURSOR_SPEED: usize = 210;
pub const COSMETICS: usize = 211;
pub const BLADES: usize = 212;
pub const BLADE_SIZE: usize = 213;
const N: usize = 214;

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
    Field { key: "hotbar_style", label: "Hotbar Style", options: ONOFF, default: 0 },
    Field { key: "smooth_hotbar", label: "Smooth Hotbar", options: ONOFF, default: 0 },
    Field { key: "inventory_style", label: "Inventory Style", options: ONOFF, default: 0 },
    Field { key: "smooth_inventory", label: "Smooth Inventory", options: ONOFF, default: 0 },
    Field { key: "old_animations", label: "1.8 Animations", options: ONOFF, default: 0 },
    Field { key: "swing_animation", label: "Swing Animation", options: ONOFF, default: 0 },
    Field { key: "swing_speed", label: "Speed", options: SWING_SPEEDS, default: 0 },
    Field { key: "hand_tint", label: "Hand Tint & Glow", options: ONOFF, default: 0 },
    Field { key: "hand_glow", label: "Glow", options: ONOFF, default: 1 },
    Field { key: "small_crystals", label: "Smaller Crystals", options: ONOFF, default: 0 },
    Field { key: "crystal_size", label: "Size", options: CRYSTAL_SIZES, default: 0 },
    Field { key: "crystal_still", label: "No spinning", options: ONOFF, default: 0 },
    Field { key: "big_items", label: "Big Items", options: ONOFF, default: 0 },
    Field { key: "big_items_which", label: "Items", options: BIG_WHAT, default: 0 },
    Field { key: "armor_color", label: "Armor Color", options: ONOFF, default: 0 },
    Field { key: "armor_tint", label: "Colour", options: TINTS, default: 6 },
    Field { key: "nametag_style", label: "Nametag Style", options: ONOFF, default: 0 },
    Field { key: "nametag_size", label: "Size", options: TAG_SIZES, default: 0 },
    Field { key: "nametag_background", label: "Background", options: TAG_BGS, default: 0 },
    Field { key: "free_look", label: "Free Look", options: ONOFF, default: 0 },
    Field { key: "block_break_animation", label: "Block Break Animation", options: ONOFF, default: 0 },
    Field { key: "break_kind", label: "Style", options: BREAK_KINDS, default: 0 },
    Field { key: "place_animation", label: "Place Animation", options: ONOFF, default: 0 },
    Field { key: "place_kind", label: "Style", options: PLACE_KINDS, default: 0 },
    Field { key: "slick_floors", label: "Slick Floors", options: ONOFF, default: 0 },
    Field { key: "tab_ping", label: "Tab List Ping", options: ONOFF, default: 0 },
    Field { key: "chat_mentions", label: "Chat Mentions", options: ONOFF, default: 0 },
    Field { key: "chat_tweaks", label: "Chat Tweaks", options: ONOFF, default: 0 },
    Field { key: "chat_copy", label: "Click to Copy Chat", options: ONOFF, default: 0 },
    Field { key: "name_hider", label: "Name Hider", options: ONOFF, default: 0 },
    Field { key: "name_hider_text", label: "Show as", options: HIDE_AS, default: 0 },
    Field { key: "sound_tweaks", label: "Sound Tweaks", options: ONOFF, default: 0 },
    Field { key: "volume_explosions", label: "Explosions", options: VOLUME_STEPS, default: 0 },
    Field { key: "volume_totems", label: "Totem pops", options: VOLUME_STEPS, default: 4 },
    Field { key: "volume_hits", label: "Player hits", options: VOLUME_STEPS, default: 4 },
    Field { key: "volume_mace", label: "Mace smashes", options: VOLUME_STEPS, default: 4 },
    Field { key: "sprint_indicator", label: "Sprint Reset Indicator", options: ONOFF, default: 0 },
    Field { key: "obsidian_guard", label: "Obsidian Click Guard", options: ONOFF, default: 0 },
    Field { key: "crystal_helper", label: "Crystal Helper", options: ONOFF, default: 0 },
    Field { key: "screenshot_clipboard", label: "Screenshot to Clipboard", options: ONOFF, default: 0 },
    Field { key: "quick_menu", label: "Quick Menu", options: ONOFF, default: 0 },
    Field { key: "performance_mode", label: "Performance Mode", options: ONOFF, default: 0 },
    Field { key: "arsenal_messages", label: "Arsenal Messages", options: ONOFF, default: 0 },
    Field { key: "arsenal_messages_where", label: "Where", options: MESSAGE_WHERE, default: 1 },
    Field { key: "party_hud", label: "Party", options: ONOFF, default: 0 },
    Field { key: "wardrobe_kill_effects", label: "Wardrobe Kill Effects", options: ONOFF, default: 0 },
    Field { key: "usage_stats", label: "Anonymous Usage Stats", options: ONOFF, default: 1 },
    Field { key: "auto_reconnect", label: "Auto Reconnect", options: ONOFF, default: 0 },
    Field { key: "crash_reports", label: "Crash Reports", options: ONOFF, default: 1 },
    Field { key: "update_notice", label: "Update Notice", options: ONOFF, default: 1 },
    Field { key: "server_notes", label: "Server Notes", options: ONOFF, default: 0 },
    Field { key: "server_profiles", label: "Server Profiles", options: ONOFF, default: 0 },
    Field { key: "spotify_lyrics", label: "Spotify Lyrics", options: ONOFF, default: 0 },
    Field { key: "hypixel_skyblock", label: "Hypixel SkyBlock", options: ONOFF, default: 0 },
    Field { key: "skyblock_prices", label: "Prices in tooltips", options: ONOFF, default: 0 },
    Field { key: "skyblock_spam_filter", label: "Hide spammy lines", options: ONOFF, default: 1 },
    Field { key: "album_colors", label: "Spotify Album Colors", options: ONOFF, default: 0 },
    Field { key: "album_sky", label: "Sky too", options: ONOFF, default: 1 },
    Field { key: "cloud_backup", label: "Cloud Backup", options: ONOFF, default: 0 },
    Field { key: "inventory_cursor_speed", label: "Inventory Cursor Speed", options: ONOFF, default: 0 },
    Field { key: "cursor_speed", label: "Speed", options: CURSOR_SPEEDS, default: 0 },
    Field { key: "arsenal_cosmetics", label: "Arsenal Cosmetics", options: ONOFF, default: 1 },
    Field { key: "blade_settings", label: "Blade Settings", options: ONOFF, default: 1 },
    Field { key: "blade_size", label: "Size", options: BLADE_SIZES, default: 0 },
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
    ("hotbar_style", HOTBAR_STYLE, &[]),
    ("smooth_hotbar", SMOOTH_HOTBAR, &[]),
    ("inventory_style", INV_STYLE, &[]),
    ("smooth_inventory", SMOOTH_INV, &[]),
    ("old_animations", OLD_ANIM, &[]),
    ("swing_animation", SWING_ANIM, &[SWING_SPEED]),
    ("hand_tint", HAND_TINT, &[HAND_GLOW]),
    ("small_crystals", SMALL_CRYSTALS, &[CRYSTAL_SIZE, CRYSTAL_STILL]),
    ("big_items", BIG_ITEMS, &[BIG_WHICH]),
    ("armor_color", ARMOR_COLOR, &[ARMOR_TINT]),
    ("nametag_style", TAG_STYLE, &[TAG_SIZE, TAG_BG]),
    ("free_look", FREE_LOOK, &[]),
    ("block_break_animation", BREAK_ANIM, &[BREAK_KIND]),
    ("place_animation", PLACE_ANIM, &[PLACE_KIND]),
    ("slick_floors", SLICK, &[]),
    ("tab_ping", TAB_PING, &[]),
    ("chat_mentions", MENTIONS, &[]),
    ("chat_tweaks", CHAT_TWEAKS, &[]),
    ("chat_copy", CHAT_COPY, &[]),
    ("name_hider", NAME_HIDER, &[HIDE_NAME_AS]),
    ("sound_tweaks", SOUND_TWEAKS, &[VOL_EXPLODE, VOL_TOTEM, VOL_HIT, VOL_MACE]),
    ("sprint_indicator", SPRINT_IND, &[]),
    ("obsidian_guard", OBSIDIAN_GUARD, &[]),
    ("crystal_helper", CRYSTAL_HELPER, &[]),
    ("screenshot_clipboard", SHOT_CLIP, &[]),
    ("quick_menu", QUICK_MENU, &[]),
    ("performance_mode", PERF, &[]),
    ("arsenal_messages", MESSAGES, &[MESSAGES_WHERE]),
    ("party_hud", PARTY, &[]),
    ("wardrobe_kill_effects", WARDROBE_FX, &[]),
    ("usage_stats", USAGE, &[]),
    ("auto_reconnect", RECONNECT, &[]),
    ("crash_reports", CRASHES, &[]),
    ("update_notice", UPDATES, &[]),
    ("server_notes", NOTES, &[]),
    ("server_profiles", PROFILES, &[]),
    ("spotify_lyrics", LYRICS, &[]),
    ("hypixel_skyblock", SKYBLOCK, &[SB_PRICES, SB_SPAM]),
    ("album_colors", ALBUM, &[ALBUM_SKY]),
    ("cloud_backup", CLOUD, &[]),
    ("inventory_cursor_speed", CURSOR, &[CURSOR_SPEED]),
    ("blade_settings", BLADES, &[BLADE_SIZE]),
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
        "hotbar_style" => "On Horizon: the hotbar on your HUD Theme's background with an accent line under the selected slot.",
        "smooth_hotbar" => "On Horizon: the selection box glides to the slot you pick.",
        "inventory_style" => "On Horizon: a frame in your HUD accent round your inventory (Horizon draws the panel itself).",
        "smooth_inventory" => "On Horizon: the item on your cursor glides after the mouse.",
        "old_animations" => "On Horizon: the old sword block-hit pose while you hold right-click, and held items lower.",
        "swing_animation" => "On Horizon: your hand swings slower or faster (only on your screen).",
        "hand_tint" => "On Horizon: your hands and held items in your HUD accent, glowing full bright if you like.",
        "small_crystals" => "On Horizon: end crystals drawn smaller, still if you like. Hitboxes stay the same.",
        "big_items" => "On Horizon: valuable loot (or every item) lying on the ground is drawn twice as big.",
        "armor_color" => "On Horizon: worn armour tinted in the colour you pick (everyone's: Horizon draws all armour the same way).",
        "nametag_style" => "On Horizon: smaller nametags and their background your way.",
        "free_look" => "On Horizon: hold the Free Look key (Left Alt) to look round your character in third person; you keep moving the same way.",
        "block_break_animation" => "On Horizon: blocks you break shatter, shrink away or flash in your HUD accent.",
        "place_animation" => "On Horizon: blocks you place pop in, flash or send out a ripple.",
        "slick_floors" => "On Horizon: floors near you look wet and glossy, through Arsenal's shader pack (pick it in Video Settings > Shader Packs).",
        "tab_ping" => "On Horizon: pings as coloured numbers in the tab list.",
        "chat_mentions" => "On Horizon: chat lines with your name are gold.",
        "chat_tweaks" => "On Horizon: timestamps, repeated lines stacked as (x3) and 500 lines of chat history.",
        "chat_copy" => "On Horizon: click a chat line to copy it.",
        "name_hider" => "On Horizon: your name shows as the text you pick everywhere Horizon draws text (chat, tab list, nametags, scoreboard, titles, HUD).",
        "sound_tweaks" => "On Horizon: your own volume for explosions, totem pops, player hits and mace smashes.",
        "sprint_indicator" => "On Horizon: a green bolt by the crosshair while your next hit gives sprint knockback.",
        "obsidian_guard" => "On Horizon: left-clicking the top of obsidian or bedrock does nothing unless you hold a pickaxe.",
        "crystal_helper" => "On Horizon: end crystals are only placed where they can spawn (on obsidian or bedrock with room above).",
        "screenshot_clipboard" => "On Horizon: F2 screenshots are copied to the clipboard as an image too.",
        "quick_menu" => "On Horizon: hold V, scroll to a feature and let go to switch it.",
        "performance_mode" => "On Horizon: the shader pack's effects, block animations and lyrics stop and particles are capped; give it a key in Options > Mods > Arsenal. Your settings come back as they were.",
        "arsenal_messages" => "On Horizon: Arsenal's own messages in chat or as small toasts at the top right.",
        "party_hud" => "On Horizon: your friends nearby with their health, distance and an arrow to them.",
        "wardrobe_kill_effects" => "On Horizon: the kill effect you equipped in the Wardrobe plays for your kills (as Kill Effects' burst, ring or lightning).",
        "usage_stats" => "On Horizon: counts you once a day like the Java mod (a one-way hash, no name or IP), signed with your Minecraft account.",
        "auto_reconnect" => "On Horizon: reconnects after a disconnect, with a Cancel button (Horizon's own).",
        "crash_reports" => "On Horizon: if Arsenal crashes the game, the next start offers /arsenal crash to send the error (no names or paths).",
        "update_notice" => "On Horizon: tells you when a newer Arsenal Horizon is out (the launcher installs it when you press Play).",
        "server_notes" => "On Horizon: /arsenal note <text> keeps a note for the server you're on; it shows when you join.",
        "server_profiles" => "On Horizon: /arsenal profile save keeps your setup for this server and loads it whenever you join; /arsenal profile clear forgets it.",
        "spotify_lyrics" => "On Horizon: the lyrics of the song playing in Spotify on the HUD, in time with Spotify (Windows, macOS, Linux with playerctl).",
        "hypixel_skyblock" => "On Horizon: your area, purse, bits and the SkyBlock date, Booster Cookie and God Potion timers, dungeon and slayer cards, Bazaar and NPC prices in tooltips and a spam filter. Only reads what the game shows.",
        "album_colors" => "On Horizon (Windows): the HUD and menu accent, and the sky with Arsenal's shader pack, take the colours of the album playing in Spotify.",
        "cloud_backup" => "On Horizon: /arsenal cloud save keeps your Horizon setup in your Arsenal cloud backup (next to the Java one, which stays as it is) and every 5 minutes after a change; /arsenal cloud restore brings it back on any PC (or your Java setup's modules when there's no Horizon one).",
        "blade_settings" => "On Horizon: your Wardrobe blade in your hand, its size, and the inspect key (Y) that spins it. Other Arsenal users' blades, capes, hats, accessories and pets show too (Arsenal Cosmetics).",
        "inventory_cursor_speed" => "On Horizon: a separate mouse speed in inventories. Your real cursor is never moved: Arsenal draws its own. F8 turns it off at once.",
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
    match id {
        _ => {}
    }
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
    /// Where the HUD editor moved elements: (element, [dx, dy, scale]).
    pub layout: Vec<(String, [f32; 5])>,
    /// The modules starred in the menu (Java ids), in the order starred.
    pub favorites: Vec<String>,
    /// A key per module (Java id, "key.keyboard.h") that switches it.
    pub keys: Vec<(String, String)>,
    /// Saved setups by name (each the stored settings), and which loads by itself on a server.
    pub profiles: Vec<(String, Vec<(String, String)>)>,
    pub auto_profiles: Vec<(String, String)>,
    /// The HUD editor: a theme for one element (element, theme name).
    pub hud_themes: Vec<(String, String)>,
    /// The layout for everywhere, and the ones for one server (`layout` is the one in use).
    pub main_layout: Vec<(String, [f32; 5])>,
    pub server_layouts: Vec<(String, Vec<(String, [f32; 5])>)>,
    /// Hide risky: the menu leaves out what some servers may not allow.
    pub hide_risky: bool,
}

impl Default for Settings {
    fn default() -> Self {
        let mut v = [0; N];
        for (i, f) in FIELDS.iter().enumerate() {
            v[i] = f.default;
        }
        Self { v, theme: None, crosshair: None, layout: Vec::new(), favorites: Vec::new(), keys: Vec::new(), profiles: Vec::new(), auto_profiles: Vec::new(),
               hud_themes: Vec::new(), main_layout: Vec::new(), server_layouts: Vec::new(), hide_risky: false }
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
        s.main_layout = get("hud_layout").map(|t| parse_layout(&t)).unwrap_or_default();
        s.server_layouts = json_pairs(get("server_huds").as_deref()).into_iter().map(|(k, v)| (k, parse_layout(&v))).collect();
        s.layout = current_server().and_then(|k| s.server_layouts.iter().find(|e| e.0 == k).map(|e| e.1.clone())).unwrap_or_else(|| s.main_layout.clone());
        s.keys = pairs_of(get("module_keys").as_deref()).into_iter().filter(|(id, k)| JAVA_MODULES.iter().any(|j| j.id == id) && k.starts_with("key.")).collect();
        s.profiles = get("profiles").and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok()).and_then(|v| v.as_object().map(|o| {
            o.iter().take(20).map(|(name, p)| (name.clone(), p.as_object().map(|m| m.iter().filter_map(|(k, v)| v.as_str().map(|v| (k.clone(), v.to_owned()))).collect())
                .unwrap_or_default())).collect()
        })).unwrap_or_default();
        s.auto_profiles = json_pairs(get("auto_profiles").as_deref());
        s.hud_themes = pairs_of(get("hud_themes").as_deref()).into_iter().filter(|(e, t)| HUD_ELEMENTS.iter().any(|x| x.0 == e) && HUD_THEMES.iter().any(|x| x.0 == t)).collect();
        s.hide_risky = get("hide_risky").is_some_and(|v| v == "true");
        s.favorites = get("favorites").map(|t| t.split(',').map(str::trim).filter(|f| JAVA_MODULES.iter().any(|j| j.id == *f)).map(str::to_owned).collect())
            .unwrap_or_default();
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
        // The layout in use is this server's when it has one, else the one for everywhere.
        let mut main = self.main_layout.clone();
        let mut servers = self.server_layouts.clone();
        match current_server().and_then(|k| servers.iter_mut().find(|e| e.0 == k)) {
            Some(entry) => entry.1 = self.layout.clone(),
            None => main = self.layout.clone(),
        }
        out.push(("hud_layout", layout_text(&main)));
        out.push(("server_huds", json_text(&servers.iter().map(|(k, l)| (k.clone(), layout_text(l))).collect::<Vec<_>>())));
        out.push(("favorites", self.favorites.join(",")));
        out.push(("module_keys", self.keys.iter().map(|(a, b)| format!("{a}={b}")).collect::<Vec<_>>().join(";")));
        let profiles: serde_json::Map<String, serde_json::Value> = self.profiles.iter()
            .map(|(n, p)| (n.clone(), serde_json::Value::Object(p.iter().map(|(k, v)| (k.clone(), v.clone().into())).collect()))).collect();
        out.push(("profiles", serde_json::Value::Object(profiles).to_string()));
        out.push(("auto_profiles", json_text(&self.auto_profiles)));
        out.push(("hud_themes", self.hud_themes.iter().map(|(a, b)| format!("{a}={b}")).collect::<Vec<_>>().join(";")));
        out.push(("hide_risky", self.hide_risky.to_string()));
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
        let base = match self.browse_theme() {
            Some(t) => t.accent,
            None if self.on(THEME) => ACCENT_RGB.get(self.v[ACCENT]).copied().unwrap_or(ACCENT_RGB[0]),
            None => ACCENT_RGB[0],
        };
        if self.on(ALBUM) {
            if let Some(c) = album_now().map(|p| p.0) {
                return mix_rgb(base, c, 0.8);
            }
        }
        base
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
    /// The HUD editor: drag, resize and reset Arsenal's HUD elements over the game.
    HudEdit,
    /// Profiles: save, load, update, delete, Auto per server; share codes.
    Profiles,
}

/// A HUD element being dragged: which, where the pointer holds it, and the layout before (for Undo).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Drag {
    pub id: &'static str,
    pub grab: [f32; 2],
    pub before: String,
    pub moved: bool,
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
    /// Modules: what's typed in the search box, and whether typing goes there.
    pub search: String,
    pub search_focus: bool,
    /// The HUD editor: the element held, the one picked, Undo's layouts and the snap lines shown.
    pub drag: Option<Drag>,
    pub picked: Option<&'static str>,
    pub undo: Vec<String>,
    pub guides: Vec<[f32; 4]>,
    /// The module waiting for its key (index in `JAVA_MODULES`).
    pub binding: Option<usize>,
    /// Profiles: the name being typed, whether typing goes there, and the profile (or "reset") waiting for a second click.
    pub profile_name: String,
    pub name_focus: bool,
    pub confirm: Option<String>,
}

/// The sidebar's Favorites row (after the categories in `category`).
pub const FAVORITES: usize = CATEGORIES.len();

#[derive(Default)]
struct State {
    /// Read once, then kept; the settings pages replace it.
    settings: Option<Settings>,
    /// The zoom while the key is held; None when it is up.
    held: Option<f32>,
    menu: Menu,
    menu_anim: MenuAnim,
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
    applied_v: [Option<String>; 8],
    saved_gamma: Option<f32>,
    gamma_checked: bool,
    /// The block aimed at this frame.
    aimed: Option<[i32; 3]>,
    /// Arsenal's own particles: where, which (1 crit, 2 sweep), since when, which way.
    fx: Vec<([f64; 3], u8, Instant, [f64; 3])>,
    kill_fx: Vec<([f64; 3], Instant)>,
    totem_flash: Option<Instant>,
    presence_at: Option<Instant>,
    extra: Extra,
    pack_done: bool,
    more: More,
    more2: More2,
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
    let mark = out.len();
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
    place(settings, &mut out, mark, "potions", w, h, width);
    let mark = out.len();
    if settings.on(ATTACK_BAR) && status.attack < 1.0 {
        out.push(Shape::Fill([cx - 8.0, cy + 9.0, 16.0, 2.0], 0xA000_0000));
        out.push(Shape::Fill([cx - 8.0, cy + 9.0, (16.0 * status.attack).round(), 2.0], 0xFF00_0000 | settings.accent()));
    }
    place(settings, &mut out, mark, "attack_bar", w, h, width);
    if settings.on(CRIT_IND) && falling && status.attack >= 0.9 {
        out.push(Shape::Quad([[cx, cy - 6.0], [cx + 2.0, cy - 4.0], [cx, cy - 2.0], [cx - 2.0, cy - 4.0]], 0xFFFF_C83D));
    }
    if settings.on(DYN_CROSSHAIR) && aiming {
        for (dx, dy) in [(-6.0, -1.0), (5.0, -1.0), (-1.0, -6.0), (-1.0, 5.0)] {
            out.push(Shape::Fill([cx + dx, cy + dy, 2.0, 2.0], 0xFFFF_5A5A));
        }
    }
    // The top-left lines, each its own HUD element.
    let mut corner: Vec<(String, u32, &'static str)> = Vec::new();
    if settings.on(CLOCK) {
        corner.push((clock.to_owned(), 0xFFFFFF, "clock"));
    }
    if settings.on(LAG) {
        match status.ping {
            Some(ms) => corner.push((format!("Ping {ms} ms"), ping_rgb(ms), "lag")),
            None => corner.push(("Ping -".into(), 0xAAAAAA, "lag")),
        }
    }
    STATE.with_borrow(|st| {
        let p = &st.pvp;
        if settings.on(SESSION_STATS) {
            let kd = p.kills_total as f32 / p.deaths_total.max(1) as f32;
            let mins = p.session_start.map_or(0, |t| t.elapsed().as_secs() / 60);
            let popped: u32 = p.pops.values().sum();
            corner.push((format!("K {}  D {}  K/D {kd:.2}", p.kills_total, p.deaths_total), 0xFFFFFF, "session_stats"));
            corner.push((format!("Pops {popped}  {}h {}m", mins / 60, mins % 60), 0xCCCCCC, "session_stats"));
        }
        if settings.on(COMBO_REACH) {
            let reach = p.reach.map_or("-".to_string(), |r| format!("{r:.2}"));
            corner.push((format!("Combo {}  Best {}  Reach {reach}", p.combo, p.best_combo), 0xFFFFFF, "combo_reach"));
        }
        if settings.on(FIGHT_TIMER) {
            if let Some(f) = &p.fight {
                corner.push((format!("Fight {}s vs {}", f.start.elapsed().as_secs(), f.with), 0xFFC83D, "fight_timer"));
            }
        }
        // Combo Counter Pop: the number rises and fades by the crosshair.
        let mark = out.len();
        if settings.on(COMBO_POP) {
            if let Some((n, at)) = p.combo_shown.filter(|c| c.0 >= 2 && c.1.elapsed() < Duration::from_millis(700)) {
                let t = at.elapsed().as_secs_f32();
                out.push(Shape::Text(format!("{n}x"), [cx + 10.0, cy - 12.0 - t * 14.0], 0xFFC83D, true));
            }
        }
        place(settings, &mut out, mark, "combo_pop", w, h, width);
        let mark = out.len();
        if settings.on(FEED) {
            let mut y = h - 70.0;
            for (line, _) in p.feed.iter().rev().take(5) {
                let tw = width(line);
                out.push(Shape::Text(line.clone(), [w - tw - 4.0, y], 0xFFFFFF, true));
                y -= 10.0;
            }
        }
        place(settings, &mut out, mark, "feed", w, h, width);
        let mark = out.len();
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
        place(settings, &mut out, mark, "fight_card", w, h, width);
        let _ = now;
    });
    let mark = out.len();
    if settings.on(INV_TOTEM) && totems > 0 {
        out.push(Shape::Item("minecraft:totem_of_undying".into(), [cx - 30.0, cy - 8.0]));
        out.push(Shape::Text(totems.to_string(), [cx - 13.0, cy - 4.0], 0xFFFFFF, true));
    }
    place(settings, &mut out, mark, "totems", w, h, width);
    let mut y = 4.0;
    let mut i = 0;
    while i < corner.len() {
        let id = corner[i].2;
        let mark = out.len();
        while i < corner.len() && corner[i].2 == id {
            let (text, rgb, _) = corner[i].clone();
            let tw = width(&text);
            out.push(Shape::Fill([2.0, y - 2.0, tw + 6.0, 11.0], BACKGROUND));
            out.push(Shape::Text(text, [5.0, y], rgb, false));
            y += 12.0;
            i += 1;
        }
        place(settings, &mut out, mark, id, w, h, width);
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
    /// Fullbright: the brightness to keep in Arsenal's settings (-1 forgets it), and whether to put the kept one back.
    pub keep_gamma: Option<f32>,
    pub restore_gamma: bool,
}

/// The lines Chat Filter hides (lower case pieces of them).
fn chat_hidden(settings: &Settings) -> Vec<String> {
    let mut spam: Vec<String> = Vec::new();
    if settings.on(SKYBLOCK) && settings.on(SB_SPAM) && STATE.with_borrow(|s| s.more2.skyblock.is_some()) {
        spam.extend(SKYBLOCK_SPAM.iter().map(|s| s.to_string()));
    }
    if !settings.on(CHAT_FILTER) {
        return spam;
    }
    let joins = ["joined the game", "left the game", "joined the server", "left the server"];
    let ads = ["discord.gg", "http://", "https://", "[ad]", "/vote", "store.", ".tebex"];
    let mut out: Vec<String> = Vec::new();
    match settings.v[FILTER_KIND] {
        1 => out.extend(joins.iter().map(|s| s.to_string())),
        2 => out.extend(ads.iter().map(|s| s.to_string())),
        _ => out.extend(joins.iter().chain(ads.iter()).map(|s| s.to_string())),
    }
    out.extend(spam);
    out
}

/// The visuals to apply now. A module switched off gives Horizon's default back once, then leaves the setting to the
/// player (Horizon's own PvP options keep working). `gamma` is the brightness now, saved before Fullbright raises it.
pub fn visuals(settings: &Settings, gamma: f32) -> Visuals {
    STATE.with_borrow_mut(|s| {
        let a = &mut s.applied;
        let (hide_particles, capture) = particle_rules(settings);
        let mut v = Visuals { particles: 100, chat_hide: chat_hidden(settings), hide_particles, capture, crystal_optimizer: settings.on(CRYSTAL_OPT), ..Default::default() };
        let last = &mut s.applied_v;
        let mut pick = |on: bool, slot: usize, value: Option<String>, off: String| -> Option<String> {
            if on {
                a[slot] = true;
                // Only when it changes: the player's own Horizon options aren't set again every tick.
                if last[slot] == value {
                    return None;
                }
                last[slot] = value.clone();
                value
            } else if std::mem::take(&mut a[slot]) {
                last[slot] = None;
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
                v.keep_gamma = Some(gamma);
            }
            v.gamma = Some(15.0);
        } else if let Some(g) = s.saved_gamma.take() {
            v.gamma = Some(g);
            v.keep_gamma = Some(-1.0);
        } else if gamma >= 2.0 && !s.gamma_checked {
            // Bright from a save made while Fullbright was on: the brightness kept from before.
            v.restore_gamma = true;
        }
        s.gamma_checked = true;
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
    let mark = out.len();
    if settings.on(COOLDOWN) {
        let mut x = cx - 91.0;
        for (id, left) in &world.cooldowns {
            out.push(Shape::Item(id.clone(), [x, h - 58.0]));
            out.push(Shape::Fill([x, h - 41.0, 16.0, 2.0], 0xA000_0000));
            out.push(Shape::Fill([x, h - 41.0, (16.0 * left).round(), 2.0], 0xFFFF_5A5A));
            x += 20.0;
        }
    }
    place(settings, &mut out, mark, "cooldowns", w, h, width);
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
    if !settings.on(KILL_FX) && !(settings.on(WARDROBE_FX) && wardrobe_style().is_some()) {
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
                match KILL_STYLES[kill_style(settings)].0 {
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
    if settings.on(SLICK) {
        p[6][0] = 1.0;
    }
    if settings.on(ALBUM) && settings.on(ALBUM_SKY) {
        if let Some((_, sky)) = album_now() {
            p[3] = [((sky >> 16) & 255) as f32 / 255.0, ((sky >> 8) & 255) as f32 / 255.0, (sky & 255) as f32 / 255.0, 0.55];
        }
    }
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
        p[5] = [rgb[0], rgb[1], rgb[2], -3.0];
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
//   mods[5]: Mist colour: rgb, w how far under the camera it lies (blocks)
//   mods[6]: x Slick Floors (1 on)

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
version = "1.2.0"
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

[[post]]
file = "post/fxaa.wgsl"
name = "Smoothing (packs turn off the game's own anti-aliasing)"

[budget]
gpu_ms = 2.0
"####),
    ("post/bloom.wgsl", r####"// Bloom: bright parts glow out softly. A disc of samples turned a little for each pixel (no rings), sized to the
// screen so it looks the same at any resolution; only what's brighter than the threshold glows.
fn post(input: PostInput) -> vec4<f32> {
    let strength = frame.mods[0].x;
    if strength <= 0.0 {
        return input.color;
    }
    let aspect = frame.resolution.x / frame.resolution.y;
    let radius = vec2<f32>(0.012 / aspect, 0.012);
    let spin = hash2(input.pixel.xy) * 6.2831853;
    var glow = vec3<f32>(0.0);
    var total = 0.0;
    for (var k = 0; k < 16; k++) {
        // Vogel's spiral: evenly spread over the disc.
        let r = sqrt((f32(k) + 0.5) / 16.0);
        let a = f32(k) * 2.3999632 + spin;
        let offset = vec2<f32>(cos(a), sin(a)) * radius * r;
        let s = textureSampleLevel(previous, linear_sampler, input.uv + offset, 0.0).rgb;
        let bright = max(luminance(s) - 0.65, 0.0);
        let w = 1.0 - r * 0.7;
        glow += s * bright * w;
        total += w;
    }
    let c = input.color.rgb + glow / max(total, 1.0) * strength * 4.0;
    return vec4<f32>(c, input.color.a);
}
"####),
    ("post/fxaa.wgsl", r####"// Smoothing: a light FXAA, as a pack with post passes turns off the game's own anti-aliasing.
fn post(input: PostInput) -> vec4<f32> {
    let px = 1.0 / frame.resolution;
    let c = input.color.rgb;
    let n = textureSampleLevel(previous, linear_sampler, input.uv + vec2<f32>(0.0, -px.y), 0.0).rgb;
    let s = textureSampleLevel(previous, linear_sampler, input.uv + vec2<f32>(0.0, px.y), 0.0).rgb;
    let e = textureSampleLevel(previous, linear_sampler, input.uv + vec2<f32>(px.x, 0.0), 0.0).rgb;
    let w = textureSampleLevel(previous, linear_sampler, input.uv + vec2<f32>(-px.x, 0.0), 0.0).rgb;
    let lc = luminance(c);
    let ln = luminance(n);
    let ls = luminance(s);
    let le = luminance(e);
    let lw = luminance(w);
    let lo = min(lc, min(min(ln, ls), min(le, lw)));
    let hi = max(lc, max(max(ln, ls), max(le, lw)));
    let range = hi - lo;
    if range < max(0.0312, hi * 0.125) {
        return input.color;
    }
    // Blur along the edge (across the larger change).
    let horizontal = abs(ln + ls - 2.0 * lc) >= abs(le + lw - 2.0 * lc);
    let dir = select(vec2<f32>(px.x, 0.0), vec2<f32>(0.0, px.y), horizontal);
    let a = textureSampleLevel(previous, linear_sampler, input.uv + dir * 0.5, 0.0).rgb;
    let b = textureSampleLevel(previous, linear_sampler, input.uv - dir * 0.5, 0.0).rgb;
    let amount = clamp(range / max(hi, 0.0001), 0.0, 1.0) * 0.6;
    return vec4<f32>(mix(c, (a + b) * 0.5, amount), input.color.a);
}
"####),
    ("post/grade.wgsl", r####"// World Look (saturation, contrast, exposure, vignette), Sky & World Colors' world tint, Mist low over the ground and
// Slick Floors.
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
        // mist.w: how far under the camera it lies, so caves and every world get it the same.
        let low = 1.0 - smoothstep(mist.w - 6.0, mist.w + 4.0, p.y);
        let far = 1.0 - exp(-length(p) * 0.03 * density);
        c = mix(c, mist.rgb, clamp(far * low, 0.0, 0.85));
    }
    // Slick Floors: up-facing ground near you darker and glossy, with puddles (the normal comes from the depth).
    if frame.mods[6].x > 0.0 && input.depth > 0.0 {
        let px = 2.0 / frame.resolution;
        let right = input.uv + vec2<f32>(px.x, 0.0);
        let down = input.uv + vec2<f32>(0.0, px.y);
        let dr = depth_at(right);
        let dd = depth_at(down);
        if dr > 0.0 && dd > 0.0 {
            let p = view_position(input.uv, input.depth);
            let n = normalize(cross(view_position(right, dr) - p, view_position(down, dd) - p));
            let near = 1.0 - smoothstep(6.0, 16.0, length(p));
            let ground = smoothstep(0.95, 0.995, abs(n.y)) * select(0.0, 1.0, p.y < -0.2) * near * frame.mods[6].x;
            if ground > 0.0 {
                let world = p.xz + frame.camera_position.xz;
                let puddle = smoothstep(0.55, 0.8, hash2(floor(world * 0.5)) * 0.6 + hash2(floor(world * 1.7)) * 0.4);
                let view = normalize(-p);
                let r = reflect(-view, vec3<f32>(0.0, 1.0, 0.0));
                let sun = pow(max(dot(r, normalize(frame.sun_direction)), 0.0), 40.0) * (1.0 - night());
                let fresnel = pow(1.0 - max(view.y, 0.0), 3.0);
                let sky = mix(vec3<f32>(0.45, 0.55, 0.7), vec3<f32>(0.05, 0.07, 0.12), night());
                let wet = mix(c * 0.68, c * 0.5 + sky * 0.35, puddle) + sky * fresnel * 0.18 + vec3<f32>(sun);
                c = mix(c, wet, ground);
            }
        }
    }
    if look.w > 0.0 {
        let aspect = frame.resolution.x / frame.resolution.y;
        let d = length((input.uv - 0.5) * vec2<f32>(aspect, 1.0)) / length(vec2<f32>(aspect, 1.0) * 0.5);
        c *= 1.0 - smoothstep(0.5, 1.15, d) * look.w;
    }
    return vec4<f32>(clamp(c, vec3<f32>(0.0), vec3<f32>(1.0)), input.color.a);
}
"####),
    ("post/motion.wgsl", r####"// Motion Blur: the last frame fades out over a set time (the same at any frame rate); none after a pause.
fn post(input: PostInput) -> vec4<f32> {
    let amount = clamp(frame.mods[0].y, 0.0, 1.0);
    let dt = frame.delta_time;
    if amount <= 0.0 || dt <= 0.0 || dt > 0.2 {
        return input.color;
    }
    // How much of the last frame is left after dt: a time constant of up to 60 ms.
    let k = min(exp(-dt / (0.06 * amount)), 0.75);
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


// ---- 1.13.0: the hotbar, inventories, hands and world, chat, sounds, keys, servers and account modules ----

/// What tick.rs puts into Horizon (Arsenal's patches/more.json): 0, empty or None leaves Horizon's own.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Looks {
    pub hotbar: u32,
    pub hotbar_accent: u32,
    pub glide: bool,
    pub inv_frame: u32,
    pub inv_glide: bool,
    pub swing: u32,
    pub lower: u32,
    pub blockhit: bool,
    pub hand_tint: u32,
    pub hand_glow: bool,
    pub crystal_size: u32,
    pub crystal_still: bool,
    pub big_items: Vec<(String, f32)>,
    pub armor_tint: u32,
    pub tag_size: u32,
    pub tag_bg: u32,
    pub tab_ping: bool,
    pub name_hide: Option<(String, String)>,
    pub mention: String,
    pub chat_keep: usize,
    pub volumes: Vec<(String, f32)>,
    pub obsidian_guard: bool,
    pub crystal_help: bool,
    /// Chat Tweaks' timestamps and stacking, Click to Copy Chat: Some when they change.
    pub chat: [Option<bool>; 3],
    pub reconnect: Option<bool>,
    pub freelook: bool,
}

/// Loot Big Items draws bigger.
pub const LOOT: &[&str] = &["minecraft:totem_of_undying", "minecraft:enchanted_golden_apple", "minecraft:golden_apple", "minecraft:netherite_ingot",
    "minecraft:netherite_scrap", "minecraft:ancient_debris", "minecraft:diamond", "minecraft:diamond_block", "minecraft:elytra", "minecraft:nether_star",
    "minecraft:netherite_sword", "minecraft:netherite_axe", "minecraft:netherite_pickaxe", "minecraft:netherite_helmet", "minecraft:netherite_chestplate",
    "minecraft:netherite_leggings", "minecraft:netherite_boots", "minecraft:mace", "minecraft:heavy_core", "minecraft:end_crystal", "minecraft:trident",
    "minecraft:beacon", "minecraft:shulker_box", "minecraft:enchanted_book", "minecraft:experience_bottle"];

/// TINTS as colours (0xRRGGBB): the last is the HUD accent.
pub fn tint_rgb(settings: &Settings, field: usize) -> u32 {
    match TINTS[settings.v[field]].0 {
        "purple" => 0xB884FF,
        "blue" => 0x5AA9FF,
        "pink" => 0xFF7AC8,
        "green" => 0x5CE08A,
        "orange" => 0xFF9A3C,
        "red" => 0xFF5A5A,
        _ => settings.accent(),
    }
}

fn option_num(settings: &Settings, field: usize) -> u32 {
    FIELDS[field].options[settings.v[field]].0.parse().unwrap_or(100)
}

#[derive(Default)]
struct More {
    /// Blocks broken (0) or placed (1) and when.
    blocks: Vec<([i32; 3], u8, Instant)>,
    /// Chat Tweaks (twice) and Click to Copy, Auto Reconnect: on last tick.
    was: [bool; 4],
    freelook_held: bool,
    /// Quick Menu: the feature picked while V is held.
    quick: Option<usize>,
    quick_len: usize,
    /// Sprint Reset Indicator: you hit since you started sprinting.
    sprint_used: bool,
    sprinting: bool,
    toasts: Vec<(String, Instant)>,
    server: Option<String>,
    hello_at: Option<Instant>,
    hello_wait: Duration,
    asked_wardrobe: bool,
    asked_update: bool,
    crash_checked: bool,
}

/// What other threads hand back (STATE is this thread's only).
#[derive(Default)]
struct NetMore {
    wardrobe: Option<usize>,
    update: Option<String>,
    /// Spotify: the song ("Artist - Song"), when it started, whether it plays, and its timed lines (seconds, text).
    #[allow(dead_code)]
    song: Option<String>,
    song_start: Option<Instant>,
    paused_at: Option<Instant>,
    lyrics: Vec<(f32, String)>,
    #[allow(dead_code)]
    lyrics_for: Option<String>,
}
static NET_MORE: std::sync::Mutex<NetMore> = std::sync::Mutex::new(NetMore {
    wardrobe: None, update: None, song: None, song_start: None, paused_at: None, lyrics: Vec::new(), lyrics_for: None,
});
fn net_more<T>(f: impl FnOnce(&mut NetMore) -> T) -> T {
    f(&mut NET_MORE.lock().unwrap_or_else(|e| e.into_inner()))
}

/// The settings put into Horizon each tick. `own` is your name (Name Hider, Chat Mentions).
pub fn looks(settings: &Settings, own: Option<&str>) -> Looks {
    let accent = 0xFF00_0000 | settings.accent();
    let mut l = Looks { swing: 100, crystal_size: 100, tag_size: 100, tag_bg: 0x3F00_0000, chat_keep: 100, ..Default::default() };
    if settings.on(HOTBAR_STYLE) {
        let bg = settings.panel_colors().0;
        l.hotbar = if bg >> 24 == 0 { 0xA000_0000 | (bg & 0xFF_FFFF) } else { bg };
        l.hotbar_accent = accent;
    }
    l.glide = settings.on(SMOOTH_HOTBAR);
    if settings.on(INV_STYLE) {
        l.inv_frame = accent;
    }
    l.inv_glide = settings.on(SMOOTH_INV);
    if settings.on(SWING_ANIM) {
        l.swing = option_num(settings, SWING_SPEED);
    }
    if settings.on(OLD_ANIM) {
        l.blockhit = true;
    }
    if settings.on(HAND_TINT) {
        l.hand_tint = settings.accent();
        l.hand_glow = settings.on(HAND_GLOW);
    }
    if settings.on(SMALL_CRYSTALS) {
        l.crystal_size = option_num(settings, CRYSTAL_SIZE);
        l.crystal_still = settings.on(CRYSTAL_STILL);
    }
    if settings.on(BIG_ITEMS) {
        l.big_items = if BIG_WHAT[settings.v[BIG_WHICH]].0 == "all" {
            vec![("*".into(), 2.0)]
        } else {
            LOOT.iter().map(|i| (i.to_string(), 2.0)).collect()
        };
    }
    if settings.on(ARMOR_COLOR) {
        l.armor_tint = tint_rgb(settings, ARMOR_TINT);
    }
    if settings.on(TAG_STYLE) {
        l.tag_size = option_num(settings, TAG_SIZE);
        l.tag_bg = match TAG_BGS[settings.v[TAG_BG]].0 { "none" => 0, "darker" => 0x8000_0000, _ => 0x3F00_0000 };
    }
    l.tab_ping = settings.on(TAB_PING);
    if let Some(name) = own.filter(|n| !n.is_empty()) {
        if settings.on(NAME_HIDER) {
            l.name_hide = Some((name.to_owned(), HIDE_AS[settings.v[HIDE_NAME_AS]].0.to_owned()));
        }
        if settings.on(MENTIONS) {
            l.mention = name.to_lowercase();
        }
    }
    if settings.on(CHAT_TWEAKS) {
        l.chat_keep = 500;
    }
    if settings.on(SOUND_TWEAKS) {
        let v = |f: usize| option_num(settings, f) as f32 / 100.0;
        l.volumes = vec![("explode".into(), v(VOL_EXPLODE)), ("totem.use".into(), v(VOL_TOTEM)), ("player.hurt".into(), v(VOL_HIT)),
            ("player.attack".into(), v(VOL_HIT)), ("mace.smash".into(), v(VOL_MACE))];
    }
    l.obsidian_guard = settings.on(OBSIDIAN_GUARD);
    l.crystal_help = settings.on(CRYSTAL_HELPER);
    STATE.with_borrow_mut(|s| {
        let m = &mut s.more;
        let now = [settings.on(CHAT_TWEAKS), settings.on(CHAT_TWEAKS), settings.on(CHAT_COPY), settings.on(RECONNECT)];
        let change = |i: usize, was: &[bool; 4]| if now[i] && !was[i] { Some(true) } else if was[i] && !now[i] { Some(false) } else { None };
        l.chat = [change(0, &m.was), change(1, &m.was), change(2, &m.was)];
        l.reconnect = change(3, &m.was);
        m.was = now;
        l.freelook = settings.on(FREE_LOOK) && m.freelook_held;
    });
    l
}

/// The keys this version adds (id, label, default).
pub const MORE_KEYS: [(&str, &str, &str); 4] = [("key_free_look", "Free Look (hold)", "key.keyboard.left.alt"), ("key_quick_menu", "Quick Menu (hold)", "key.keyboard.v"),
    ("key_inspect", "Inspect blade", "key.keyboard.y"),
    ("key_performance", "Performance Mode", "key.keyboard.unknown")];

/// The features on Quick Menu's wheel when nothing is starred.
pub const QUICK: [usize; 8] = [BLOOM, LOOK, MOTION, MIST, HITBOXES, PARTICLES, FREE_LOOK, PERF];

/// Quick Menu's wheel: the starred modules that run on Horizon (up to 10), else the usual eight.
pub fn quick_list(settings: &Settings) -> Vec<usize> {
    let starred: Vec<usize> = settings.favorites.iter().filter_map(|f| on_horizon(f).map(|(field, _)| field)).take(10).collect();
    if starred.is_empty() { QUICK.to_vec() } else { starred }
}

/// The wheel's entry the mouse points to (the pointer's pull from the middle), else the one the wheel picked.
pub fn quick_pick(n: usize, aim: Option<(f32, f32)>, wheel: usize) -> usize {
    match aim {
        Some((x, y)) if (x * x + y * y).sqrt() > 12.0 && n > 0 => {
            let a = y.atan2(x) + std::f32::consts::FRAC_PI_2;
            ((a / std::f32::consts::TAU * n as f32).round() as i64).rem_euclid(n as i64) as usize
        }
        _ => wheel.min(n.saturating_sub(1)),
    }
}

/// A key of this version went down or up: Free Look and Quick Menu are held; a feature to switch comes back.
pub fn more_key(settings: &Settings, id: &str, pressed: bool, playing: bool, aim: Option<(f32, f32)>) -> Option<usize> {
    STATE.with_borrow_mut(|s| {
        let m = &mut s.more;
        match id {
            "key_free_look" => {
                m.freelook_held = pressed && playing;
                None
            }
            "key_quick_menu" if settings.on(QUICK_MENU) => {
                if pressed && playing {
                    // No pick yet: a tap of V switches nothing.
                    m.quick = Some(usize::MAX);
                    m.quick_len = quick_list(settings).len();
                    None
                } else {
                    let list = quick_list(settings);
                    let aimed = aim.is_some_and(|(x, y)| (x * x + y * y).sqrt() > 12.0);
                    m.quick.take().filter(|i| aimed || *i != usize::MAX).map(|i| list[quick_pick(list.len(), aim, if i == usize::MAX { 0 } else { i })])
                }
            }
            "key_performance" if pressed => Some(PERF),
            "key_inspect" if pressed && playing => {
                blade_inspect();
                None
            }
            _ => None,
        }
    })
}

/// Quick Menu's wheel moves the pick while V is held.
pub fn quick_wheel(steps: f32) -> bool {
    STATE.with_borrow_mut(|s| {
        let n = s.more.quick_len.max(1) as i32;
        let Some(i) = s.more.quick.as_mut() else { return false };
        *i = if *i == usize::MAX { 0 } else { (*i as i32 - steps.signum() as i32).rem_euclid(n) as usize };
        true
    })
}

/// Switches `field` in the stored settings: the new settings, what to store and the message.
pub fn toggled(raw: &Settings, field: usize) -> (Settings, &'static str, String) {
    let mut s = raw.clone();
    s.v[field] = if s.v[field] == 1 { 0 } else { 1 };
    let value = FIELDS[field].options[s.v[field]].0;
    let message = format!("{}: {}", FIELDS[field].label, FIELDS[field].options[s.v[field]].1);
    (s, value, message)
}

/// Sprint Reset Indicator: you're sprinting (each tick) and you attacked (the left button).
pub fn sprint_tick(sprinting: bool) {
    STATE.with_borrow_mut(|s| {
        if !sprinting {
            s.more.sprint_used = false;
        }
        s.more.sprinting = sprinting;
    });
}
pub fn attacked() {
    STATE.with_borrow_mut(|s| {
        if s.more.sprinting {
            s.more.sprint_used = true;
        }
    });
}

/// Block Break and Place Animation: what you broke or placed since the last tick.
pub fn blocks_tick(settings: &Settings, events: &[([i32; 3], u8)]) {
    STATE.with_borrow_mut(|s| {
        let now = Instant::now();
        for (pos, kind) in events {
            if (*kind == 0 && settings.on(BREAK_ANIM)) || (*kind == 1 && settings.on(PLACE_ANIM)) {
                s.more.blocks.push((*pos, *kind, now));
            }
        }
        s.more.blocks.retain(|b| b.2.elapsed() < Duration::from_millis(700));
    });
}

/// Arsenal Messages: Some to show in chat, None when it became a toast.
pub fn route(settings: &Settings, note: String) -> Option<String> {
    if settings.on(MESSAGES) && MESSAGE_WHERE[settings.v[MESSAGES_WHERE]].0 == "toast" {
        STATE.with_borrow_mut(|s| {
            s.more.toasts.push((note, Instant::now()));
            if s.more.toasts.len() > 4 {
                s.more.toasts.remove(0);
            }
        });
        None
    } else {
        Some(note)
    }
}

/// A server address as a setting's name: "note_play.example.net".
pub fn server_key(kind: &str, address: &str) -> String {
    let clean: String = address.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' { c } else { '_' }).collect();
    format!("{kind}_{clean}")
}

/// Your setup as Server Profiles keeps it: "key=value;key=value".
pub fn profile_of(raw: &Settings) -> String {
    FIELDS.iter().enumerate().filter(|(i, _)| *i != PROFILES).map(|(i, f)| format!("{}={}", f.key, f.options[raw.v[i]].0)).collect::<Vec<_>>().join(";")
}

/// A kept profile's pairs.
pub fn profile_pairs(stored: &str) -> Vec<(String, String)> {
    stored.split(';').filter_map(|p| p.split_once('=')).filter(|(k, _)| FIELDS.iter().any(|f| f.key == *k)).map(|(k, v)| (k.to_owned(), v.to_owned())).collect()
}

/// What joining a server asks for.
#[derive(Debug, PartialEq)]
pub enum Joined {
    Note(String),
    Profile(String),
}

/// Server Notes and Profiles: the server changed to `server`; `stored` reads a setting.
pub fn server_tick(settings: &Settings, server: Option<&str>, stored: &dyn Fn(&str) -> Option<String>) -> Vec<Joined> {
    let changed = STATE.with_borrow_mut(|s| {
        let now = server.map(str::to_owned);
        (s.more.server != now).then(|| std::mem::replace(&mut s.more.server, now))
    });
    let (Some(_), Some(address)) = (changed, server) else { return Vec::new() };
    let mut out = Vec::new();
    if settings.on(PROFILES) {
        if let Some(p) = stored(&server_key("profile", address)).filter(|p| !p.is_empty()) {
            out.push(Joined::Profile(p));
        }
    }
    if settings.on(NOTES) {
        if let Some(n) = stored(&server_key("note", address)).filter(|n| !n.is_empty()) {
            out.push(Joined::Note(format!("Note for {address}: {n}")));
        }
    }
    out
}

/// What `/arsenal ...` asks for.
#[derive(Debug, PartialEq)]
pub enum Command {
    Say(String),
    Store(String, String, String),
    SendCrash,
    ForgetCrash,
    /// Cloud Backup: save (true) or restore.
    Cloud(bool),
}

/// `/arsenal <args>`: notes, profiles and crash reports.
pub fn command(settings: &Settings, raw: &Settings, args: &str, server: Option<&str>, stored: &dyn Fn(&str) -> Option<String>) -> Command {
    let args = args.trim();
    let (word, rest) = args.split_once(' ').map_or((args, ""), |(w, r)| (w, r.trim()));
    match word {
        "note" => {
            if !settings.on(NOTES) {
                return Command::Say("Turn on Server Notes in Arsenal's menu first.".into());
            }
            let Some(address) = server else { return Command::Say("Notes are kept per server: join one first.".into()) };
            let key = server_key("note", address);
            match rest {
                "" => Command::Say(stored(&key).filter(|n| !n.is_empty()).map_or_else(|| format!("No note for {address}."), |n| format!("Note for {address}: {n}"))),
                "clear" => Command::Store(key, String::new(), format!("Note for {address} forgotten.")),
                text => Command::Store(key, text.chars().take(200).collect(), format!("Note kept for {address}.")),
            }
        }
        "profile" => {
            if !settings.on(PROFILES) {
                return Command::Say("Turn on Server Profiles in Arsenal's menu first.".into());
            }
            let Some(address) = server else { return Command::Say("Profiles are kept per server: join one first.".into()) };
            let key = server_key("profile", address);
            match rest {
                "save" => Command::Store(key, profile_of(raw), format!("Your setup loads whenever you join {address}.")),
                "clear" => Command::Store(key, String::new(), format!("{address} no longer loads a setup.")),
                _ => Command::Say("/arsenal profile save or /arsenal profile clear".into()),
            }
        }
        "cloud" => match rest {
            "save" => Command::Cloud(true),
            "restore" => Command::Cloud(false),
            _ => Command::Say("/arsenal cloud save or /arsenal cloud restore".into()),
        },
        "crash" => match rest {
            "forget" => Command::ForgetCrash,
            _ => Command::SendCrash,
        },
        _ => Command::Say("Arsenal: /arsenal note <text>, /arsenal note clear, /arsenal profile save, /arsenal profile clear, /arsenal cloud save, /arsenal cloud restore".into()),
    }
}

/// Crash Reports: Arsenal's own panics are written to `dir/crash.txt` (message and place, no names or paths).
pub fn crash_hook(dir: &std::path::Path) {
    static ONCE: std::sync::Once = std::sync::Once::new();
    let file = dir.join("crash.txt");
    ONCE.call_once(move || {
        let before = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            if let Some(at) = info.location().filter(|l| l.file().contains("arsenal")) {
                let message = info.payload().downcast_ref::<&str>().map(|s| s.to_string())
                    .or_else(|| info.payload().downcast_ref::<String>().cloned()).unwrap_or_default();
                let name = at.file().rsplit(['/', '\\']).next().unwrap_or("");
                let _ = std::fs::create_dir_all(file.parent().unwrap_or(std::path::Path::new(".")));
                let _ = std::fs::write(&file, format!("{} at {}:{}:{} (Arsenal Horizon {VERSION})", scrub(&message), name, at.line(), at.column()));
            }
            before(info);
        }));
    });
}

/// Paths and user names out of a crash message.
pub fn scrub(text: &str) -> String {
    text.split_whitespace().map(|w| if w.contains(":\\") || w.contains(":/") || w.starts_with('/') || w.contains("Users") { "<path>" } else { w })
        .collect::<Vec<_>>().join(" ")
}

/// Crash Reports at start: the note offering to send it, once.
pub fn crash_note(settings: &Settings, dir: &std::path::Path) -> Option<String> {
    let first = STATE.with_borrow_mut(|s| !std::mem::replace(&mut s.more.crash_checked, true));
    (first && settings.on(CRASHES) && dir.join("crash.txt").is_file())
        .then(|| "Arsenal crashed your game last time. /arsenal crash sends the error (no names or paths) so it gets fixed; /arsenal crash forget drops it.".into())
}

#[cfg(not(test))]
pub fn send_crash(dir: &std::path::Path) {
    let file = dir.join("crash.txt");
    let Ok(trace) = std::fs::read_to_string(&file) else { return };
    let _ = std::fs::remove_file(&file);
    std::thread::spawn(move || {
        let body = serde_json::json!({ "trace": trace, "version": VERSION, "mc": "26.3", "mods": ["arsenal-horizon"] }).to_string();
        let _ = ureq::post(&format!("{WEBSITE}/v1/crash")).timeout(Duration::from_secs(15)).set("User-Agent", "Arsenal-Horizon")
            .set("Content-Type", "application/json").send_string(&body);
    });
}
#[cfg(test)]
pub fn send_crash(_dir: &std::path::Path) {}

/// Usage Stats and Arsenal Users: the website's hello is due (at start, then every 6 hours; again soon if it couldn't go).
pub fn hello_due(settings: &Settings) -> bool {
    (settings.on(USAGE) || settings.on(USERS)) && STATE.with_borrow(|s| s.more.hello_at.is_none_or(|t| t.elapsed() > s.more.hello_wait))
}

/// Sends the hello: `signed` is the body Horizon signed ({uuid, ts, sig, cert}), None when it couldn't (retried in 30 s).
pub fn send_hello(settings: &Settings, signed: Option<String>) {
    let Some(signed) = signed else {
        STATE.with_borrow_mut(|s| {
            s.more.hello_at = Some(Instant::now());
            s.more.hello_wait = Duration::from_secs(30);
        });
        return;
    };
    STATE.with_borrow_mut(|s| {
        s.more.hello_at = Some(Instant::now());
        s.more.hello_wait = Duration::from_secs(6 * 3600);
    });
    let Ok(serde_json::Value::Object(mut body)) = serde_json::from_str::<serde_json::Value>(&signed) else { return };
    body.insert("version".into(), VERSION.into());
    body.insert("mc".into(), "26.3".into());
    body.insert("users".into(), settings.on(USERS).into());
    body.insert("stats".into(), settings.on(USAGE).into());
    post_json("/v1/hello", serde_json::Value::Object(body).to_string());
}

#[cfg(not(test))]
fn post_json(path: &'static str, body: String) {
    std::thread::spawn(move || {
        let _ = ureq::post(&format!("{WEBSITE}{path}")).timeout(Duration::from_secs(15)).set("User-Agent", "Arsenal-Horizon")
            .set("Content-Type", "application/json").send_string(&body);
    });
}
#[cfg(test)]
fn post_json(_path: &'static str, _body: String) {}

/// Wardrobe Kill Effects: the kill effect you equipped, as Kill Effects' style (burst, ring, lightning).
pub fn wardrobe_style() -> Option<usize> {
    net_more(|n| n.wardrobe)
}
/// The style a website kill effect id plays as.
pub fn effect_style(id: &str) -> usize {
    match id {
        "lightning" => 2,
        "firework" | "totem_burst" => 0,
        _ => 1,
    }
}
fn kill_style(settings: &Settings) -> usize {
    if settings.on(WARDROBE_FX) {
        if let Some(i) = wardrobe_style() {
            return i;
        }
    }
    settings.v[KILL_STYLE]
}

/// Asks the website once for your equipped kill effect (`uuid` is your account's).
pub fn ask_wardrobe(settings: &Settings, uuid: Option<u128>) {
    let Some(uuid) = uuid else { return };
    if !settings.on(WARDROBE_FX) || STATE.with_borrow_mut(|s| std::mem::replace(&mut s.more.asked_wardrobe, true)) {
        return;
    }
    lookup_effect(user_hash(uuid));
}
#[cfg(not(test))]
fn lookup_effect(hash: String) {
    std::thread::spawn(move || {
        let body = serde_json::json!({ "hashes": [hash.clone()] }).to_string();
        let answer = ureq::post(&format!("{WEBSITE}/v1/cosmetics/lookup")).timeout(Duration::from_secs(15)).set("User-Agent", "Arsenal-Horizon")
            .set("Content-Type", "application/json").send_string(&body).ok().and_then(|r| r.into_string().ok());
        let id = answer.and_then(|a| serde_json::from_str::<serde_json::Value>(&a).ok())
            .and_then(|v| v["killEffects"][&hash].as_str().map(str::to_owned));
        net_more(|n| n.wardrobe = id.as_deref().map(effect_style));
    });
}
#[cfg(test)]
fn lookup_effect(_hash: String) {}

/// Update Notice: the newest Arsenal Horizon's version from its releases ("...-arsenal-1.13.0").
pub fn release_version(tag: &str) -> Option<String> {
    let v = tag.rsplit("-arsenal-").next().filter(|_| tag.contains("-arsenal-"))?;
    (v.split('.').count() == 3 && v.split('.').all(|p| p.parse::<u32>().is_ok())).then(|| v.to_owned())
}
/// Whether `a` is a newer version than `b`.
pub fn newer(a: &str, b: &str) -> bool {
    let parse = |v: &str| v.split('.').map(|p| p.parse::<u32>().unwrap_or(0)).collect::<Vec<_>>();
    parse(a) > parse(b)
}
/// Asks once a start; the note when a newer one is out.
pub fn update_note(settings: &Settings) -> Option<String> {
    if !settings.on(UPDATES) {
        return None;
    }
    if !STATE.with_borrow_mut(|s| std::mem::replace(&mut s.more.asked_update, true)) {
        ask_update();
    }
    let latest = net_more(|n| n.update.take())?;
    newer(&latest, VERSION).then(|| format!("Arsenal Horizon {latest} is out (you have {VERSION}): the launcher installs it the next time you press Play."))
}
#[cfg(not(test))]
fn ask_update() {
    std::thread::spawn(|| {
        let answer = ureq::get("https://api.github.com/repos/SoloHandCoder/arsenal-horizon/releases/latest").timeout(Duration::from_secs(15))
            .set("User-Agent", "Arsenal-Horizon").call().ok().and_then(|r| r.into_string().ok());
        let v = answer.and_then(|a| serde_json::from_str::<serde_json::Value>(&a).ok()).and_then(|v| v["tag_name"].as_str().and_then(release_version));
        net_more(|n| n.update = v);
    });
}
#[cfg(test)]
fn ask_update() {}

/// Screenshot to Clipboard: copies the image with the system's own tool.
#[cfg(not(test))]
pub fn copy_image(path: &std::path::Path) {
    let p = path.to_string_lossy().to_string();
    std::thread::spawn(move || {
        let mut cmd;
        if cfg!(windows) {
            cmd = std::process::Command::new("powershell");
            cmd.args(["-NoProfile", "-STA", "-Command",
                &format!("Add-Type -AssemblyName System.Windows.Forms,System.Drawing; [System.Windows.Forms.Clipboard]::SetImage([System.Drawing.Image]::FromFile('{}'))", p.replace('\'', "''"))]);
            #[cfg(windows)]
            {
                use std::os::windows::process::CommandExt;
                cmd.creation_flags(0x0800_0000);
            }
        } else if cfg!(target_os = "macos") {
            cmd = std::process::Command::new("osascript");
            cmd.args(["-e", &format!("set the clipboard to (read (POSIX file \"{}\") as «class PNGf»)", p.replace('"', ""))]);
        } else {
            cmd = std::process::Command::new("xclip");
            cmd.args(["-selection", "clipboard", "-t", "image/png", "-i", &p]);
        }
        let _ = cmd.stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).status();
    });
}
#[cfg(test)]
pub fn copy_image(_path: &std::path::Path) {}

/// Spotify Lyrics: "[mm:ss.xx] text" lines as (seconds, text).
pub fn parse_lrc(text: &str) -> Vec<(f32, String)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let Some(rest) = line.strip_prefix('[') else { continue };
        let Some((stamp, words)) = rest.split_once(']') else { continue };
        let Some((m, sec)) = stamp.split_once(':') else { continue };
        if let (Ok(m), Ok(sec)) = (m.parse::<f32>(), sec.parse::<f32>()) {
            out.push((m * 60.0 + sec, words.trim().to_owned()));
        }
    }
    out
}

/// Spotify's window title as the song: "Artist - Song", none while paused or not playing.
#[allow(dead_code)]
pub fn song_of(title: &str) -> Option<String> {
    let t = title.trim();
    (t.contains(" - ") && !t.starts_with("Spotify")).then(|| t.to_owned())
}

fn percent(text: &str) -> String {
    text.bytes().map(|b| if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) { (b as char).to_string() } else { format!("%{b:02X}") }).collect()
}

/// Spotify Lyrics and Album Colors read Spotify through the media reader (started once).
pub fn lyrics_tick(settings: &Settings) {
    if settings.on(LYRICS) || settings.on(ALBUM) {
        media_start();
    }
}

/// The lyric line sung `at` seconds into the song, and the next.
pub fn lyric_at(lines: &[(f32, String)], at: f32) -> Option<(&str, Option<&str>)> {
    let i = lines.iter().rposition(|l| l.0 <= at)?;
    Some((lines[i].1.as_str(), lines.get(i + 1).map(|l| l.1.as_str())))
}

/// Party: friends among the players around, with health, distance and the way to them (degrees from where you face).
pub fn party(others: &[Other], friends: &[String], me: [f64; 3], yaw: f32) -> Vec<(String, f32, f64, f32)> {
    let mut out: Vec<(String, f32, f64, f32)> = others.iter().filter(|o| friends.iter().any(|f| f.eq_ignore_ascii_case(&o.name))).map(|o| {
        let (dx, dz) = (o.pos[0] - me[0], o.pos[2] - me[2]);
        let distance = (dx * dx + dz * dz + (o.pos[1] - me[1]).powi(2)).sqrt();
        // Minecraft's yaw: 0 faces +z, 90 faces -x.
        let to = (-dx).atan2(dz).to_degrees() as f32;
        (o.name.clone(), o.health, distance, (to - yaw + 540.0).rem_euclid(360.0) - 180.0)
    }).collect();
    out.sort_by(|a, b| a.2.total_cmp(&b.2));
    out.truncate(6);
    out
}

/// Draws this version's HUD and world parts: block animations, the sprint bolt, toasts, Party, lyrics and Quick Menu.
#[allow(clippy::too_many_arguments)]
pub fn later_shapes(settings: &Settings, others: &[Other], friends: &[String], me: Option<([f64; 3], f32)>, attack: f32, aim: Option<(f32, f32)>, cam: Option<&Cam>,
                    w: f32, h: f32, width: &dyn Fn(&str) -> f32) -> Vec<Shape> {
    let mut out = Vec::new();
    let accent = settings.accent();
    let (blocks, sprint, toasts, quick) = STATE.with_borrow_mut(|s| {
        s.more.toasts.retain(|t| t.1.elapsed() < Duration::from_secs(5));
        (s.more.blocks.clone(), s.more.sprinting && !s.more.sprint_used, s.more.toasts.clone(), s.more.quick)
    });
    if let Some(cam) = cam {
        for (pos, kind, at) in &blocks {
            let t = (at.elapsed().as_secs_f32() / 0.7).min(1.0);
            let alpha = ((1.0 - t) * 255.0) as u32;
            let argb = (alpha << 24) | accent;
            let c = [pos[0] as f64 + 0.5, pos[1] as f64 + 0.5, pos[2] as f64 + 0.5];
            let cube = |half: f64, at: [f64; 3], out: &mut Vec<Shape>| {
                box_edges(cam, [at[0] - half, at[1] - half, at[2] - half], [at[0] + half, at[1] + half, at[2] + half], w, h, argb, out)
            };
            let kind_name = if *kind == 0 { BREAK_KINDS[settings.v[BREAK_KIND]].0 } else { PLACE_KINDS[settings.v[PLACE_KIND]].0 };
            match kind_name {
                "shatter" => {
                    for k in 0..8 {
                        let a = k as f64 * 0.785 + 0.3;
                        let (dx, dz) = (a.cos(), a.sin());
                        let tt = t as f64;
                        let p = [c[0] + dx * tt * 1.2, c[1] + tt * 1.4 - tt * tt * 2.2 + (k % 3) as f64 * 0.1, c[2] + dz * tt * 1.2];
                        cube(0.12 * (1.0 - tt) + 0.02, p, &mut out);
                    }
                }
                "shrink" => cube(0.5 * (1.0 - t as f64), c, &mut out),
                "pop" => {
                    let grow = (t * 2.0).min(1.0) as f64;
                    let over = 1.0 + (std::f64::consts::PI * grow).sin() * 0.15;
                    cube(0.5 * grow * over, c, &mut out);
                }
                "ripple" => {
                    let r = 0.5 + t as f64 * 1.6;
                    let y = pos[1] as f64 + 0.02;
                    box_edges(cam, [c[0] - r, y, c[2] - r], [c[0] + r, y, c[2] + r], w, h, argb, &mut out);
                }
                _ => {
                    cube(0.51, c, &mut out);
                    cube(0.53, c, &mut out);
                }
            }
        }
    }
    let (cx, cy) = (w / 2.0, h / 2.0);
    if settings.on(SPRINT_IND) && sprint && attack >= 0.9 {
        let g = 0xFF55_FF55;
        let (x, y) = (cx + 8.0, cy - 4.0);
        out.push(Shape::Quad([[x + 3.0, y], [x + 6.0, y], [x + 3.5, y + 4.0], [x + 1.0, y + 4.0]], g));
        out.push(Shape::Quad([[x + 2.5, y + 3.5], [x + 5.5, y + 3.5], [x + 1.5, y + 8.5], [x + 3.0, y + 3.5]], g));
    }
    let mark = out.len();
    for (i, (text, _)) in toasts.iter().enumerate() {
        let tw = width(text);
        let y = 4.0 + i as f32 * 14.0;
        out.push(Shape::Fill([w - tw - 12.0, y, tw + 8.0, 12.0], 0xB000_0000));
        out.push(Shape::Fill([w - tw - 12.0, y, 2.0, 12.0], 0xFF00_0000 | accent));
        out.push(Shape::Text(text.clone(), [w - tw - 7.0, y + 2.0], 0xFFFFFF, false));
    }
    place(settings, &mut out, mark, "toasts", w, h, width);
    let mark = out.len();
    if settings.on(PARTY) {
        if let Some((pos, yaw)) = me {
            for (i, (name, health, distance, angle)) in party(others, friends, pos, yaw).iter().enumerate() {
                let y = h / 2.0 - 40.0 + i as f32 * 12.0;
                let text = format!("{name}  {:.0}hp  {:.0}m", health, distance);
                out.push(Shape::Text(text, [16.0, y], 0xFFFFFF, true));
                let a = angle.to_radians();
                let (s, c) = (a.sin(), -a.cos());
                let (ax, ay) = (8.0, y + 4.0);
                let tip = [ax + s * 4.0, ay + c * 4.0];
                let l = [ax - c * 3.0 - s * 3.0, ay + s * 3.0 - c * 3.0];
                let r = [ax + c * 3.0 - s * 3.0, ay - s * 3.0 - c * 3.0];
                out.push(Shape::Quad([tip, l, [ax, ay], r], 0xFF00_0000 | accent));
            }
        }
    }
    place(settings, &mut out, mark, "party", w, h, width);
    let mark = out.len();
    if settings.on(LYRICS) {
        let line = net_more(|n| {
            let at = n.song_start.map(|s| (n.paused_at.unwrap_or_else(Instant::now)).saturating_duration_since(s).as_secs_f32())?;
            lyric_at(&n.lyrics, at).map(|(a, b)| (a.to_owned(), b.map(str::to_owned)))
        });
        if let Some((now, next)) = line {
            let tw = width(&now);
            out.push(Shape::Text(now, [cx - tw / 2.0, 22.0], accent, true));
            if let Some(next) = next {
                let nw = width(&next);
                out.push(Shape::Text(next, [cx - nw / 2.0, 34.0], 0xAAAAAA, true));
            }
        }
    }
    place(settings, &mut out, mark, "lyrics", w, h, width);
    if let Some(wheel) = quick {
        let list = quick_list(settings);
        let aimed = aim.is_some_and(|(x, y)| (x * x + y * y).sqrt() > 12.0);
        let pick = if aimed || wheel != usize::MAX { quick_pick(list.len(), aim, if wheel == usize::MAX { 0 } else { wheel }) } else { usize::MAX };
        let n = list.len() as f32;
        rounded(&mut out, [cx - 18.0, cy - 18.0, 36.0, 36.0], 18.0, 0x9010_1014);
        if let Some((x, y)) = aim {
            let len = (x * x + y * y).sqrt().min(16.0);
            let a = y.atan2(x);
            rounded(&mut out, [cx + a.cos() * len - 2.5, cy + a.sin() * len - 2.5, 5.0, 5.0], 2.5, 0xFF00_0000 | accent);
        }
        for (i, f) in list.iter().enumerate() {
            let a = i as f32 / n * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
            let on = settings.v[*f] == 1;
            let text = FIELDS[*f].label.to_owned();
            let tw = width(&text) + 20.0;
            let (x, y) = (cx + a.cos() * 84.0 - tw / 2.0, cy + a.sin() * 58.0 - 8.0);
            let r = [x, y, tw, 16.0];
            if i == pick {
                rounded(&mut out, [r[0] - 2.0, r[1] - 2.0, r[2] + 4.0, r[3] + 4.0], 9.0, argb(accent, 0.35));
            }
            rounded(&mut out, r, 8.0, if i == pick { 0xF020_2028 } else { 0xD014_141A });
            rounded(&mut out, [r[0] + 5.0, r[1] + 5.0, 6.0, 6.0], 3.0, if on { 0xFF00_0000 | accent } else { 0x60FF_FFFF });
            out.push(Shape::Text(text, [r[0] + 14.0, r[1] + 4.0], 0xFFFFFF, false));
        }
        let hint = "Point, then let go";
        out.push(Shape::Text(hint.into(), [cx - width(hint) / 2.0, cy + 84.0], 0xAAAAAA, true));
    }
    out
}

/// The text of a quick toggle's toast when Arsenal Messages makes toasts.
pub fn toast(text: String) {
    STATE.with_borrow_mut(|s| s.more.toasts.push((text, Instant::now())));
}


// ---- Movable HUD elements ----

/// The HUD elements the editor moves: (id, name).
pub const HUD_ELEMENTS: &[(&str, &str)] = &[
    ("info_fps", "FPS"), ("info_cps", "CPS"), ("info_xyz", "Coordinates"), ("info_facing", "Facing"), ("info_speed", "Speed"),
    ("info_session", "Session time"), ("info_sneak", "Toggle Sneak"), ("graph", "FPS Graph"), ("health", "Health HUD"), ("saturation", "Saturation"), ("armor", "Armor HUD"),
    ("counter", "Item Counter"), ("warnings", "Warnings"), ("ready", "Ready Check"), ("potions", "Potion Timers"),
    ("attack_bar", "Attack Charge Bar"), ("clock", "Clock"), ("lag", "Lag Meter"), ("session_stats", "Session Stats"), ("combo_reach", "Combo & Reach"),
    ("fight_timer", "Fight Timer"), ("combo_pop", "Combo Pop"), ("feed", "Combat Feed"), ("fight_card", "Fight Summary"),
    ("totems", "Totem Count"), ("streak", "Kill Streaks"), ("combat_tag", "Combat Tag Timer"), ("mention", "Mention Ping"),
    ("pops", "Totem Pop Counter"), ("target", "Target HUD"), ("bow", "Bow Charge"), ("retotem", "Re-totem Timer"),
    ("goal", "Session Goal"), ("intel", "Opponent Intel"), ("summary", "Session Summary"), ("cooldowns", "Cooldown HUD"),
    ("toasts", "Arsenal toasts"), ("party", "Party"), ("lyrics", "Spotify Lyrics"), ("skyblock_info", "SkyBlock"),
    ("skyblock_timers", "SkyBlock timers"), ("skyblock_dungeon", "SkyBlock dungeon"), ("skyblock_slayer", "SkyBlock slayer"),
];

/// "id=col,row,x,y,scale;..." as the layout: each moved element's place, as the Java mod keeps it: which third of
/// the screen it's anchored to (col and row 0..2), how far from that anchor, and its size.
pub fn parse_layout(text: &str) -> Vec<(String, [f32; 5])> {
    text.split(';').filter_map(|p| {
        let (id, v) = p.split_once('=')?;
        let n: Vec<f32> = v.split(',').filter_map(|x| x.trim().parse().ok()).collect();
        (n.len() == 5 && HUD_ELEMENTS.iter().any(|e| e.0 == id.trim()) && n.iter().all(|x| x.is_finite()))
            .then(|| (id.trim().to_owned(), [n[0].round().clamp(0.0, 2.0), n[1].round().clamp(0.0, 2.0), n[2].clamp(-4000.0, 4000.0),
                n[3].clamp(-4000.0, 4000.0), n[4].clamp(0.5, 3.0)]))
    }).collect()
}

pub fn layout_text(layout: &[(String, [f32; 5])]) -> String {
    layout.iter().map(|(id, v)| format!("{id}={:.0},{:.0},{:.0},{:.0},{:.1}", v[0], v[1], v[2], v[3], v[4])).collect::<Vec<_>>().join(";")
}

/// Where an element of `size` anchored to third `part` (0, 1, 2) of `screen` starts.
fn anchor_base(part: f32, screen: f32, size: f32) -> f32 {
    screen * part / 2.0 - size * part / 2.0
}

/// An element at `at` (top left) of `size` as a layout entry: the third its middle is in, and the distance from there.
pub fn anchored(at: [f32; 2], size: [f32; 2], scale: f32, screen: [f32; 2]) -> [f32; 5] {
    let third = |mid: f32, full: f32| if mid < full / 3.0 { 0.0 } else if mid < full * 2.0 / 3.0 { 1.0 } else { 2.0 };
    let col = third(at[0] + size[0] / 2.0, screen[0]);
    let row = third(at[1] + size[1] / 2.0, screen[1]);
    [col, row, (at[0] - anchor_base(col, screen[0], size[0])).round(), (at[1] - anchor_base(row, screen[1], size[1])).round(), scale]
}

thread_local! {
    /// This frame's HUD elements as drawn: (id, where it is, where it would be unmoved).
    static HUD_BOXES: RefCell<Vec<(&'static str, [f32; 4], [f32; 4])>> = const { RefCell::new(Vec::new()) };
}

/// A HUD frame starts.
pub fn hud_begin() {
    HUD_BOXES.with_borrow_mut(|b| b.clear());
}

/// This frame's HUD elements: (id, rect drawn, rect unmoved).
pub fn hud_boxes() -> Vec<(&'static str, [f32; 4], [f32; 4])> {
    HUD_BOXES.with_borrow(|b| b.clone())
}

/// What `shapes` cover.
pub fn bounds(shapes: &[Shape], width: &dyn Fn(&str) -> f32) -> Option<[f32; 4]> {
    let mut b: Option<[f32; 4]> = None;
    let mut add = |r: [f32; 4]| {
        if r[2] <= 0.0 && r[3] <= 0.0 {
            return;
        }
        b = Some(match b {
            None => [r[0], r[1], r[0] + r[2], r[1] + r[3]],
            Some(o) => [o[0].min(r[0]), o[1].min(r[1]), o[2].max(r[0] + r[2]), o[3].max(r[1] + r[3])],
        });
    };
    for sh in shapes {
        match sh {
            Shape::Fill(r, c) => if c >> 24 != 0 { add(*r) },
            Shape::Text(t, at, _, _) => add([at[0], at[1], width(t), 8.0]),
            Shape::Styled(t, at, _, _, sc) => add([at[0], at[1], width(t) * sc, 8.0 * sc]),
            Shape::Item(_, at) => add([at[0], at[1], 16.0, 16.0]),
            Shape::Icon(_, at, size) => add([at[0], at[1], *size, *size]),
            Shape::Quad(p, _) => {
                let (x0, x1) = (p.iter().map(|q| q[0]).fold(f32::MAX, f32::min), p.iter().map(|q| q[0]).fold(f32::MIN, f32::max));
                let (y0, y1) = (p.iter().map(|q| q[1]).fold(f32::MAX, f32::min), p.iter().map(|q| q[1]).fold(f32::MIN, f32::max));
                add([x0, y0, x1 - x0, y1 - y0]);
            }
        }
    }
    b.map(|b| [b[0], b[1], b[2] - b[0], b[3] - b[1]])
}

/// A shape moved by `d`.
pub fn offset(shape: Shape, d: [f32; 2]) -> Shape {
    let p = |q: [f32; 2]| [q[0] + d[0], q[1] + d[1]];
    match shape {
        Shape::Fill(r, c) => Shape::Fill([r[0] + d[0], r[1] + d[1], r[2], r[3]], c),
        Shape::Text(t, at, rgb, sh) => Shape::Text(t, p(at), rgb, sh),
        Shape::Styled(t, at, c, sh, sc) => Shape::Styled(t, p(at), c, sh, sc),
        Shape::Item(i, at) => Shape::Item(i, p(at)),
        Shape::Icon(i, at, size) => Shape::Icon(i, p(at), size),
        Shape::Quad(q, c) => Shape::Quad(q.map(p), c),
    }
}

/// HUD element `id` is `out[from..]`: moved and sized as the HUD editor left it, kept on screen, and noted for the editor.
pub fn place(settings: &Settings, out: &mut Vec<Shape>, from: usize, id: &'static str, w: f32, h: f32, width: &dyn Fn(&str) -> f32) {
    let Some(home) = bounds(&out[from..], width) else { return };
    let entry = settings.layout.iter().find(|(i, _)| i == id).map(|e| e.1);
    let sc = entry.map_or(1.0, |e| e[4]);
    let (bw, bh) = (home[2] * sc, home[3] * sc);
    // Unmoved: where it draws itself. Moved: from its anchor, so a change of size or text never makes it jump.
    let (x, y) = match entry {
        Some([col, row, ax, ay, _]) => (anchor_base(col, w, bw) + ax, anchor_base(row, h, bh) + ay),
        None => (home[0], home[1]),
    };
    let x = x.clamp(0.0, (w - bw).max(0.0));
    let y = y.clamp(0.0, (h - bh).max(0.0));
    let moved = [x, y, bw, bh];
    if entry.is_some() || x != home[0] || y != home[1] {
        let (ox, oy) = (home[0], home[1]);
        let p = |q: [f32; 2]| [x + (q[0] - ox) * sc, y + (q[1] - oy) * sc];
        for sh in &mut out[from..] {
            *sh = match std::mem::replace(sh, Shape::Fill([0.0; 4], 0)) {
                Shape::Fill(r, c) => {
                    let a = p([r[0], r[1]]);
                    Shape::Fill([a[0], a[1], r[2] * sc, r[3] * sc], c)
                }
                Shape::Text(t, at, rgb, shadow) if sc != 1.0 => Shape::Styled(t, p(at), 0xFF00_0000 | rgb, shadow, sc),
                Shape::Text(t, at, rgb, shadow) => Shape::Text(t, p(at), rgb, shadow),
                Shape::Styled(t, at, c, shadow, s2) => Shape::Styled(t, p(at), c, shadow, s2 * sc),
                Shape::Item(i, at) if sc != 1.0 => Shape::Icon(i, p(at), 16.0 * sc),
                Shape::Item(i, at) => Shape::Item(i, p(at)),
                Shape::Icon(i, at, size) => Shape::Icon(i, p(at), size * sc),
                Shape::Quad(q, c) => Shape::Quad(q.map(p), c),
            };
        }
    }
    theme_element(settings, &mut out[from..], id);
    HUD_BOXES.with_borrow_mut(|b| b.push((id, moved, home)));
}


// ---- 1.14: SkyBlock, Album Colors and the media reader, Cloud Backup ----

#[derive(Default)]
struct More2 {
    /// In SkyBlock now (Hypixel), what the sidebar and tab list said.
    skyblock: Option<SkyBlock>,
    /// Booster Cookie and God Potion: seconds left when read, and when.
    cookie: Option<(u64, Instant)>,
    god: Option<(u64, Instant)>,
    /// The ability's cooldown: until when.
    ability: Option<Instant>,
    deaths: u32,
    run_start: Option<Instant>,
    cookie_warned: bool,
    cloud: Cloud,
}

/// SkyBlock's lines Hide spammy lines hides (lower case).
pub const SKYBLOCK_SPAM: &[&str] = &["kill combo", "there are blocks in the way", "you are playing on profile:", "watchdog has banned",
    "staff have banned an additional", "[watchdog announcement]", "this item is on cooldown", "autopet equipped your", "enemies for "];

/// What SkyBlock's sidebar and tab list show.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkyBlock {
    pub area: String,
    pub purse: Option<(String, f64)>,
    pub bits: Option<f64>,
    pub date: String,
    pub time: String,
    pub objective: String,
    pub pet: String,
    pub slayer: Option<(String, String)>,
    pub dungeon_time: String,
    pub cleared: Option<u32>,
    pub crypts: Option<u32>,
    pub secrets: Option<u32>,
}

/// A line without colour codes, odd symbols or extra spaces.
pub fn sb_clean(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c == '§' {
            chars.next();
            continue;
        }
        if matches!(c, '⚽' | '⛃' | '\u{200B}' | '\u{FE0F}') || ('\u{1F000}'..='\u{1FAFF}').contains(&c) {
            continue;
        }
        out.push(c);
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The first number in a line: "1,234.5" as 1234.5.
pub fn first_number(text: &str) -> Option<f64> {
    let start = text.find(|c: char| c.is_ascii_digit())?;
    let neg = start > 0 && text[..start].ends_with('-');
    let num: String = text[start..].chars().take_while(|c| c.is_ascii_digit() || *c == ',' || *c == '.').filter(|c| *c != ',').collect();
    num.trim_end_matches('.').parse::<f64>().ok().map(|v| if neg { -v } else { v })
}

/// "1d 2h 3m", "12:34" or "1:02:03" as seconds.
pub fn sb_duration(text: &str) -> Option<u64> {
    let t = text.to_lowercase();
    let mut total = 0u64;
    let mut found = false;
    let words: Vec<&str> = t.split_whitespace().collect();
    for w in &words {
        let digits: String = w.chars().take_while(|c| c.is_ascii_digit()).collect();
        let unit = &w[digits.len()..];
        if let (Ok(n), Some(u)) = (digits.parse::<u64>(), unit.chars().next()) {
            if unit.len() == 1 || unit.chars().nth(1).is_some_and(|c| !c.is_alphabetic()) {
                let mul = match u { 'y' => 365 * 86400, 'd' => 86400, 'h' => 3600, 'm' => 60, 's' => 1, _ => 0 };
                if mul > 0 {
                    total += n * mul;
                    found = true;
                }
            }
        }
    }
    if found {
        return Some(total);
    }
    let clock = words.iter().find(|w| w.contains(':') && w.chars().all(|c| c.is_ascii_digit() || c == ':'))?;
    let parts: Vec<u64> = clock.split(':').filter_map(|p| p.parse().ok()).collect();
    match parts.as_slice() {
        [m, s] => Some(m * 60 + s),
        [h, m, s] => Some(h * 3600 + m * 60 + s),
        _ => None,
    }
}

/// Coins as SkyBlock shows them short: 1.2B, 4.56M, 12.3k.
pub fn coins(v: f64) -> String {
    let a = v.abs();
    if a >= 1e9 {
        format!("{:.1}B", v / 1e9)
    } else if a >= 1e6 {
        format!("{:.2}M", v / 1e6)
    } else if a >= 1e4 {
        format!("{:.1}k", v / 1e3)
    } else if a >= 100.0 {
        let n = v.round() as i64;
        let s = n.abs().to_string();
        let mut out = String::new();
        for (i, c) in s.chars().enumerate() {
            if i > 0 && (s.len() - i) % 3 == 0 {
                out.push(',');
            }
            out.push(c);
        }
        if n < 0 { format!("-{out}") } else { out }
    } else {
        let t = format!("{v:.2}");
        t.trim_end_matches('0').trim_end_matches('.').to_owned()
    }
}

/// On Hypixel: the address is hypixel.net or under it.
pub fn on_hypixel(server: Option<&str>) -> bool {
    server.is_some_and(|a| {
        let host = a.rsplit_once(':').map_or(a, |(h, _)| h).to_lowercase();
        host == "hypixel.net" || host.ends_with(".hypixel.net")
    })
}

fn is_date(line: &str) -> bool {
    let rest = line.strip_prefix("Early ").or_else(|| line.strip_prefix("Late ")).unwrap_or(line);
    let Some((season, day)) = rest.split_once(' ') else { return false };
    matches!(season, "Spring" | "Summer" | "Autumn" | "Winter")
        && day.len() > 2 && day[..day.len() - 2].chars().all(|c| c.is_ascii_digit()) && ["st", "nd", "rd", "th"].contains(&&day[day.len() - 2..])
}

/// The sidebar (title then lines) and tab list read as SkyBlock: none when it isn't SkyBlock's.
pub fn read_skyblock(sidebar: &[String], tab: &[String]) -> Option<SkyBlock> {
    let title = sb_clean(sidebar.first()?).to_uppercase().replace(' ', "");
    if !title.contains("SKYBLOCK") {
        return None;
    }
    let lines: Vec<String> = sidebar.iter().skip(1).map(|l| sb_clean(l)).collect();
    let mut sb = SkyBlock::default();
    for (i, l) in lines.iter().enumerate() {
        let next = lines.get(i + 1).cloned().unwrap_or_default();
        if let Some(a) = l.strip_prefix('⏣').or_else(|| l.strip_prefix('ф')) {
            sb.area = a.trim().to_owned();
        } else if l.starts_with("Purse:") || l.starts_with("Piggy:") {
            sb.purse = first_number(&l[6..]).map(|v| (l[..5].to_owned(), v));
        } else if let Some(b) = l.strip_prefix("Bits:") {
            sb.bits = first_number(b);
        } else if is_date(l) {
            sb.date = l.clone();
        } else if sb.time.is_empty() && l.len() >= 5 && l.chars().next().is_some_and(|c| c.is_ascii_digit()) && l.contains(':')
            && (l.to_lowercase().contains("am") || l.to_lowercase().contains("pm")) {
            let low = l.to_lowercase();
            let end = low.find("am").or_else(|| low.find("pm")).map_or(low.len(), |i| i + 2);
            sb.time = low[..end].replace(' ', "");
        } else if l == "Objective" {
            sb.objective = next.clone();
        } else if l == "Slayer Quest" {
            let progress = lines.get(i + 2).cloned().unwrap_or_default();
            sb.slayer = Some((next.clone(), progress));
        } else if let Some(t) = l.strip_prefix("Time Elapsed:") {
            sb.dungeon_time = t.trim().to_owned();
        } else if let Some(c) = l.strip_prefix("Cleared:") {
            sb.cleared = first_number(c).map(|v| v as u32);
        }
    }
    let tab: Vec<String> = tab.iter().flat_map(|t| t.split('\n').map(sb_clean).collect::<Vec<_>>()).collect();
    for (i, l) in tab.iter().enumerate() {
        if let Some(p) = l.strip_prefix("Pet:") {
            let p = p.trim();
            let pet = if p.is_empty() { tab.get(i + 1).cloned().unwrap_or_default() } else { p.to_owned() };
            if !pet.is_empty() && pet != "None" && !pet.ends_with(':') {
                sb.pet = pet;
            }
        } else if let Some(a) = l.strip_prefix("Area: ").or_else(|| l.strip_prefix("Dungeon: ")) {
            if sb.area.is_empty() {
                sb.area = a.trim().to_owned();
            }
        } else if let Some(c) = l.strip_prefix("Crypts:") {
            sb.crypts = first_number(c).map(|v| v as u32);
        } else if let Some(c) = l.strip_prefix("Secrets Found:") {
            if !c.contains('%') {
                sb.secrets = first_number(c).map(|v| v as u32);
            }
        }
    }
    Some(sb)
}

/// Booster Cookie and God Potion from the tab list's lines: seconds left (0 when the cookie isn't active).
pub fn buff_times(tab: &[String]) -> (Option<u64>, Option<u64>) {
    let lines: Vec<String> = tab.iter().flat_map(|t| t.split('\n').map(sb_clean).collect::<Vec<_>>()).filter(|l| !l.is_empty()).collect();
    let mut cookie = None;
    let mut god = None;
    for (i, l) in lines.iter().enumerate() {
        if l.eq_ignore_ascii_case("Cookie Buff") {
            for n in lines.iter().skip(i + 1).take(2) {
                if n.to_lowercase().contains("not active") {
                    cookie = Some(0);
                    break;
                }
                if let Some(d) = sb_duration(n) {
                    cookie = Some(d);
                    break;
                }
            }
        }
        let low = l.to_lowercase();
        if let Some(at) = low.find("god potion") {
            if let Some(d) = sb_duration(&low[at + 10..]) {
                god = Some(d);
            }
        }
    }
    (cookie, god)
}

/// Each tick: on Hypixel, what the sidebar and tab list say (`tab` is the header, the players and the footer).
pub fn skyblock_tick(settings: &Settings, server: Option<&str>, sidebar: &[String], tab: &[String]) -> Option<String> {
    let sb = (settings.on(SKYBLOCK) && on_hypixel(server)).then(|| read_skyblock(sidebar, tab)).flatten();
    STATE.with_borrow_mut(|s| {
        let x = &mut s.more2;
        let now = Instant::now();
        if sb.is_some() {
            let (cookie, god) = buff_times(tab);
            if let Some(c) = cookie {
                x.cookie = Some((c, now));
            }
            if let Some(g) = god {
                x.god = Some((g, now));
            }
        }
        x.skyblock = sb;
        // One reminder when the cookie runs low (under 12 hours).
        let left = x.cookie.map(|(c, at)| c.saturating_sub(at.elapsed().as_secs()));
        if x.skyblock.is_some() && left.is_some_and(|l| l > 0 && l < 12 * 3600) && !x.cookie_warned {
            x.cookie_warned = true;
            return Some(format!("Your Booster Cookie runs out in {}", short_time(left.unwrap_or(0))));
        }
        None
    })
}

/// SkyBlock's chat: the ability's cooldown, dungeon deaths and runs.
pub fn skyblock_chat(lines: &[String]) {
    STATE.with_borrow_mut(|s| {
        let x = &mut s.more2;
        if x.skyblock.is_none() {
            return;
        }
        for l in lines {
            let l = sb_clean(l);
            if let Some(rest) = l.strip_prefix("This ability is on cooldown for ").or_else(|| l.strip_prefix("This ability is currently on cooldown for ")) {
                if let Some(secs) = first_number(rest) {
                    x.ability = Some(Instant::now() + Duration::from_secs_f64(secs));
                }
            } else if l.starts_with("[NPC] Mort: Here, I found this map when I first entered the dungeon.") {
                x.run_start = Some(Instant::now());
                x.deaths = 0;
            } else if l.starts_with("☠ ") && l.contains("became a ghost") {
                x.deaths += 1;
            } else if l == "> EXTRA STATS <" {
                x.run_start = None;
            }
        }
    });
}

fn short_time(secs: u64) -> String {
    if secs >= 86400 {
        format!("{}d {}h", secs / 86400, secs / 3600 % 24)
    } else if secs >= 3600 {
        format!("{}h {}m", secs / 3600, secs / 60 % 60)
    } else {
        format!("{}:{:02}", secs / 60, secs % 60)
    }
}

/// SkyBlock's HUD: the info card, timers, dungeon and slayer cards (each a HUD element the editor moves).
pub fn skyblock_shapes(settings: &Settings, w: f32, h: f32, width: &dyn Fn(&str) -> f32) -> Vec<Shape> {
    let mut out = Vec::new();
    if !settings.on(SKYBLOCK) {
        return out;
    }
    let accent = settings.accent();
    let (bg, text) = settings.panel_colors();
    let (sb, cookie, god, ability, deaths, run) = STATE.with_borrow(|s| {
        let x = &s.more2;
        (x.skyblock.clone(), x.cookie, x.god, x.ability, x.deaths, x.run_start)
    });
    let Some(sb) = sb else { return out };
    let card = |out: &mut Vec<Shape>, at: [f32; 2], lines: Vec<(String, u32)>, id: &'static str| {
        if lines.is_empty() {
            return;
        }
        let mark = out.len();
        let tw = lines.iter().map(|l| width(&l.0)).fold(40.0, f32::max);
        out.push(Shape::Fill([at[0], at[1], tw + 8.0, lines.len() as f32 * 10.0 + 4.0], bg));
        for (i, (t, c)) in lines.into_iter().enumerate() {
            out.push(Shape::Text(t, [at[0] + 4.0, at[1] + 3.0 + i as f32 * 10.0], c, false));
        }
        place(settings, out, mark, id, w, h, width);
    };
    let mut info = vec![("SkyBlock".to_owned(), accent)];
    if !sb.area.is_empty() {
        info.push((format!("Area: {}", sb.area), 0x55FFFF));
    }
    if let Some((label, v)) = &sb.purse {
        info.push((format!("{label}: {}", coins(*v)), 0xFFAA00));
    }
    if let Some(b) = sb.bits {
        info.push((format!("Bits: {}", coins(b)), 0x55FFFF));
    }
    if !sb.date.is_empty() || !sb.time.is_empty() {
        info.push((format!("{} {}", sb.date, sb.time).trim().to_owned(), 0xFFFF55));
    }
    if !sb.objective.is_empty() {
        info.push((format!("Objective: {}", sb.objective), text));
    }
    if !sb.pet.is_empty() {
        info.push((format!("Pet: {}", sb.pet), 0x55FF55));
    }
    card(&mut out, [4.0, 60.0], info, "skyblock_info");
    let mut timers = Vec::new();
    if let Some((c, at)) = cookie {
        let left = c.saturating_sub(at.elapsed().as_secs());
        timers.push((if left == 0 { "Booster Cookie: none".to_owned() } else { format!("Booster Cookie: {}", short_time(left)) },
                     if left < 12 * 3600 { 0xFF5555 } else { text }));
    }
    if let Some((g, at)) = god {
        let left = g.saturating_sub(at.elapsed().as_secs());
        if left > 0 {
            timers.push((format!("God Potion: {}", short_time(left)), text));
        }
    }
    if let Some(until) = ability.filter(|u| *u > Instant::now()) {
        timers.push((format!("Ability: {:.1}s", (until - Instant::now()).as_secs_f32()), 0xFFAA00));
    }
    card(&mut out, [4.0, 130.0], timers, "skyblock_timers");
    let in_dungeon = !sb.dungeon_time.is_empty() || sb.cleared.is_some() || run.is_some();
    if in_dungeon {
        let time = if !sb.dungeon_time.is_empty() { sb.dungeon_time.clone() } else { run.map_or(String::new(), |r| short_time(r.elapsed().as_secs())) };
        let mut d = vec![(format!("Time: {time}"), text)];
        if let Some(c) = sb.cleared {
            d.push((format!("Cleared: {c}%"), text));
        }
        d.push((format!("Deaths: {deaths}"), if deaths > 0 { 0xFF5555 } else { text }));
        if let Some(sec) = sb.secrets {
            d.push((format!("Secrets: {sec}"), text));
        }
        if let Some(c) = sb.crypts {
            d.push((format!("Crypts: {c}"), if c >= 5 { 0x55FF55 } else { text }));
        }
        card(&mut out, [4.0, h / 2.0 + 20.0], d, "skyblock_dungeon");
    }
    if let Some((boss, progress)) = &sb.slayer {
        card(&mut out, [4.0, h / 2.0 - 40.0], vec![(boss.clone(), 0xFF5555), (progress.clone(), text)], "skyblock_slayer");
    }
    out
}

/// Bazaar and NPC prices as fetched (item id to (buy, sell) and NPC sell), and when.
#[derive(Default)]
struct Prices {
    bazaar: std::collections::HashMap<String, (f64, f64)>,
    npc: std::collections::HashMap<String, f64>,
    at: Option<Instant>,
    asking: bool,
}
static PRICES: std::sync::Mutex<Option<Prices>> = std::sync::Mutex::new(None);

/// SkyBlock's price lines for the item `id` (its SkyBlock id) in a tooltip; asks Hypixel's public API now and then.
pub fn skyblock_price_lines(settings: &Settings, id: &str, count: u32) -> Vec<(String, u32)> {
    if !settings.on(SKYBLOCK) || !settings.on(SB_PRICES) || STATE.with_borrow(|s| s.more2.skyblock.is_none()) {
        return Vec::new();
    }
    let mut p = PRICES.lock().unwrap_or_else(|e| e.into_inner());
    let p = p.get_or_insert_with(Prices::default);
    if !p.asking && p.at.is_none_or(|t| t.elapsed() > Duration::from_secs(300)) {
        p.asking = true;
        fetch_prices();
    }
    let mut out = Vec::new();
    let n = count.max(1) as f64;
    if let Some((buy, sell)) = p.bazaar.get(id) {
        let mut line = format!("Bazaar: buy {} · sell {}", coins(*buy), coins(*sell));
        if count > 1 {
            line.push_str(&format!(" (x{count}: {} / {})", coins(buy * n), coins(sell * n)));
        }
        out.push((line, 0xFFAA00));
    }
    if let Some(v) = p.npc.get(id).filter(|v| **v > 0.0) {
        out.push((if count > 1 { format!("NPC sell: {} (x{count}: {})", coins(*v), coins(v * n)) } else { format!("NPC sell: {}", coins(*v)) }, 0xFFFF55));
    }
    if let Some(at) = p.at.filter(|_| !out.is_empty()) {
        out.push((format!("Hypixel public API, {} ago", short_time(at.elapsed().as_secs())), 0x555555));
    }
    out
}
#[cfg(not(test))]
fn fetch_prices() {
    std::thread::spawn(|| {
        let get = |url: &str| ureq::get(url).timeout(Duration::from_secs(20)).set("User-Agent", "Arsenal (arsenal.monoforge.net)").call().ok()
            .and_then(|r| r.into_string().ok()).and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok()).filter(|v| v["success"] == true);
        let bazaar = get("https://api.hypixel.net/v2/skyblock/bazaar").map(|v| {
            v["products"].as_object().map(|o| o.iter().filter_map(|(id, p)| {
                let q = &p["quick_status"];
                Some((id.clone(), (q["buyPrice"].as_f64()?, q["sellPrice"].as_f64()?)))
            }).collect::<std::collections::HashMap<_, _>>()).unwrap_or_default()
        });
        let npc = get("https://api.hypixel.net/v2/resources/skyblock/items").map(|v| {
            v["items"].as_array().map(|a| a.iter().filter_map(|i| Some((i["id"].as_str()?.to_owned(), i["npc_sell_price"].as_f64()?))).collect::<std::collections::HashMap<_, _>>())
                .unwrap_or_default()
        });
        let mut p = PRICES.lock().unwrap_or_else(|e| e.into_inner());
        let p = p.get_or_insert_with(Prices::default);
        p.asking = false;
        p.at = Some(Instant::now());
        if let Some(b) = bazaar {
            p.bazaar = b;
        }
        if let Some(n) = npc {
            p.npc = n;
        }
    });
}
#[cfg(test)]
fn fetch_prices() {}

// ---- Spotify: the media reader (Windows' media session; macOS osascript; Linux playerctl) and Album Colors ----

/// The Windows helper: every 500 ms, Spotify's media session as one JSON line {title, artist, album, durationMs,
/// positionMs, playing, art} when something changed (art: the cover shrunk to 16x16, base64 of its RGB bytes).
/// Exits when the game is gone.
#[cfg_attr(test, allow(dead_code))]
const WINDOWS_MEDIA: &str = r###"$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = New-Object System.Text.UTF8Encoding $false
function Say($o) { [Console]::Out.WriteLine(($o | ConvertTo-Json -Compress)); [Console]::Out.Flush() }
try {
  Add-Type -AssemblyName System.Runtime.WindowsRuntime
  $null = [Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager, Windows.Media.Control, ContentType = WindowsRuntime]
  $null = [Windows.Media.Control.GlobalSystemMediaTransportControlsSessionMediaProperties, Windows.Media.Control, ContentType = WindowsRuntime]
  $asTask = [System.WindowsRuntimeSystemExtensions].GetMethods() | Where-Object { $_.Name -eq 'AsTask' -and $_.IsGenericMethod -and $_.GetParameters().Count -eq 1 -and $_.GetParameters()[0].ParameterType.Name -eq 'IAsyncOperation`1' } | Select-Object -First 1
  function Await($op, [Type]$type) { $t = $asTask.MakeGenericMethod($type).Invoke($null, @($op)); if (-not $t.Wait(5000)) { throw 'timeout' }; $t.Result }
  $manager = Await ([Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager]::RequestAsync()) ([Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager])
} catch {
  Say @{ error = 'winrt' }
  exit 3
}
$canArt = $true
try {
  Add-Type -AssemblyName System.Drawing
  $streamType = [Windows.Storage.Streams.IRandomAccessStreamWithContentType, Windows.Storage.Streams, ContentType = WindowsRuntime]
  $inputType = [Windows.Storage.Streams.IInputStream, Windows.Storage.Streams, ContentType = WindowsRuntime]
  $asStream = [System.IO.WindowsRuntimeStreamExtensions].GetMethod('AsStreamForRead', [Type[]]@($inputType))
} catch { $canArt = $false }
function Art($props) {
  if (-not $canArt -or -not $props.Thumbnail) { return '' }
  $ras = Await ($props.Thumbnail.OpenReadAsync()) $streamType
  $stream = $asStream.Invoke($null, @($ras))
  try {
    $img = [System.Drawing.Image]::FromStream($stream)
    $bmp = New-Object System.Drawing.Bitmap 16, 16
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBilinear
    $g.DrawImage($img, 0, 0, 16, 16)
    $g.Dispose(); $img.Dispose()
    $bytes = New-Object byte[] 768
    for ($i = 0; $i -lt 256; $i++) { $c = $bmp.GetPixel($i % 16, [Math]::Floor($i / 16)); $bytes[$i * 3] = $c.R; $bytes[$i * 3 + 1] = $c.G; $bytes[$i * 3 + 2] = $c.B }
    $bmp.Dispose()
    return [Convert]::ToBase64String($bytes)
  } finally { $stream.Dispose() }
}
$parent = 0
[void][int]::TryParse([string]$env:ARSENAL_PARENT_PID, [ref]$parent)
$last = ''
$lastSent = [DateTime]::MinValue
$artFor = ''
$art = ''
$artTries = 0
$artAt = [DateTime]::UtcNow
$loops = 0
while ($true) {
  $loops++
  if ($parent -gt 0 -and $loops % 10 -eq 0 -and -not (Get-Process -Id $parent -ErrorAction SilentlyContinue)) { exit 0 }
  $state = [ordered]@{ title = ''; artist = ''; album = ''; durationMs = 0; positionMs = 0; playing = $false; art = '' }
  $key = 'none'
  try {
    $session = $manager.GetSessions() | Where-Object { $_.SourceAppUserModelId -like '*spotify*' } | Select-Object -First 1
    if ($session) {
      $props = Await ($session.TryGetMediaPropertiesAsync()) ([Windows.Media.Control.GlobalSystemMediaTransportControlsSessionMediaProperties])
      $timeline = $session.GetTimelineProperties()
      $playing = $session.GetPlaybackInfo().PlaybackStatus -eq 'Playing'
      $position = $timeline.Position.TotalMilliseconds
      $duration = ($timeline.EndTime - $timeline.StartTime).TotalMilliseconds
      if ($playing -and $timeline.LastUpdatedTime.Year -gt 2000) { $position += ([DateTimeOffset]::Now - $timeline.LastUpdatedTime).TotalMilliseconds }
      if ($duration -gt 0 -and $position -gt $duration) { $position = $duration }
      if ($position -lt 0) { $position = 0 }
      $state.title = [string]$props.Title
      $state.artist = [string]$props.Artist
      $state.album = [string]$props.AlbumTitle
      $state.durationMs = [long]$duration
      $state.positionMs = [long]$position
      $state.playing = [bool]$playing
      $song = '{0}|{1}|{2}' -f $state.title, $state.artist, $state.album
      if ($song -ne $artFor) { $artFor = $song; $art = ''; $artTries = 0; $artAt = [DateTime]::UtcNow }
      if ($art -eq '' -and $artTries -lt 4 -and ([DateTime]::UtcNow - $artAt).TotalSeconds -ge 1.5) {
        $artTries++
        $artAt = [DateTime]::UtcNow
        try { $art = [string](Art $props) } catch { $art = '' }
      }
      $state.art = $art
      $key = '{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}' -f $state.title, $state.artist, $state.album, $state.durationMs, $playing, $timeline.Position.Ticks, $timeline.LastUpdatedTime.Ticks, $art.Length
    }
  } catch {
    $key = 'none'
  }
  $now = [DateTime]::UtcNow
  if ($key -ne $last -or ($now - $lastSent).TotalSeconds -ge 5) {
    Say $state
    $last = $key
    $lastSent = $now
  }
  Start-Sleep -Milliseconds 500
}
"###;

/// A song as the media reader saw it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Media {
    pub title: String,
    pub artist: String,
    pub position_ms: u64,
    pub playing: bool,
    /// The cover as 16x16 RGB bytes (768), when known.
    pub art: Vec<u8>,
}

fn b64_decode(text: &str) -> Vec<u8> {
    let val = |c: u8| match c {
        b'A'..=b'Z' => Some(c - b'A'),
        b'a'..=b'z' => Some(c - b'a' + 26),
        b'0'..=b'9' => Some(c - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    };
    let mut out = Vec::new();
    let mut acc = 0u32;
    let mut bits = 0;
    for c in text.bytes().filter_map(val) {
        acc = (acc << 6) | u32::from(c);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
        }
    }
    out
}

/// One line of the Windows helper.
pub fn parse_media(line: &str) -> Option<Media> {
    let v: serde_json::Value = serde_json::from_str(line).ok()?;
    if v.get("error").is_some() {
        return None;
    }
    Some(Media {
        title: v["title"].as_str().unwrap_or("").to_owned(),
        artist: v["artist"].as_str().unwrap_or("").to_owned(),
        position_ms: v["positionMs"].as_u64().unwrap_or(0),
        playing: v["playing"].as_bool().unwrap_or(false),
        art: b64_decode(v["art"].as_str().unwrap_or("")),
    })
}

/// A song heard: the lyrics' clock follows its position; a new song fetches its lyrics, a new cover its colours.
#[cfg_attr(test, allow(dead_code))]
fn media_heard(m: Media) {
    let song = (!m.title.is_empty()).then(|| format!("{} - {}", m.artist, m.title));
    let fetch = net_more(|n| {
        let now = Instant::now();
        let changed = n.song != song;
        n.song = song.clone();
        if song.is_none() {
            if n.paused_at.is_none() {
                n.paused_at = Some(now);
            }
            return None;
        }
        n.song_start = Some(now.checked_sub(Duration::from_millis(m.position_ms)).unwrap_or(now));
        n.paused_at = (!m.playing).then_some(now);
        if changed {
            n.lyrics.clear();
        }
        (changed && n.lyrics_for != song).then(|| song.clone().unwrap_or_default())
    });
    if m.art.len() == 768 {
        let colours = album_palette(&m.art);
        ALBUM_FADE.with_lock(|a| {
            if a.target != colours {
                a.from = a.current();
                a.target = colours;
                a.at = Some(Instant::now());
            }
        });
    }
    if let Some(song) = fetch {
        fetch_lyrics(song);
    }
}

#[cfg(not(test))]
fn fetch_lyrics(song: String) {
    std::thread::spawn(move || {
        let (artist, track) = song.split_once(" - ").unwrap_or(("", &song));
        let url = format!("https://lrclib.net/api/get?artist_name={}&track_name={}", percent(artist), percent(track));
        let answer = ureq::get(&url).timeout(Duration::from_secs(15)).set("User-Agent", "Arsenal-Horizon").call().ok().and_then(|r| r.into_string().ok());
        let lines = answer.and_then(|a| serde_json::from_str::<serde_json::Value>(&a).ok())
            .and_then(|v| v["syncedLyrics"].as_str().map(parse_lrc)).unwrap_or_default();
        net_more(|n| {
            n.lyrics = lines;
            n.lyrics_for = Some(song);
        });
    });
}
#[cfg(test)]
#[allow(dead_code)]
fn fetch_lyrics(_song: String) {}

/// Starts the media reader once (Windows: the helper; elsewhere a poll every second), again after it stops.
#[cfg(not(test))]
fn media_start() {
    use std::sync::atomic::{AtomicBool, Ordering};
    static RUNNING: AtomicBool = AtomicBool::new(false);
    if RUNNING.swap(true, Ordering::SeqCst) {
        return;
    }
    std::thread::spawn(|| {
        use std::io::BufRead;
        loop {
            if cfg!(windows) {
                let mut cmd = std::process::Command::new("powershell");
                cmd.args(["-NoProfile", "-NonInteractive", "-Command", WINDOWS_MEDIA]).env("ARSENAL_PARENT_PID", std::process::id().to_string())
                    .stdin(std::process::Stdio::null()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::null());
                #[cfg(windows)]
                {
                    use std::os::windows::process::CommandExt;
                    cmd.creation_flags(0x0800_0000);
                }
                if let Ok(mut child) = cmd.spawn() {
                    if let Some(out) = child.stdout.take() {
                        for line in std::io::BufReader::new(out).lines().map_while(Result::ok) {
                            if line.contains("\"error\"") {
                                break;
                            }
                            if let Some(m) = parse_media(&line) {
                                media_heard(m);
                            }
                        }
                    }
                    let _ = child.kill();
                }
                std::thread::sleep(Duration::from_secs(60));
            } else {
                let m = if cfg!(target_os = "macos") {
                    std::process::Command::new("osascript").args(["-e", "if application \"Spotify\" is running then\ntell application \"Spotify\"\nif player state is stopped then return \"\"\nset t to current track\nreturn (name of t) & linefeed & (artist of t) & linefeed & (player position) & linefeed & (player state as text)\nend tell\nend if\nreturn \"\""])
                        .output().ok().map(|o| String::from_utf8_lossy(&o.stdout).to_string()).and_then(|t| {
                            let l: Vec<&str> = t.lines().collect();
                            (l.len() >= 4).then(|| Media { title: l[0].into(), artist: l[1].into(), position_ms: (l[2].replace(',', ".").parse::<f64>().unwrap_or(0.0) * 1000.0) as u64,
                                playing: l[3].trim() == "playing", art: Vec::new() })
                        })
                } else {
                    std::process::Command::new("playerctl").args(["-p", "spotify", "metadata", "--format", "{{title}}\t{{artist}}\t{{position}}\t{{status}}"])
                        .output().ok().map(|o| String::from_utf8_lossy(&o.stdout).to_string()).and_then(|t| {
                            let l: Vec<&str> = t.trim_end().split('\t').collect();
                            (l.len() >= 4).then(|| Media { title: l[0].into(), artist: l[1].into(), position_ms: l[2].parse::<u64>().unwrap_or(0) / 1000,
                                playing: l[3] == "Playing", art: Vec::new() })
                        })
                };
                media_heard(m.unwrap_or_default());
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    });
}
#[cfg(test)]
fn media_start() {}

/// Album Colors' colours: what they fade from and to, and since when.
#[derive(Default)]
struct AlbumFade {
    from: Option<(u32, u32)>,
    target: Option<(u32, u32)>,
    at: Option<Instant>,
}
impl AlbumFade {
    fn current(&self) -> Option<(u32, u32)> {
        let t = self.at.map_or(1.0, |a| (a.elapsed().as_secs_f32() / 1.5).min(1.0));
        match (self.from, self.target) {
            (Some(f), Some(to)) => Some((mix_rgb(f.0, to.0, t), mix_rgb(f.1, to.1, t))),
            (None, to) => to,
            (f, None) => if t >= 1.0 { None } else { f },
        }
    }
}
struct AlbumLock(std::sync::Mutex<AlbumFade>);
impl AlbumLock {
    fn with_lock<T>(&self, f: impl FnOnce(&mut AlbumFade) -> T) -> T {
        f(&mut self.0.lock().unwrap_or_else(|e| e.into_inner()))
    }
}
static ALBUM_FADE: AlbumLock = AlbumLock(std::sync::Mutex::new(AlbumFade { from: None, target: None, at: None }));

/// The album's accent and sky colours now (0xRRGGBB), fading between songs.
pub fn album_now() -> Option<(u32, u32)> {
    ALBUM_FADE.with_lock(|a| a.current())
}

/// Between two 0xRRGGBB colours.
pub fn mix_rgb(a: u32, b: u32, t: f32) -> u32 {
    mix_argb(a | 0xFF00_0000, b | 0xFF00_0000, t) & 0xFF_FFFF
}

fn rgb_hsv(c: [f32; 3]) -> (f32, f32, f32) {
    let max = c[0].max(c[1]).max(c[2]);
    let min = c[0].min(c[1]).min(c[2]);
    let d = max - min;
    let h = if d == 0.0 { 0.0 } else if max == c[0] { ((c[1] - c[2]) / d).rem_euclid(6.0) } else if max == c[1] { (c[2] - c[0]) / d + 2.0 } else { (c[0] - c[1]) / d + 4.0 } / 6.0;
    (h.rem_euclid(1.0), if max == 0.0 { 0.0 } else { d / max }, max)
}

fn hsl_rgb(h: f32, s: f32, l: f32) -> u32 {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0).rem_euclid(2.0) - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = match (h * 6.0) as u32 % 6 { 0 => (c, x, 0.0), 1 => (x, c, 0.0), 2 => (0.0, c, x), 3 => (0.0, x, c), 4 => (x, 0.0, c), _ => (c, 0.0, x) };
    let to = |v: f32| (((v + m) * 255.0).round().clamp(0.0, 255.0)) as u32;
    (to(r) << 16) | (to(g) << 8) | to(b)
}

fn rgb_hsl(rgb: u32) -> (f32, f32, f32) {
    let c = [((rgb >> 16) & 255) as f32 / 255.0, ((rgb >> 8) & 255) as f32 / 255.0, (rgb & 255) as f32 / 255.0];
    let max = c[0].max(c[1]).max(c[2]);
    let min = c[0].min(c[1]).min(c[2]);
    let l = (max + min) / 2.0;
    let d = max - min;
    let s = if d == 0.0 { 0.0 } else { d / (1.0 - (2.0 * l - 1.0).abs()) };
    (rgb_hsv(c).0, s, l)
}

fn clamp_hsl(rgb: u32, s: (f32, f32), l: (f32, f32)) -> u32 {
    let (h, sat, light) = rgb_hsl(rgb);
    hsl_rgb(h, sat.clamp(s.0, s.1), light.clamp(l.0, l.1))
}

/// The album's colours from its 16x16 cover: (accent, sky), from the strongest hue (none for a grey cover).
pub fn album_palette(rgb: &[u8]) -> Option<(u32, u32)> {
    let px: Vec<[f32; 3]> = rgb.chunks(3).filter(|c| c.len() == 3).map(|c| [c[0] as f32 / 255.0, c[1] as f32 / 255.0, c[2] as f32 / 255.0]).collect();
    let mut bins = [0f32; 24];
    let mut total = 0.0;
    let info: Vec<(usize, f32)> = px.iter().map(|c| {
        let (h, s, v) = rgb_hsv(*c);
        let w = if s < 0.18 || v < 0.16 { 0.0 } else { s * s * v };
        let b = ((h * 24.0) as usize).min(23);
        bins[b] += w;
        total += w;
        (b, w)
    }).collect();
    if px.is_empty() || (total / px.len() as f32) < 0.06 {
        return None;
    }
    let score = |b: usize| bins[b] + 0.5 * (bins[(b + 23) % 24] + bins[(b + 1) % 24]);
    let best = (0..24).max_by(|a, b| score(*a).total_cmp(&score(*b)))?;
    let (mut sum, mut wsum) = ([0f32; 3], 0f32);
    for (c, (b, w)) in px.iter().zip(&info) {
        let d = (*b as i32 - best as i32).rem_euclid(24);
        if d <= 1 || d >= 23 {
            for k in 0..3 {
                sum[k] += c[k] * w;
            }
            wsum += w;
        }
    }
    if wsum <= 0.0 {
        return None;
    }
    let dom = sum.map(|v| ((v / wsum) * 255.0).round().clamp(0.0, 255.0) as u32);
    let dom = (dom[0] << 16) | (dom[1] << 8) | dom[2];
    Some((clamp_hsl(dom, (0.45, 0.9), (0.62, 0.78)), clamp_hsl(dom, (0.22, 0.55), (0.56, 0.70))))
}

// ---- Cloud Backup ----

#[derive(Default)]
struct Cloud {
    /// Save (true) or restore (false), waiting for the backup to load.
    asked: Option<bool>,
    waiting: bool,
    last_saved: String,
    saved_at: Option<Instant>,
}

/// What a backup answer brought (filled by the network thread).
#[derive(Default)]
struct CloudNet {
    /// The stored config (None: no backup yet), or the reason it failed.
    loaded: Option<Result<Option<String>, String>>,
    saved: Option<Result<(), String>>,
}
static CLOUD_NET: std::sync::Mutex<CloudNet> = std::sync::Mutex::new(CloudNet { loaded: None, saved: None });

/// Your Horizon setup as it goes into the backup.
pub fn horizon_section(raw: &Settings) -> serde_json::Value {
    let settings: serde_json::Map<String, serde_json::Value> = raw.stored().into_iter().map(|(k, v)| (k.to_owned(), v.into())).collect();
    serde_json::json!({ "version": VERSION, "settings": settings })
}

/// The backup with your Horizon setup put in (the Java mod's setup stays as it was).
pub fn merged_backup(existing: Option<&str>, raw: &Settings) -> String {
    let mut root = existing.and_then(|e| serde_json::from_str::<serde_json::Value>(e).ok()).filter(|v| v.is_object())
        .unwrap_or_else(|| serde_json::json!({ "configVersion": 1, "modules": {} }));
    root["horizon"] = horizon_section(raw);
    serde_json::to_string_pretty(&root).unwrap_or_default()
}

/// The settings a backup restores: its Horizon setup, else the Java setup's modules switched on or off.
pub fn restored_from(config: &str) -> Vec<(String, String)> {
    let Ok(root) = serde_json::from_str::<serde_json::Value>(config) else { return Vec::new() };
    if let Some(map) = root["horizon"]["settings"].as_object() {
        return map.iter().filter_map(|(k, v)| v.as_str().map(|v| (k.clone(), v.to_owned())))
            .filter(|(k, _)| FIELDS.iter().any(|f| f.key == k) || k == "hud_layout" || k == "favorites" || k == "browse_theme" || k == "browse_crosshair")
            .collect();
    }
    let Some(modules) = root["modules"].as_object() else { return Vec::new() };
    modules.iter().filter_map(|(id, m)| {
        let on = m["enabled"].as_bool()?;
        let (f, _) = on_horizon(id)?;
        Some((FIELDS[f].key.to_owned(), if on { "true" } else { "false" }.to_owned()))
    }).collect()
}

/// `/arsenal cloud save` or `restore`.
pub fn cloud_ask(save: bool) {
    STATE.with_borrow_mut(|s| s.more2.cloud.asked = Some(save));
}

/// What Cloud Backup asks of the game this tick.
#[derive(Debug, Default, PartialEq)]
pub struct CloudOut {
    pub say: Option<String>,
    /// Settings to restore (key, value).
    pub restore: Vec<(String, String)>,
}

/// Each tick: asks (signed with `sign`, as the Java mod does) for the backup, then saves or restores; saves by itself
/// 5 minutes after the setup changed.
pub fn cloud_tick(settings: &Settings, raw: &Settings, sign: &dyn Fn(&str, &[String]) -> Option<String>) -> CloudOut {
    let mut out = CloudOut::default();
    let net = {
        let mut n = CLOUD_NET.lock().unwrap_or_else(|e| e.into_inner());
        (n.loaded.take(), n.saved.take())
    };
    let section = horizon_section(raw).to_string();
    STATE.with_borrow_mut(|s| {
        let c = &mut s.more2.cloud;
        if let Some(saved) = net.1 {
            c.waiting = false;
            match saved {
                Ok(()) => {
                    c.last_saved = section.clone();
                    if c.asked.take().is_some() {
                        out.say = Some("Your Horizon setup is in your Arsenal cloud backup.".into());
                    }
                }
                Err(why) => out.say = Some(format!("Cloud backup failed ({why}).")),
            }
            c.saved_at = Some(Instant::now());
        }
        if let Some(loaded) = net.0 {
            c.waiting = false;
            let save = c.asked.unwrap_or(true);
            match loaded {
                Err(why) => {
                    c.asked = None;
                    out.say = Some(format!("Cloud backup failed ({why})."));
                    c.saved_at = Some(Instant::now());
                }
                Ok(existing) if save => {
                    let config = merged_backup(existing.as_deref(), raw);
                    match sign("arsenal-config-save-v1", &[sha256_hex(config.as_bytes())]) {
                        Some(body) => {
                            c.waiting = true;
                            cloud_post("/v1/config/save", body, Some(config), false);
                        }
                        None => out.say = Some("Cloud backup needs a Minecraft account (offline accounts can't back up).".into()),
                    }
                }
                Ok(Some(config)) => {
                    c.asked = None;
                    out.restore = restored_from(&config);
                    out.say = Some(if out.restore.is_empty() { "Your cloud backup has nothing Horizon can use.".into() } else { "Restored your setup from the Arsenal cloud.".into() });
                }
                Ok(None) => {
                    c.asked = None;
                    out.say = Some("There's no cloud backup yet: /arsenal cloud save makes one.".into());
                }
            }
        }
        if c.waiting {
            return;
        }
        let auto = settings.on(CLOUD) && c.asked.is_none() && !c.last_saved.is_empty() && c.last_saved != section
            && c.saved_at.is_none_or(|t| t.elapsed() > Duration::from_secs(300));
        if c.last_saved.is_empty() {
            c.last_saved = section.clone();
        }
        if c.asked.is_some() || auto {
            if auto {
                c.asked = None;
            }
            match sign("arsenal-config-load-v1", &[]) {
                Some(body) => {
                    c.waiting = true;
                    if auto {
                        c.asked = None;
                    }
                    cloud_post("/v1/config/load", body, None, true);
                }
                None => {
                    if c.asked.take().is_some() {
                        out.say = Some("Cloud backup needs a Minecraft account (offline accounts can't back up).".into());
                    }
                    c.saved_at = Some(Instant::now());
                }
            }
        }
    });
    out
}

#[cfg(not(test))]
fn cloud_post(path: &'static str, signed: String, config: Option<String>, load: bool) {
    std::thread::spawn(move || {
        let mut body: serde_json::Value = serde_json::from_str(&signed).unwrap_or_default();
        if let Some(c) = config {
            body["config"] = c.into();
        }
        let answer = ureq::post(&format!("{WEBSITE}{path}")).timeout(Duration::from_secs(20)).set("User-Agent", "Arsenal-Horizon")
            .set("Content-Type", "application/json").send_string(&body.to_string());
        let mut n = CLOUD_NET.lock().unwrap_or_else(|e| e.into_inner());
        let reason = |text: Option<String>, code: u16| text.and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
            .and_then(|v| v["reason"].as_str().map(str::to_owned)).unwrap_or_else(|| format!("HTTP {code}"));
        if load {
            n.loaded = Some(match answer {
                Ok(r) => r.into_string().ok().and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
                    .and_then(|v| v["config"].as_str().map(|c| Some(c.to_owned()))).ok_or_else(|| "bad answer".to_owned()),
                Err(ureq::Error::Status(404, _)) => Ok(None),
                Err(ureq::Error::Status(code, r)) => Err(reason(r.into_string().ok(), code)),
                Err(_) => Err("Arsenal API not reachable".into()),
            });
        } else {
            n.saved = Some(match answer {
                Ok(_) => Ok(()),
                Err(ureq::Error::Status(code, r)) => Err(reason(r.into_string().ok(), code)),
                Err(_) => Err("Arsenal API not reachable".into()),
            });
        }
    });
}
#[cfg(test)]
fn cloud_post(_path: &'static str, _signed: String, _config: Option<String>, _load: bool) {}


// ---- 1.15: keys, profiles, share codes, servers, risky and new modules, HUD themes ----

/// "a=b;c=d" as pairs.
pub fn pairs_of(text: Option<&str>) -> Vec<(String, String)> {
    text.unwrap_or("").split(';').filter_map(|p| p.split_once('=')).map(|(a, b)| (a.trim().to_owned(), b.trim().to_owned()))
        .filter(|(a, _)| !a.is_empty()).collect()
}

/// A JSON object of strings as pairs.
pub fn json_pairs(text: Option<&str>) -> Vec<(String, String)> {
    text.and_then(|t| serde_json::from_str::<serde_json::Value>(t).ok()).and_then(|v| v.as_object().map(|o| {
        o.iter().filter_map(|(k, v)| v.as_str().map(|v| (k.clone(), v.to_owned()))).collect()
    })).unwrap_or_default()
}

pub fn json_text(pairs: &[(String, String)]) -> String {
    serde_json::Value::Object(pairs.iter().map(|(k, v)| (k.clone(), v.clone().into())).collect()).to_string()
}

/// A server address as the Java mod keeps its per-server things: "EU.Play.Example.NET:25565" is "example.net".
pub fn server_of(address: &str) -> String {
    let mut a = address.trim().to_lowercase();
    if a.ends_with('.') {
        a.pop();
    }
    if let Some(stripped) = a.strip_suffix(":25565") {
        a = stripped.to_owned();
    }
    if a.contains(':') || a.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return a;
    }
    let labels: Vec<&str> = a.split('.').collect();
    let n = labels.len();
    if n >= 3 && labels[n - 1].len() == 2 && ["co", "com", "net", "org", "gov", "ac", "edu"].contains(&labels[n - 2]) {
        return labels[n - 3..].join(".");
    }
    labels[n.saturating_sub(2)..].join(".")
}

thread_local! {
    /// The server you're on (as `server_of` keeps it), for its own HUD layout.
    static SERVER: RefCell<Option<String>> = const { RefCell::new(None) };
}

pub fn current_server() -> Option<String> {
    SERVER.with_borrow(|s| s.clone())
}

/// Each tick: the server you're on. Joining one with its own HUD layout puts it in use; leaving puts the usual one back.
pub fn set_server(address: Option<&str>) {
    let now = address.map(server_of);
    let before = SERVER.with_borrow(|s| s.clone());
    if before == now {
        return;
    }
    STATE.with_borrow_mut(|st| {
        if let Some(s) = st.settings.as_mut() {
            match before.as_ref().and_then(|k| s.server_layouts.iter_mut().find(|e| &e.0 == k)) {
                Some(entry) => entry.1 = s.layout.clone(),
                None => s.main_layout = s.layout.clone(),
            }
            s.layout = now.as_ref().and_then(|k| s.server_layouts.iter().find(|e| &e.0 == k).map(|e| e.1.clone())).unwrap_or_else(|| s.main_layout.clone());
        }
    });
    SERVER.with_borrow_mut(|s| *s = now);
}

/// What some servers may not allow (Hide risky leaves them out of the menu; their pages say so).
pub const RISKY: &[&str] = &["crystal_optimizer", "crystal_preview", "crystal_helper", "crystal_damage_preview", "anchor_helper", "obsidian_guard",
    "target_hud", "nametag_health", "nametag_ping", "enemy_armor_alert", "pearl_timer", "eating_warning", "target_low_hp", "damage_numbers",
    "totem_pop_counter", "totem_pop_alerts", "shield_helper", "shield_break_sound", "combat_hitboxes", "auto_hitbox", "dynamic_crosshair",
    "combo_reach", "crit_indicator", "sprint_indicator", "mace_aim", "attack_bar", "fullbright", "no_fog", "camera_tweaks", "weather_time", "free_look"];

pub fn risky(id: &str) -> bool {
    RISKY.contains(&id)
}

/// The modules new on Horizon (the version they came in): the New row lists the last two versions'.
pub const ADDED: &[(&str, &str)] = &[
    ("blade_settings", "1.15"), ("hypixel_skyblock", "1.14"), ("album_colors", "1.14"), ("cloud_backup", "1.14"), ("inventory_cursor_speed", "1.14"),
    ("hotbar_style", "1.13"), ("smooth_hotbar", "1.13"), ("inventory_style", "1.13"), ("smooth_inventory", "1.13"), ("old_animations", "1.13"),
    ("swing_animation", "1.13"), ("hand_tint", "1.13"), ("small_crystals", "1.13"), ("big_items", "1.13"), ("armor_color", "1.13"),
    ("nametag_style", "1.13"), ("free_look", "1.13"), ("block_break_animation", "1.13"), ("place_animation", "1.13"), ("slick_floors", "1.13"),
    ("tab_ping", "1.13"), ("chat_mentions", "1.13"), ("chat_tweaks", "1.13"), ("chat_copy", "1.13"), ("name_hider", "1.13"), ("sound_tweaks", "1.13"),
    ("sprint_indicator", "1.13"), ("obsidian_guard", "1.13"), ("crystal_helper", "1.13"), ("screenshot_clipboard", "1.13"), ("quick_menu", "1.13"),
    ("performance_mode", "1.13"), ("arsenal_messages", "1.13"), ("party_hud", "1.13"), ("wardrobe_kill_effects", "1.13"), ("usage_stats", "1.13"),
    ("auto_reconnect", "1.13"), ("crash_reports", "1.13"), ("update_notice", "1.13"), ("server_notes", "1.13"), ("server_profiles", "1.13"),
    ("spotify_lyrics", "1.13"),
];

/// The sidebar's New row (after Favorites and the categories).
pub const NEW_TAB: usize = CATEGORIES.len() + 1;

pub fn new_modules() -> Vec<usize> {
    let minor = |v: &str| v.split('.').nth(1).and_then(|m| m.parse::<i32>().ok()).unwrap_or(0);
    let now = minor(VERSION);
    let mut out: Vec<(i32, usize)> = ADDED.iter().filter(|(_, v)| (now - 2 + 1..=now).contains(&minor(v)))
        .filter_map(|(id, v)| JAVA_MODULES.iter().position(|j| j.id == *id).map(|i| (-minor(v), i))).collect();
    out.sort_by_key(|(v, i)| (*v, JAVA_MODULES[*i].name.to_ascii_lowercase()));
    out.into_iter().map(|e| e.1).collect()
}

/// "key.keyboard.left.shift" as "Left Shift".
pub fn key_name(key: &str) -> String {
    let k = key.trim_start_matches("key.keyboard.").trim_start_matches("key.mouse.");
    k.split('.').map(|w| {
        let mut c = w.chars();
        c.next().map_or(String::new(), |f| f.to_uppercase().collect::<String>() + c.as_str())
    }).collect::<Vec<_>>().join(" ")
}

/// A key went down in game: the module it switches (index in `FIELDS`) and its name, if one has it.
pub fn module_key(raw: &Settings, key: &str) -> Option<(usize, &'static str)> {
    let (id, _) = raw.keys.iter().find(|(_, k)| k == key)?;
    let j = JAVA_MODULES.iter().find(|j| j.id == id)?;
    on_horizon(j.id).map(|(f, _)| (f, j.name))
}

/// The other modules on the same key.
pub fn key_clash(s: &Settings, id: &str) -> Vec<&'static str> {
    let Some(key) = s.keys.iter().find(|(i, _)| i == id).map(|e| e.1.clone()) else { return Vec::new() };
    s.keys.iter().filter(|(i, k)| i != id && *k == key).filter_map(|(i, _)| JAVA_MODULES.iter().find(|j| j.id == i).map(|j| j.name)).collect()
}

/// The HUD themes an element can take (as the Java mod's): name, panel (0 none), text, accent.
pub const HUD_THEMES: &[(&str, u32, u32, u32)] = &[
    ("CLASSIC", 0, 0xFFFFFF, 0xFFAA00), ("MINIMAL", 0, 0xEEEEEE, 0xFFFFFF), ("DARK", 0xA010_1014, 0xFFFFFF, 0xFFAA00),
    ("GLASS", 0x40FF_FFFF, 0xFFFFFF, 0xCCE6FF), ("ARSENAL", 0xB014_0B22, 0xEEE6FF, 0x9D5CFF), ("MIDNIGHT", 0xC00B_1020, 0xDDE3FF, 0x5A7BFF),
    ("NEON", 0xB005_0510, 0xE0FFFF, 0x00F0FF), ("CRIMSON", 0xB020_0608, 0xFFE5E5, 0xFF3344), ("OCEAN", 0xA006_2038, 0xE0F4FF, 0x3FC1FF),
    ("FOREST", 0xA00E_1F12, 0xE6FFE9, 0x55D16E), ("SUNSET", 0xB03A_1240, 0xFFF0E6, 0xFF8A3D), ("LAVA", 0xB030_0800, 0xFFE6CC, 0xFF6A00),
    ("ICE", 0x8030_607A, 0xF0FCFF, 0xA8E8FF), ("VAPOR", 0xB02A_0A4A, 0xFFE9FB, 0x01CDFE), ("ROYAL", 0xB01A_0F33, 0xF5EBD0, 0xD4AF37),
    ("GOLD", 0xB020_1A08, 0xFFF6D5, 0xFFD700), ("TERMINAL", 0xC000_0000, 0x33FF66, 0x33FF66), ("LIGHT", 0xD0F4_F5F9, 0x161A25, 0xE09000),
    ("CANDY", 0xC0FF_E0F0, 0x5A1840, 0xFF4FA3), ("PASTEL", 0xB0F7_EFFF, 0x4A3B66, 0xB48CFF),
];

/// HUD element `id` in its own theme: the panel's background, the text and the accent swapped for the theme's.
fn theme_element(settings: &Settings, shapes: &mut [Shape], id: &str) {
    let Some(theme) = settings.hud_themes.iter().find(|(e, _)| e == id).and_then(|(_, t)| HUD_THEMES.iter().find(|x| x.0 == t)) else { return };
    let (bg, text) = settings.panel_colors();
    let accent = settings.accent();
    let swap = |rgb: u32| if rgb & 0xFF_FFFF == accent { theme.3 } else if rgb & 0xFF_FFFF == text & 0xFF_FFFF || rgb & 0xFF_FFFF == WHITE { theme.2 } else { rgb & 0xFF_FFFF };
    for sh in shapes.iter_mut() {
        match sh {
            Shape::Fill(_, c) if *c == bg || *c == BACKGROUND => *c = theme.1,
            Shape::Fill(_, c) if *c & 0xFF_FFFF == accent => *c = (*c & 0xFF00_0000) | theme.3,
            Shape::Text(_, _, rgb, _) => *rgb = swap(*rgb),
            Shape::Styled(_, _, c, _, _) => *c = (*c & 0xFF00_0000) | swap(*c),
            _ => {}
        }
    }
}

// ---- Share codes: "A2:" + base64url(raw deflate(JSON)), as the Java mod's ----

fn b64url(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut out = String::new();
    for c in data.chunks(3) {
        let n = (u32::from(c[0]) << 16) | (u32::from(*c.get(1).unwrap_or(&0)) << 8) | u32::from(*c.get(2).unwrap_or(&0));
        out.push(T[(n >> 18) as usize & 63] as char);
        out.push(T[(n >> 12) as usize & 63] as char);
        if c.len() > 1 {
            out.push(T[(n >> 6) as usize & 63] as char);
        }
        if c.len() > 2 {
            out.push(T[n as usize & 63] as char);
        }
    }
    out
}

/// Your setup as a share code: what differs from the defaults (the Java mod reads its module switches; another
/// Horizon all of it). Personal things (friends, profiles, notes, keys to servers) stay out.
pub fn share_code(raw: &Settings) -> String {
    let defaults = Settings::default().stored();
    let personal = ["friend_list", "profiles", "auto_profiles", "server_huds"];
    let changed: serde_json::Map<String, serde_json::Value> = raw.stored().into_iter()
        .filter(|(k, v)| !personal.contains(k) && !k.starts_with("note_") && defaults.iter().find(|d| d.0 == *k).is_none_or(|d| d.1 != *v))
        .map(|(k, v)| (k.to_owned(), v.into())).collect();
    let mut modules = serde_json::Map::new();
    for (id, field, _) in ON_HORIZON {
        let on = raw.v[*field] == 1;
        let fav = raw.favorites.iter().any(|f| f == id);
        let key = raw.keys.iter().find(|(i, _)| i == id).map(|e| e.1.clone());
        if on != (Settings::default().v[*field] == 1) || fav || key.is_some() {
            let mut m = serde_json::Map::new();
            m.insert("enabled".into(), on.into());
            if fav {
                m.insert("favorite".into(), true.into());
            }
            if let Some(k) = key {
                m.insert("key".into(), k.into());
            }
            modules.insert((*id).to_owned(), serde_json::Value::Object(m));
        }
    }
    let json = serde_json::json!({ "configVersion": 1, "onlyChanges": true, "modules": modules, "horizon": { "version": VERSION, "settings": changed } });
    format!("A2:{}", b64url(&miniz_oxide::deflate::compress_to_vec(json.to_string().as_bytes(), 9)))
}

/// A share code (or the JSON in it) read back: the settings it sets, None when it isn't one.
pub fn read_share_code(code: &str) -> Option<Vec<(String, String)>> {
    let code = code.trim();
    let json = if let Some(body) = code.strip_prefix("A2:") {
        let data = b64_decode(&body.replace('-', "+").replace('_', "/"));
        String::from_utf8(miniz_oxide::inflate::decompress_to_vec_with_limit(&data, 1_000_000).ok()?).ok()?
    } else if code.starts_with('{') {
        code.to_owned()
    } else {
        return None;
    };
    let root: serde_json::Value = serde_json::from_str(&json).ok()?;
    root.get("modules")?;
    // Every setting back to its default, then the code's.
    let mut out: Vec<(String, String)> = Settings::default().stored().into_iter()
        .filter(|(k, _)| !["friend_list", "profiles", "auto_profiles", "server_huds", "hud_layout"].contains(k)).map(|(k, v)| (k.to_owned(), v)).collect();
    let mut set = |k: String, v: String| {
        if let Some(e) = out.iter_mut().find(|e| e.0 == k) {
            e.1 = v;
        } else {
            out.push((k, v));
        }
    };
    if let Some(map) = root["horizon"]["settings"].as_object() {
        for (k, v) in map {
            if let Some(v) = v.as_str() {
                set(k.clone(), v.to_owned());
            }
        }
    } else if let Some(modules) = root["modules"].as_object() {
        let mut favorites = Vec::new();
        let mut keys = Vec::new();
        for (id, m) in modules {
            if let (Some(on), Some((f, _))) = (m["enabled"].as_bool(), on_horizon(id)) {
                set(FIELDS[f].key.to_owned(), if on { "true" } else { "false" }.to_owned());
            }
            if m["favorite"] == true {
                favorites.push(id.clone());
            }
            if let Some(k) = m["key"].as_str().filter(|k| k.starts_with("key.")) {
                keys.push(format!("{id}={k}"));
            }
        }
        set("favorites".into(), favorites.join(","));
        set("module_keys".into(), keys.join(";"));
    }
    Some(out)
}

/// What the clipboard is asked for: text to copy, or to read a code to paste.
#[derive(Clone, Debug, PartialEq)]
pub enum Clip {
    Copy(String),
    Paste,
}
thread_local! {
    static CLIP: RefCell<Option<Clip>> = const { RefCell::new(None) };
}
pub fn take_clip() -> Option<Clip> {
    CLIP.with_borrow_mut(|c| c.take())
}

/// The clipboard's text when Paste code asked: the settings to keep, and what to say.
pub fn pasted(text: &str) -> (Option<Vec<(String, String)>>, String) {
    match read_share_code(text) {
        Some(pairs) => (Some(pairs), "Loaded the setup from the share code.".into()),
        None => (None, "The clipboard has no Arsenal share code (they start with A2:).".into()),
    }
}

thread_local! {
    static AUTO_FOR: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Server Profiles: the profile to load on joining server `key` (once per server), when one is set to Auto there.
pub fn joined_profile(raw: &Settings, key: &str) -> Option<String> {
    let first = AUTO_FOR.with_borrow_mut(|a| (a.as_deref() != Some(key)).then(|| *a = Some(key.to_owned())).is_some());
    if !first || !raw.on(PROFILES) {
        return None;
    }
    raw.auto_profiles.iter().find(|(k, _)| k == key).map(|e| e.1.clone())
}

/// A profile loaded: its settings on top, your profiles, server choices and friends kept.
pub fn load_profile(raw: &Settings, name: &str) -> Option<Settings> {
    let (_, pairs) = raw.profiles.iter().find(|(n, _)| n == name)?;
    let mut s = Settings::read(|k| pairs.iter().find(|(key, _)| key == k).map(|(_, v)| v.clone()));
    s.profiles = raw.profiles.clone();
    s.auto_profiles = raw.auto_profiles.clone();
    s.server_layouts = raw.server_layouts.clone();
    s.main_layout = s.layout.clone();
    if let Some(l) = current_server().and_then(|k| s.server_layouts.iter().find(|e| e.0 == k).map(|e| e.1.clone())) {
        s.layout = l;
    }
    Some(s)
}

/// The setup as a profile keeps it (without the profiles themselves).
pub fn profile_now(raw: &Settings) -> Vec<(String, String)> {
    raw.stored().into_iter().filter(|(k, _)| !["profiles", "auto_profiles", "server_huds"].contains(k)).map(|(k, v)| (k.to_owned(), v)).collect()
}


// ---- 1.15: Arsenal capes and Wardrobe blades on players ----

/// The Wardrobe's blades (their models are `arsenal:blade_<id>` in Arsenal's cosmetics pack).
pub const BLADE_IDS: &[&str] = &["karambit", "butterfly_knife", "bayonet", "talon", "katana", "kunai", "scythe_knife", "tanto", "shiv", "crystal_shard",
    "sickle", "storm_blade", "bowie_knife", "reapers_edge", "untouchable_blade", "cleaver", "machete", "dagger", "cutlass", "falchion", "kris",
    "bone_knife", "obsidian_dagger", "stiletto", "ice_blade", "ember_blade", "gut_hook", "champions_sabre", "veterans_knife", "shatterfang",
    "viper_fang", "moonlight_saber", "glacier_edge", "bloodmoon_cleaver", "comet_dagger", "golden_katana"];

/// The capes Arsenal has (their pictures are in this repository's cosmetics/capes).
pub const CAPE_IDS: &[&str] = &["arsenal", "owner", "ember", "frost", "emerald", "crystal", "void", "mace", "totem", "sakura", "tester", "dragon",
    "netherite", "galaxy", "lava", "rainbow", "aurora", "champion", "kingslayer", "bug_hunter", "ocean", "phoenix", "blood_moon", "thunder",
    "enderman", "creeper", "warden", "cherry_grove", "diamond", "veteran", "crystal_king", "fresh_start", "silver_skull", "clock", "comeback",
    "streak_flame", "monthly_regular", "grinder", "duelist", "totem_breaker", "clutch", "globetrotter", "collector", "haunted", "winter_lights",
    "sunset", "love_letter", "obsidian", "amethyst", "pulse", "warlord", "ironclad", "vip_cape", "supporter_cape"];

/// A blade's item model.
pub fn blade_model(id: &str) -> Option<String> {
    BLADE_IDS.contains(&id).then(|| format!("arsenal:blade_{}", if id == "butterfly_knife" { "butterfly" } else { id }))
}

/// What a player wears that Horizon draws: (cape, blade).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Wear {
    pub cape: Option<String>,
    pub blade: Option<String>,
    pub hat: Option<String>,
    pub accessory: Option<String>,
    pub pet: Option<String>,
}

/// What the player with `uuid` wears, as the Wardrobe lookups last said.
pub fn worn(uuid: u128) -> Option<Wear> {
    wardrobe(|w| w.worn.get(&uuid).cloned())
}

fn cosmetic_id(v: &serde_json::Value) -> Option<String> {
    v.as_str().filter(|id| !id.is_empty() && id.len() < 64 && id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')).map(str::to_owned)
}

#[derive(Default)]
struct Wardrobe {
    worn: std::collections::HashMap<u128, Wear>,
    own_asked: Option<Instant>,
    others_asked: Option<Instant>,
    capes_asked: std::collections::HashMap<String, Instant>,
    pack_asked: Option<Instant>,
    pack_ready: bool,
    inspect: Option<Instant>,
}
static WARDROBE: std::sync::Mutex<Option<Wardrobe>> = std::sync::Mutex::new(None);
fn wardrobe<T>(f: impl FnOnce(&mut Wardrobe) -> T) -> T {
    f(WARDROBE.lock().unwrap_or_else(|e| e.into_inner()).get_or_insert_with(Wardrobe::default))
}

/// The website's answer for your own cosmetics: what you wear.
pub fn own_wear(json: &str) -> Wear {
    let v: serde_json::Value = serde_json::from_str(json).unwrap_or_default();
    let pick = |k: &str, known: &dyn Fn(&str) -> bool| v[k].as_str().filter(|id| known(id)).map(str::to_owned);
    Wear {
        cape: pick("equipped", &|id| CAPE_IDS.contains(&id)),
        blade: pick("equippedBlade", &|id| BLADE_IDS.contains(&id)),
        hat: cosmetic_id(&v["equippedHat"]),
        accessory: cosmetic_id(&v["equippedAccessory"]),
        pet: cosmetic_id(&v["equippedPet"]),
    }
}

/// The lookup's answer for other players (by account hash).
pub fn others_wear(json: &str, hashes: &[(u128, String)]) -> Vec<(u128, Wear)> {
    let v: serde_json::Value = serde_json::from_str(json).unwrap_or_default();
    hashes.iter().map(|(uuid, h)| {
        let cape = v["capes"][h].as_str().filter(|id| CAPE_IDS.contains(id)).map(str::to_owned);
        let blade = v["blades"][h].as_str().filter(|id| BLADE_IDS.contains(id)).map(str::to_owned);
        (*uuid, Wear { cape, blade, hat: cosmetic_id(&v["hats"][h]), accessory: cosmetic_id(&v["accessories"][h]), pet: cosmetic_id(&v["pets"][h]) })
    }).collect()
}

/// What tick.rs puts into Horizon for cosmetics.
#[derive(Debug, Default, PartialEq)]
pub struct Dressing {
    /// Each player's cape picture (a file here) and blade model.
    pub capes: Vec<(u128, std::path::PathBuf)>,
    pub blades: Vec<(u128, String)>,
    /// Arsenal's cosmetics pack is downloaded and should be on.
    pub pack: bool,
}

/// Each tick: asks the website what you (every 5 minutes) and the Arsenal users around (every 20 seconds) wear,
/// fetches the cape pictures worn and Arsenal's cosmetics pack once, and says what to draw.
pub fn dressing(settings: &Settings, own: Option<u128>, data: &std::path::Path, packs: &std::path::Path) -> Dressing {
    let mut out = Dressing::default();
    if !settings.on(COSMETICS) && !settings.on(BLADES) {
        return out;
    }
    let users: Vec<u128> = if settings.on(COSMETICS) && settings.on(USERS) { net(|n| n.users.iter().copied().collect()) } else { Vec::new() };
    wardrobe(|w| {
        let now = Instant::now();
        if let Some(own) = own {
            if w.own_asked.is_none_or(|t| t.elapsed() > Duration::from_secs(300)) {
                w.own_asked = Some(now);
                ask_own(own);
            }
        }
        let others: Vec<u128> = users.into_iter().filter(|u| Some(*u) != own).take(100).collect();
        if !others.is_empty() && w.others_asked.is_none_or(|t| t.elapsed() > Duration::from_secs(20)) {
            w.others_asked = Some(now);
            ask_others(others);
        }
        let pack = packs.join("arsenal.zip");
        let marker = data.join("cosmetics-pack.txt");
        let current = std::fs::read_to_string(&marker).ok().is_some_and(|v| v.trim() == VERSION) && pack.is_file();
        if current {
            w.pack_ready = true;
        } else if w.pack_asked.is_none_or(|t| t.elapsed() > Duration::from_secs(600)) {
            w.pack_asked = Some(now);
            fetch_pack(pack, marker);
        }
        out.pack = w.pack_ready;
        for (uuid, wear) in &w.worn {
            if settings.on(BLADES) || settings.on(COSMETICS) {
                if let Some(model) = wear.blade.as_deref().and_then(blade_model) {
                    if Some(*uuid) == own || settings.on(COSMETICS) {
                        out.blades.push((*uuid, model));
                    }
                }
            }
            if let (true, Some(cape)) = (settings.on(COSMETICS), &wear.cape) {
                let file = data.join("capes").join(format!("{cape}.png"));
                if file.is_file() {
                    out.capes.push((*uuid, file));
                } else if w.capes_asked.get(cape).is_none_or(|t| t.elapsed() > Duration::from_secs(120)) {
                    w.capes_asked.insert(cape.clone(), now);
                    fetch_cape(cape.clone(), file);
                }
            }
        }
    });
    out.capes.sort();
    out.blades.sort();
    out
}

#[cfg(not(test))]
fn ask_own(own: u128) {
    std::thread::spawn(move || {
        let url = format!("{WEBSITE}/v1/cosmetics?user={}", user_hash(own));
        let answer = ureq::get(&url).timeout(Duration::from_secs(15)).set("User-Agent", "Arsenal-Horizon").call().ok().and_then(|r| r.into_string().ok());
        if let Some(a) = answer {
            let wear = own_wear(&a);
            wardrobe(|w| {
                w.worn.insert(own, wear);
            });
        }
    });
}
#[cfg(test)]
fn ask_own(_own: u128) {}

#[cfg(not(test))]
fn ask_others(others: Vec<u128>) {
    std::thread::spawn(move || {
        let hashes: Vec<(u128, String)> = others.iter().map(|u| (*u, user_hash(*u))).collect();
        let body = serde_json::json!({ "hashes": hashes.iter().map(|h| h.1.clone()).collect::<Vec<_>>() }).to_string();
        let answer = ureq::post(&format!("{WEBSITE}/v1/cosmetics/lookup")).timeout(Duration::from_secs(15)).set("User-Agent", "Arsenal-Horizon")
            .set("Content-Type", "application/json").send_string(&body).ok().and_then(|r| r.into_string().ok());
        if let Some(a) = answer {
            let found = others_wear(&a, &hashes);
            wardrobe(|w| {
                for (uuid, wear) in found {
                    w.worn.insert(uuid, wear);
                }
            });
        }
    });
}
#[cfg(test)]
fn ask_others(_others: Vec<u128>) {}

/// Downloads `url` to `file` (through a part file, so a half download is never used).
#[cfg(not(test))]
fn download_to(url: &str, file: &std::path::Path, max: u64) -> bool {
    use std::io::Read;
    let Ok(r) = ureq::get(url).timeout(Duration::from_secs(60)).set("User-Agent", "Arsenal-Horizon").call() else { return false };
    let mut bytes = Vec::new();
    if r.into_reader().take(max).read_to_end(&mut bytes).is_err() || bytes.is_empty() {
        return false;
    }
    let part = file.with_extension("part");
    if let Some(dir) = file.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    std::fs::write(&part, &bytes).is_ok() && std::fs::rename(&part, file).is_ok()
}

#[cfg(not(test))]
fn fetch_cape(id: String, file: std::path::PathBuf) {
    std::thread::spawn(move || {
        download_to(&format!("https://raw.githubusercontent.com/SoloHandCoder/arsenal-horizon/main/cosmetics/capes/{id}.png"), &file, 2 << 20);
    });
}
#[cfg(test)]
fn fetch_cape(_id: String, _file: std::path::PathBuf) {}

#[cfg(not(test))]
fn fetch_pack(pack: std::path::PathBuf, marker: std::path::PathBuf) {
    std::thread::spawn(move || {
        if download_to("https://github.com/SoloHandCoder/arsenal-horizon/releases/latest/download/arsenal-cosmetics.zip", &pack, 64 << 20) {
            if let Some(dir) = marker.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            let _ = std::fs::write(&marker, VERSION);
            wardrobe(|w| w.pack_ready = true);
        }
    });
}
#[cfg(test)]
fn fetch_pack(_pack: std::path::PathBuf, _marker: std::path::PathBuf) {}

/// 1.8 Animations: how far held items sit lower (blocks).
pub fn held_lower(settings: &Settings) -> f32 {
    if settings.on(OLD_ANIM) { 0.12 } else { 0.0 }
}

/// Blade Settings' inspect key: the blade spins once.
pub fn blade_inspect() {
    wardrobe(|w| w.inspect = Some(Instant::now()));
}

/// The held blade's pose: (size, extra turn in degrees) when you hold a sword with a Wardrobe blade.
pub fn blade_pose(settings: &Settings, holding_sword: bool, has_blade: bool) -> Option<(f32, f32)> {
    if !settings.on(BLADES) || !holding_sword || !has_blade {
        return None;
    }
    let size = option_num(settings, BLADE_SIZE) as f32 / 100.0;
    let turn = wardrobe(|w| {
        let t = w.inspect.map_or(1.0, |i| i.elapsed().as_secs_f32() / 1.2);
        if t >= 1.0 {
            w.inspect = None;
            0.0
        } else {
            let e = t * t * (3.0 - 2.0 * t);
            e * 360.0
        }
    });
    Some((size, turn))
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
        let mark = out.len();
        if let (true, Some((item, ticks))) = (settings.on(BOW), using) {
            if let Some(c) = bow_charge(item, ticks) {
                label(&mut out, format!("{}%", (c * 100.0).round() as i32), [cx, cy + 14.0], if c >= 1.0 { 0x5CE08A } else { 0xFFFFFF });
            }
        }
        place(settings, &mut out, mark, "bow", w, h, width);
        let mark = out.len();
        if settings.on(RETOTEM) {
            if let Some(d) = x.retotem {
                label(&mut out, format!("Re-totem {:.2}s", d.as_secs_f32()), [cx, cy + 48.0], if d.as_millis() < 300 { 0x5CE08A } else { 0xFFC83D });
            } else if x.popped_at.is_some() {
                label(&mut out, "Re-totem!".into(), [cx, cy + 48.0], 0xFF5A5A);
            }
        }
        place(settings, &mut out, mark, "retotem", w, h, width);
        let mark = out.len();
        if settings.on(GOAL) {
            let goal: u32 = GOALS[settings.v[GOAL_KILLS]].0.parse().unwrap_or(10);
            let done = s.pvp.kills_total;
            let text = if done >= goal { format!("Goal reached: {done}/{goal} kills") } else { format!("Goal: {done}/{goal} kills") };
            out.push(Shape::Text(text, [4.0, h - 12.0], if done >= goal { 0x5CE08A } else { 0xFFFFFF }, true));
        }
        place(settings, &mut out, mark, "goal", w, h, width);
        let mark = out.len();
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
        place(settings, &mut out, mark, "intel", w, h, width);
        let mark = out.len();
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
        place(settings, &mut out, mark, "summary", w, h, width);
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
        let mark = out.len();
        if let Some((name, at)) = p.streak.filter(|s| s.1.elapsed() < Duration::from_secs(2)) {
            let _ = at;
            let tw = width(name);
            out.push(Shape::Text(name.to_owned(), [cx - tw / 2.0, h * 0.22], 0xFFC83D, true));
        }
        place(settings, &mut out, mark, "streak", w, h, width);
        // Combat Tag Timer: under the crosshair's left.
        let mark = out.len();
        if let Some(until) = p.combat_until.filter(|u| *u > now) {
            let text = format!("Combat {}s", (until - now).as_secs() + 1);
            let tw = width(&text);
            out.push(Shape::Text(text, [cx - tw / 2.0, cy - 22.0], 0xFF5A5A, true));
        }
        place(settings, &mut out, mark, "combat_tag", w, h, width);
        // Mention Ping: who wrote your name, for three seconds.
        let mark = out.len();
        if let Some((line, at)) = p.mention.as_ref().filter(|m| m.1.elapsed() < Duration::from_secs(3)) {
            let _ = at;
            let tw = width(line);
            out.push(Shape::Fill([cx - tw / 2.0 - 4.0, 4.0, tw + 8.0, 12.0], BACKGROUND));
            out.push(Shape::Text(line.clone(), [cx - tw / 2.0, 6.0], 0xFFC83D, true));
        }
        place(settings, &mut out, mark, "mention", w, h, width);
        // Totem Pop Counter: top right.
        let mark = out.len();
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
        place(settings, &mut out, mark, "pops", w, h, width);
        // Target HUD: right of the crosshair.
        let mark = out.len();
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
        place(settings, &mut out, mark, "target", w, h, width);
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

/// What changed between two setups, to store (each stored setting is its own file: only these are written).
pub fn changed(before: &Settings, after: &Settings) -> Vec<(&'static str, String)> {
    let old = before.stored();
    after.stored().into_iter().filter(|(k, v)| old.iter().find(|(ok, _)| ok == k).is_none_or(|(_, ov)| ov != v)).collect()
}

thread_local! {
    /// Friends' names, read once (the Friend key and the menu change them through `set_friends`).
    static FRIENDS_CACHE: RefCell<Option<Vec<String>>> = const { RefCell::new(None) };
}

/// The friends list, read through `get` once and kept.
pub fn friends(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    FRIENDS_CACHE.with_borrow_mut(|f| f.get_or_insert_with(|| friends_of(get("friend_list"))).clone())
}

pub fn set_friends(list: Vec<String>) {
    FRIENDS_CACHE.with_borrow_mut(|f| *f = Some(list));
}

/// The settings as stored, read once through `get` (the menu changes these).
pub fn raw_settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    STATE.with_borrow_mut(|s| s.settings.get_or_insert_with(|| Settings::read(get)).clone())
}

/// The settings the modules run with: Performance Mode turns the heavy ones off (the stored ones stay as they are).
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    performance(raw_settings(get))
}

/// Performance Mode's switch: the shader pack's effects, block animations, kill and particle effects and lyrics off,
/// particles capped.
pub fn performance(mut s: Settings) -> Settings {
    if s.on(PERF) {
        for f in [BLOOM, LOOK, COLORS, MOTION, MIST, SKY_FX, AMBIENCE, SLICK, BREAK_ANIM, PLACE_ANIM, KILL_FX, CRITS, SWEEP, LYRICS, WARDROBE_FX] {
            s.v[f] = 0;
        }
        s.v[PARTICLES] = 1;
    }
    s
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


/// An Info HUD line's own element.
pub fn info_id(label: &str) -> &'static str {
    match label {
        "FPS" => "info_fps",
        "CPS" => "info_cps",
        "XYZ" => "info_xyz",
        "Facing" => "info_facing",
        "Speed" => "info_speed",
        "Session" => "info_session",
        _ => "info_sneak",
    }
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
    /// Text with its own opacity (0xAARRGGBB) and size (1 is the game's): the menu's.
    Styled(String, [f32; 2], u32, bool, f32),
    /// An item's icon at its own size (a resized HUD element's).
    Icon(String, [f32; 2], f32),
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
    /// Star or unstar module `i` (index in `JAVA_MODULES`).
    Favorite(usize),
    /// Typing goes to the search box.
    Search,
    /// The HUD editor's buttons.
    UndoHud,
    ResetHud,
    /// The HUD editor: the picked element's next theme; this server's own layout on or off.
    HudTheme,
    ServerLayout,
    /// Set the module's key (the next key pressed).
    BindKey(usize),
    /// The module's settings back to their defaults.
    ResetModule(usize),
    HideRisky,
    /// Profiles (index in `Settings.profiles`).
    ProfileName,
    ProfileSave,
    ProfileLoad(usize),
    ProfileUpdate(usize),
    ProfileDelete(usize),
    ProfileAuto(usize),
    CopyCode,
    PasteCode,
    ResetAll,
}

const MENU_BG: u32 = 0xF412_1219;
const PANEL_EDGE: u32 = 0x26FF_FFFF;
const SIDEBAR_BG: u32 = 0x700A_0A0E;
const CARD: u32 = 0x12FF_FFFF;
const CARD_HOVER: u32 = 0x24FF_FFFF;
const TRACK: u32 = 0x38FF_FFFF;
const MUTED: u32 = 0x9A9AA8;
const GREY: u32 = 0x5E5E6A;
const ROW: f32 = 28.0;
const SIDE: f32 = 116.0;
const HEADER: f32 = 34.0;
/// The category's title over its modules.
const LIST_TOP: f32 = 26.0;
const CARD_W: f32 = 98.0;
const CARD_H: f32 = 76.0;

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

/// `text` in lines of at most `max` pixels (at most `lines` of them, the last cut with "...").
pub fn wrap(text: &str, max: f32, lines: usize, width: &dyn Fn(&str) -> f32) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        let next = if line.is_empty() { word.to_owned() } else { format!("{line} {word}") };
        if width(&next) <= max || line.is_empty() {
            line = next;
        } else {
            out.push(std::mem::replace(&mut line, word.to_owned()));
        }
    }
    if !line.is_empty() {
        out.push(line);
    }
    if out.len() > lines {
        let rest = out[lines - 1..].join(" ");
        out.truncate(lines - 1);
        out.push(fit(&rest, max, width));
    }
    out
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
    (((h - HEADER - LIST_TOP - 8.0) / ROW).floor().max(1.0)) as usize
}

fn menu_size(screen: [f32; 4]) -> (f32, f32) {
    ((screen[2] - 20.0).min(540.0).max(260.0), (screen[3] - 20.0).min(330.0).max(150.0))
}

/// Browse's card grid in a menu of size `w`×`h`: (columns, rows shown).
fn grid(w: f32, h: f32) -> (usize, usize) {
    let cols = (((w - 20.0) / (CARD_W + 6.0)).floor().max(1.0)) as usize;
    let rows = (((h - HEADER - 30.0) / (CARD_H + 6.0)).floor().max(1.0)) as usize;
    (cols, rows)
}

/// How many Horizon modules are on.
pub fn modules_on(s: &Settings) -> usize {
    ON_HORIZON.iter().filter(|m| s.v[m.1] == 1).count()
}

/// 0xRRGGBB at opacity `a` (0..1) as 0xAARRGGBB.
fn argb(rgb: u32, a: f32) -> u32 {
    (((a.clamp(0.0, 1.0) * 255.0).round() as u32) << 24) | (rgb & 0xFF_FFFF)
}

/// Between two 0xAARRGGBB colours.
fn mix_argb(a: u32, b: u32, t: f32) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let ch = |shift: u32| {
        let (x, y) = (((a >> shift) & 255) as f32, ((b >> shift) & 255) as f32);
        ((x + (y - x) * t).round() as u32) << shift
    };
    ch(24) | ch(16) | ch(8) | ch(0)
}

/// A rectangle with rounded corners, in rows that don't overlap (so see-through colours stay even).
fn rounded(out: &mut Vec<Shape>, r: [f32; 4], radius: f32, c: u32) {
    let [x, y, w, h] = r;
    let rad = radius.min(w / 2.0).min(h / 2.0).floor().max(0.0);
    if rad < 1.0 {
        out.push(Shape::Fill(r, c));
        return;
    }
    for i in 0..rad as i32 {
        let dy = rad - i as f32 - 0.5;
        let inset = (rad - (rad * rad - dy * dy).max(0.0).sqrt()).round();
        out.push(Shape::Fill([x + inset, y + i as f32, w - 2.0 * inset, 1.0], c));
        out.push(Shape::Fill([x + inset, y + h - 1.0 - i as f32, w - 2.0 * inset, 1.0], c));
    }
    out.push(Shape::Fill([x, y + rad, w, h - 2.0 * rad], c));
}

fn label(out: &mut Vec<Shape>, text: &str, at: [f32; 2], c: u32, shadow: bool, scale: f32) {
    out.push(Shape::Styled(text.to_owned(), at, c, shadow, scale));
}

/// The menu's animations: when it opened, when the view changed, and each eased value (hovers, switches, the
/// sliding markers), by key.
#[derive(Default)]
struct MenuAnim {
    last: Option<Instant>,
    opened: Option<Instant>,
    view: Option<(Page, usize, Option<usize>, usize)>,
    view_at: Option<Instant>,
    values: std::collections::HashMap<u64, f32>,
    dt: f32,
}

/// A frame of the menu starts: a menu not drawn for a moment has just opened.
fn anim_begin(m: &Menu) {
    STATE.with_borrow_mut(|s| {
        let a = &mut s.menu_anim;
        let now = Instant::now();
        let gap = a.last.map(|l| now.duration_since(l));
        if gap.is_none_or(|g| g > Duration::from_millis(300)) {
            a.opened = Some(now);
            a.values.clear();
            a.view = None;
        }
        a.dt = gap.map_or(0.0, |g| g.as_secs_f32().min(0.1));
        a.last = Some(now);
        let view = (m.page, m.category, m.detail, m.tab);
        if a.view != Some(view) {
            if a.view.is_some() {
                a.view_at = Some(now);
            }
            a.view = Some(view);
        }
    });
}

/// Eases the value kept as `key` toward `target` (only while drawing, `animate`); tests see the target.
fn ease(key: u64, target: f32, start: f32, animate: bool) -> f32 {
    if cfg!(test) {
        return target;
    }
    STATE.with_borrow_mut(|s| {
        let a = &mut s.menu_anim;
        let dt = a.dt;
        let v = a.values.entry(key).or_insert(start);
        if animate {
            *v += (target - *v) * (1.0 - (-dt * 16.0).exp());
            if (target - *v).abs() < 0.002 {
                *v = target;
            }
        }
        *v
    })
}

/// Ease-out over `secs` since `at` (1 when done, or with no start).
fn since(at: Option<Instant>, secs: f32) -> f32 {
    if cfg!(test) {
        return 1.0;
    }
    let t = at.map_or(1.0, |a| (a.elapsed().as_secs_f32() / secs).min(1.0));
    1.0 - (1.0 - t).powi(3)
}

fn key(tag: u64, i: usize) -> u64 {
    (tag << 32) | i as u64
}

/// A five-pointed star of radius `r` at `c`.
fn star(out: &mut Vec<Shape>, c: [f32; 2], r: f32, color: u32) {
    let pt = |a: f32, len: f32| [c[0] + a.cos() * len, c[1] + a.sin() * len];
    for k in 0..5 {
        let a = -std::f32::consts::FRAC_PI_2 + k as f32 * std::f32::consts::TAU / 5.0;
        out.push(Shape::Quad([c, pt(a - 0.63, r * 0.42), pt(a, r), pt(a + 0.63, r * 0.42)], color));
    }
}

/// An on/off switch: its track and the sliding knob.
fn switch(out: &mut Vec<Shape>, at: [f32; 2], on: f32, accent: u32) {
    let track = [at[0], at[1], 22.0, 12.0];
    rounded(out, track, 6.0, mix_argb(TRACK, 0xFF00_0000 | accent, on));
    let knob = at[0] + 1.5 + on * 10.0;
    rounded(out, [knob, at[1] + 1.5, 9.0, 9.0], 4.5, 0xFFFF_FFFF);
}

fn button(out: &mut Vec<Shape>, hits: &mut Vec<([f32; 4], Action)>, r: [f32; 4], text: &str, action: Action, mouse: [f32; 2],
          width: &dyn Fn(&str) -> f32, hover: f32) {
    let _ = mouse;
    rounded(out, r, 4.0, mix_argb(CARD, CARD_HOVER, hover));
    label(out, text, [r[0] + (r[2] - width(text)) / 2.0, r[1] + (r[3] - 8.0) / 2.0], 0xFFFF_FFFF, false, 1.0);
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
    let (shapes, hits, _) = layout(screen, mouse, m, s, browse, width, false);
    (shapes, hits)
}

/// The menu as drawn this frame: eased hovers and switches, the panel rising in as it opens and each new page sliding in.
pub fn menu_frame(screen: [f32; 4], mouse: [f32; 2], m: &Menu, s: &Settings, browse: &Fetch, width: &dyn Fn(&str) -> f32) -> Vec<Shape> {
    anim_begin(m);
    let (shapes, _, content) = layout(screen, mouse, m, s, browse, width, true);
    let (open, view) = STATE.with_borrow(|st| (since(st.menu_anim.opened, 0.28), since(st.menu_anim.view_at, 0.22)));
    shapes.into_iter().enumerate().map(|(i, sh)| {
        if i == 0 {
            return fade(sh, open, [0.0, 0.0]);
        }
        let mut k = open;
        if i >= content {
            k *= view;
        }
        fade(sh, k, [0.0, 0.0])
    }).collect()
}

/// A shape moved by `d` at opacity `k`.
fn fade(shape: Shape, k: f32, d: [f32; 2]) -> Shape {
    let a = |c: u32| ((((c >> 24) as f32) * k).round() as u32) << 24 | (c & 0xFF_FFFF);
    match shape {
        Shape::Fill(r, c) => Shape::Fill([r[0] + d[0], r[1] + d[1], r[2], r[3]], a(c)),
        Shape::Text(t, at, rgb, sh) => Shape::Styled(t, [at[0] + d[0], at[1] + d[1]], a(0xFF00_0000 | rgb), sh, 1.0),
        Shape::Styled(t, at, c, sh, sc) => Shape::Styled(t, [at[0] + d[0], at[1] + d[1]], a(c), sh, sc),
        Shape::Quad(p, c) => Shape::Quad(p.map(|q| [q[0] + d[0], q[1] + d[1]]), a(c)),
        Shape::Item(id, at) => if k < 0.6 { Shape::Fill([0.0; 4], 0) } else { Shape::Item(id, [at[0] + d[0], at[1] + d[1]]) },
        Shape::Icon(id, at, size) => if k < 0.6 { Shape::Fill([0.0; 4], 0) } else { Shape::Icon(id, [at[0] + d[0], at[1] + d[1]], size) },
    }
}


/// The HUD editor over the game: each element's frame (picked in the accent), snap lines, and the bar of hints and buttons.
fn hud_editor(screen: [f32; 4], mouse: [f32; 2], m: &Menu, s: &Settings, width: &dyn Fn(&str) -> f32, animate: bool)
              -> (Vec<Shape>, Vec<([f32; 4], Action)>, usize) {
    let accent = s.accent();
    let solid = 0xFF00_0000 | accent;
    let (sw, sh) = (screen[0] + screen[2], screen[1] + screen[3]);
    let mut out = vec![Shape::Fill([0.0, 0.0, sw, sh], 0x3000_0000)];
    let mut hits = Vec::new();
    out.push(Shape::Fill([sw / 2.0, 0.0, 1.0, sh], 0x18FF_FFFF));
    out.push(Shape::Fill([0.0, sh / 2.0, sw, 1.0], 0x18FF_FFFF));
    let boxes = hud_boxes();
    let held = m.drag.as_ref().map(|d| d.id);
    let under = hud_at(&boxes, mouse).map(|b| b.0);
    for (id, r, _) in &boxes {
        let k = key(20, HUD_ELEMENTS.iter().position(|e| e.0 == *id).unwrap_or(0));
        let t = ease(k, if under == Some(*id) || held == Some(*id) { 1.0 } else { 0.0 }, 0.0, animate);
        let picked = m.picked == Some(*id);
        let edge = if picked { solid } else { mix_argb(0x50FF_FFFF, 0xC055_AAFF, t) };
        let fill = if picked { argb(accent, 0.16) } else { argb(0x55AAFF, 0.10 * t) };
        let rr = [r[0] - 2.0, r[1] - 2.0, r[2] + 4.0, r[3] + 4.0];
        out.push(Shape::Fill(rr, fill));
        out.push(Shape::Fill([rr[0], rr[1], rr[2], 1.0], edge));
        out.push(Shape::Fill([rr[0], rr[1] + rr[3] - 1.0, rr[2], 1.0], edge));
        out.push(Shape::Fill([rr[0], rr[1] + 1.0, 1.0, rr[3] - 2.0], edge));
        out.push(Shape::Fill([rr[0] + rr[2] - 1.0, rr[1] + 1.0, 1.0, rr[3] - 2.0], edge));
        if picked || t > 0.05 {
            let name = HUD_ELEMENTS.iter().find(|e| e.0 == *id).map_or(*id, |e| e.1);
            let scale = s.layout.iter().find(|(i, _)| i == id).map_or(1.0, |e| e.1[4]);
            let text = if scale != 1.0 { format!("{name}  {:.0}%", scale * 100.0) } else { name.to_owned() };
            let tw = width(&text) + 8.0;
            let ty = if rr[1] >= 13.0 { rr[1] - 12.0 } else { rr[1] + rr[3] + 1.0 };
            rounded(&mut out, [rr[0], ty, tw, 11.0], 3.0, if picked { solid } else { 0xD018_1820 });
            label(&mut out, &text, [rr[0] + 4.0, ty + 2.0], if picked { 0xFF10_1014 } else { 0xFFFF_FFFF }, false, 1.0);
        }
    }
    for g in &m.guides {
        out.push(Shape::Fill(*g, 0xA055_FFFF));
    }
    let content = out.len();
    // The bar: hints and buttons, top middle (bottom when an element is up there).
    let hint = if boxes.is_empty() {
        "Nothing to move: turn on some HUD modules (and F1 hides the HUD)".to_owned()
    } else {
        "Drag to move  ·  Scroll to resize  ·  Right-click puts one back  ·  Shift: no snapping".to_owned()
    };
    let theme_label = m.picked.map(|id| format!("Theme: {}", s.hud_themes.iter().find(|(e, _)| e == id).map_or("Same as all".to_owned(), |e| {
        let t = e.1.to_lowercase();
        t[..1].to_uppercase() + &t[1..]
    })));
    let layout_label = current_server().map(|k| if s.server_layouts.iter().any(|e| e.0 == k) { "Layout: only this server".to_owned() } else { "Layout: all servers".to_owned() });
    let mut buttons: Vec<(String, Action)> = Vec::new();
    if let Some(t) = theme_label {
        buttons.push((t, Action::HudTheme));
    }
    if let Some(l) = layout_label {
        buttons.push((l, Action::ServerLayout));
    }
    buttons.extend([("Undo".to_owned(), Action::UndoHud), ("Reset all".to_owned(), Action::ResetHud), ("Done".to_owned(), Action::Go(Page::Home))]);
    let last = buttons.len() - 1;
    let bw: f32 = buttons.iter().map(|b| width(&b.0) + 16.0).sum::<f32>() + 4.0 * buttons.len() as f32;
    let total = width(&hint) + 20.0 + bw;
    let mut bar = [(sw - total) / 2.0, 8.0, total, 22.0];
    if boxes.iter().any(|b| b.1[1] < 34.0 && b.1[0] < bar[0] + bar[2] && b.1[0] + b.1[2] > bar[0]) {
        bar[1] = sh - 30.0;
    }
    rounded(&mut out, [bar[0] - 1.0, bar[1] - 1.0, bar[2] + 2.0, bar[3] + 2.0], 7.0, PANEL_EDGE);
    rounded(&mut out, bar, 6.0, MENU_BG);
    label(&mut out, &hint, [bar[0] + 10.0, bar[1] + 7.0], argb(MUTED, 1.0), false, 1.0);
    let mut bx = bar[0] + bar[2] - bw;
    for (i, (name, action)) in buttons.iter().enumerate() {
        let r = [bx, bar[1] + 3.0, width(name) + 16.0, 16.0];
        let t = ease(key(21, i), if inside(r, mouse) { 1.0 } else { 0.0 }, 0.0, animate);
        let enabled = match action { Action::UndoHud => !m.undo.is_empty(), Action::ResetHud => !s.layout.is_empty(), _ => true };
        let base = if i == last { argb(accent, 0.55 + 0.3 * t) } else { mix_argb(CARD, CARD_HOVER, t) };
        rounded(&mut out, r, 4.0, base);
        label(&mut out, name, [r[0] + 8.0, r[1] + 4.0], if enabled { 0xFFFF_FFFF } else { argb(GREY, 1.0) }, false, 1.0);
        if enabled {
            hits.push((r, *action));
        }
        bx += r[2] + 4.0;
    }
    (out, hits, content)
}

#[allow(clippy::too_many_arguments)]
fn layout(
    screen: [f32; 4],
    mouse: [f32; 2],
    m: &Menu,
    s: &Settings,
    browse: &Fetch,
    width: &dyn Fn(&str) -> f32,
    animate: bool,
) -> (Vec<Shape>, Vec<([f32; 4], Action)>, usize) {
    let accent = s.accent();
    let solid = 0xFF00_0000 | accent;
    if m.page == Page::HudEdit {
        return hud_editor(screen, mouse, m, s, width, animate);
    }
    let (w, h) = menu_size(screen);
    let x0 = (screen[0] + (screen[2] - w) / 2.0).round();
    let y0 = (screen[1] + (screen[3] - h) / 2.0).round();
    let hover = |k: u64, r: [f32; 4]| ease(k, if inside(r, mouse) { 1.0 } else { 0.0 }, 0.0, animate);
    let mut out = vec![Shape::Fill([0.0, 0.0, screen[2] + screen[0], screen[3] + screen[1]], 0x9800_0000)];
    // A soft shadow and the accent's glow round the panel, its edge, then the panel.
    for (spread, alpha) in [(18.0, 0.10), (10.0, 0.14), (4.0, 0.22)] {
        rounded(&mut out, [x0 - spread, y0 - spread + 3.0, w + 2.0 * spread, h + 2.0 * spread], 6.0 + spread / 2.0, argb(0x000000, alpha));
    }
    rounded(&mut out, [x0 - 6.0, y0 - 6.0, w + 12.0, h + 12.0], 9.0, argb(accent, 0.05));
    rounded(&mut out, [x0 - 1.0, y0 - 1.0, w + 2.0, h + 2.0], 6.0, PANEL_EDGE);
    rounded(&mut out, [x0, y0, w, h], 5.0, MENU_BG);
    // The accent line along the top, fading out to the right.
    let segs = 32;
    let sw = (w - 10.0) / segs as f32;
    for i in 0..segs {
        let t = i as f32 / segs as f32;
        out.push(Shape::Fill([x0 + 5.0 + i as f32 * sw, y0, sw + 0.01, 2.0], argb(accent, 1.0 - t * 0.9)));
    }
    let mut hits = Vec::new();
    // Title and where you are.
    let title_w = width("ARSENAL") * 1.5;
    let title = [x0 + 8.0, y0 + 6.0, title_w + 10.0, 20.0];
    if m.page != Page::Home {
        let t = hover(key(1, 0), title);
        if t > 0.01 {
            rounded(&mut out, title, 4.0, argb(0xFFFFFF, 0.08 * t));
        }
        hits.push((title, Action::Go(Page::Home)));
    }
    label(&mut out, "ARSENAL", [x0 + 13.0, y0 + 11.0], solid, true, 1.5);
    let crumb = match (m.page, m.detail.and_then(|d| JAVA_MODULES.get(d))) {
        (Page::Home, _) => format!("on Horizon  {VERSION}"),
        (Page::Modules, Some(j)) => format!("/  {}  /  {}", CATEGORIES[j.category], j.name),
        (Page::Modules, None) => format!("/  Modules  /  {}", CATEGORIES.get(m.category).copied().unwrap_or("")),
        (Page::Browse, _) => "/  Browse".into(),
        (Page::Cosmetics, _) => "/  Cosmetics".into(),
        (Page::HudEdit, _) => "/  Edit HUD".into(),
        (Page::Profiles, _) => "/  Profiles".into(),
    };
    label(&mut out, &fit(&crumb, w - title_w - 60.0, width), [x0 + 22.0 + title_w, y0 + 14.0], argb(MUTED, 1.0), false, 1.0);
    if m.page == Page::Modules {
        let mut bx = x0 + w - 30.0;
        for (i, (name, page)) in [("Edit HUD", Page::HudEdit), ("Profiles", Page::Profiles)].iter().enumerate() {
            let r = [bx - width(name) - 14.0, y0 + 9.0, width(name) + 14.0, 15.0];
            let t = hover(key(24, i), r);
            rounded(&mut out, r, 4.0, mix_argb(CARD, CARD_HOVER, t));
            label(&mut out, name, [r[0] + 7.0, r[1] + 4.0], 0xFFFF_FFFF, false, 1.0);
            hits.push((r, Action::Go(*page)));
            bx = r[0] - 4.0;
        }
    }
    let close = [x0 + w - 24.0, y0 + 9.0, 15.0, 15.0];
    let t = hover(key(2, 0), close);
    rounded(&mut out, close, 7.5, mix_argb(CARD, 0xD0E0_4848, t));
    label(&mut out, "x", [close[0] + (close[2] - width("x")) / 2.0, close[1] + 3.5], 0xFFFF_FFFF, false, 1.0);
    hits.push((close, Action::Close));
    out.push(Shape::Fill([x0 + 10.0, y0 + HEADER - 2.0, w - 20.0, 1.0], 0x14FF_FFFF));
    let top = y0 + HEADER;
    let mut content = out.len();

    match m.page {
        Page::HudEdit => {}
        Page::Profiles => {
            let cx = x0 + 18.0;
            let cw = w - 36.0;
            label(&mut out, "Profiles", [cx, top + 8.0], 0xFFFF_FFFF, true, 1.25);
            label(&mut out, "Saved setups you switch between; Auto loads one whenever you join this server.", [cx, top + 24.0], argb(MUTED, 1.0), false, 1.0);
            // The name box and Save current.
            let nb = [cx, top + 38.0, cw - 104.0, 17.0];
            let focus = m.name_focus || !m.profile_name.is_empty();
            rounded(&mut out, nb, 4.0, if focus { argb(accent, 0.16) } else { 0x14FF_FFFF });
            if m.profile_name.is_empty() {
                label(&mut out, "Name a new profile...", [nb[0] + 7.0, nb[1] + 5.0], argb(GREY, 1.0), false, 1.0);
            } else {
                label(&mut out, &m.profile_name, [nb[0] + 7.0, nb[1] + 5.0], 0xFFFF_FFFF, false, 1.0);
            }
            if m.name_focus {
                out.push(Shape::Fill([nb[0] + 8.0 + width(&m.profile_name), nb[1] + 4.0, 1.0, 9.0], 0xFFFF_FFFF));
            }
            hits.push((nb, Action::ProfileName));
            let save = [nb[0] + nb[2] + 6.0, nb[1], 98.0, 17.0];
            let t = hover(key(25, 0), save);
            rounded(&mut out, save, 4.0, argb(accent, 0.5 + 0.3 * t));
            label(&mut out, "Save current", [save[0] + (save[2] - width("Save current")) / 2.0, save[1] + 5.0], 0xFFFF_FFFF, false, 1.0);
            hits.push((save, Action::ProfileSave));
            content = out.len();
            let server = current_server();
            let mut y = top + 62.0;
            if s.profiles.is_empty() {
                label(&mut out, "No profiles yet.", [cx + 2.0, y + 4.0], argb(GREY, 1.0), false, 1.0);
            }
            for (i, (name, _)) in s.profiles.iter().enumerate() {
                if y + 20.0 > y0 + h - 30.0 {
                    break;
                }
                let r = [cx, y, cw, 19.0];
                let t = hover(key(26, i), r);
                rounded(&mut out, r, 4.0, mix_argb(CARD, CARD_HOVER, t));
                let auto = server.as_ref().is_some_and(|k| s.auto_profiles.iter().any(|(sk, n)| sk == k && n == name));
                label(&mut out, &fit(name, cw - 250.0, width), [r[0] + 8.0, r[1] + 6.0], 0xFFFF_FFFF, false, 1.0);
                let mut bx = r[0] + r[2] - 4.0;
                let deleting = m.confirm.as_deref() == Some(name.as_str());
                let mut buttons = vec![(if deleting { "Sure?" } else { "Delete" }, Action::ProfileDelete(i)), ("Update", Action::ProfileUpdate(i)), ("Load", Action::ProfileLoad(i))];
                if server.is_some() {
                    buttons.push(("Auto", Action::ProfileAuto(i)));
                }
                for (n, (text, action)) in buttons.iter().enumerate() {
                    let br = [bx - width(text) - 12.0, r[1] + 3.0, width(text) + 12.0, 13.0];
                    let bt = hover(key(27, i * 8 + n), br);
                    let lit = (*text == "Auto" && auto) || deleting && n == 0;
                    rounded(&mut out, br, 4.0, if lit { argb(if deleting { 0xE04848 } else { accent }, 0.6 + 0.2 * bt) } else { mix_argb(0x18FF_FFFF, 0x30FF_FFFF, bt) });
                    label(&mut out, text, [br[0] + 6.0, br[1] + 3.0], 0xFFFF_FFFF, false, 1.0);
                    hits.push((br, *action));
                    bx = br[0] - 4.0;
                }
                y += 22.0;
            }
            // Share codes and Reset all along the bottom.
            let by = y0 + h - 24.0;
            let mut bx = cx;
            let resetting = m.confirm.as_deref() == Some("\u{0}reset");
            for (i, (text, action)) in [("Copy code", Action::CopyCode), ("Paste code", Action::PasteCode), (if resetting { "Sure? Reset all" } else { "Reset all" }, Action::ResetAll)].iter().enumerate() {
                let r = [bx, by, width(text) + 16.0, 16.0];
                let t = hover(key(28, i), r);
                rounded(&mut out, r, 4.0, if i == 2 && resetting { argb(0xE04848, 0.7) } else { mix_argb(CARD, CARD_HOVER, t) });
                label(&mut out, text, [r[0] + 8.0, r[1] + 4.0], 0xFFFF_FFFF, false, 1.0);
                hits.push((r, *action));
                bx += r[2] + 6.0;
            }
            if let Some(n) = &m.notice {
                let text = fit(n, x0 + w - bx - 24.0, width);
                label(&mut out, &text, [bx + 6.0, by + 4.0], solid, false, 1.0);
            }
        }
        Page::Home => {
            label(&mut out, "Welcome back", [x0 + 18.0, top + 10.0], 0xFFFF_FFFF, true, 1.25);
            let on = modules_on(s);
            let line = format!("{} of {} Arsenal modules run on Horizon  ·  {} on", ON_HORIZON.len(), JAVA_MODULES.len(), on);
            label(&mut out, &fit(&line, w - 36.0, width), [x0 + 18.0, top + 26.0], argb(MUTED, 1.0), false, 1.0);
            let bar = [x0 + 18.0, top + 40.0, w - 36.0, 4.0];
            rounded(&mut out, bar, 2.0, 0x20FF_FFFF);
            let part = ease(key(3, 0), ON_HORIZON.len() as f32 / JAVA_MODULES.len() as f32, 0.0, animate);
            rounded(&mut out, [bar[0], bar[1], (bar[2] * part).max(4.0), bar[3]], 2.0, solid);
            let tiles: [(&str, String, Page, u8); 4] = [
                ("Modules", format!("{on} on. Switch them, set them, search, star favourites."), Page::Modules, 0),
                ("Edit HUD", "Drag, resize and line up your HUD over the game.".into(), Page::HudEdit, 3),
                ("Browse", "Crosshairs and HUD themes from the website, one click to use.".into(), Page::Browse, 1),
                ("Cosmetics", "Capes, hats and more: shown to players on the Java version.".into(), Page::Cosmetics, 2),
            ];
            let tw = ((w - 36.0 - 24.0) / 4.0).floor();
            let ty = top + 54.0;
            let th = (h - HEADER - 54.0 - 36.0).max(60.0);
            for (i, (name, text, page, icon)) in tiles.iter().enumerate() {
                let r = [x0 + 18.0 + i as f32 * (tw + 8.0), ty, tw, th];
                let t = hover(key(4, i), r);
                let lift = -2.0 * t;
                let rr = [r[0], r[1] + lift, r[2], r[3]];
                if t > 0.01 {
                    rounded(&mut out, [rr[0] - 2.0, rr[1] - 2.0, rr[2] + 4.0, rr[3] + 4.0], 7.0, argb(accent, 0.12 * t));
                }
                rounded(&mut out, rr, 5.0, mix_argb(CARD, CARD_HOVER, t));
                let line_w = rr[2] * (0.3 + 0.7 * t) - 10.0;
                rounded(&mut out, [rr[0] + 5.0, rr[1] + 1.0, line_w.max(6.0), 2.0], 1.0, if *icon == 2 { 0xFF5E_5E6A } else { solid });
                // The tile's mark.
                let (ix, iy) = (rr[0] + 12.0, rr[1] + 14.0);
                let ic = if *icon == 2 { argb(MUTED, 1.0) } else { solid };
                match icon {
                    0 => {
                        for (dx, dy) in [(0.0, 0.0), (8.0, 0.0), (0.0, 8.0), (8.0, 8.0)] {
                            rounded(&mut out, [ix + dx, iy + dy, 6.0, 6.0], 1.0, ic);
                        }
                    }
                    3 => {
                        rounded(&mut out, [ix, iy + 2.0, 14.0, 10.0], 2.0, argb(accent, 0.35));
                        rounded(&mut out, [ix + 2.0, iy + 4.0, 5.0, 3.0], 1.0, ic);
                        rounded(&mut out, [ix + 8.0, iy + 8.0, 4.0, 2.0], 1.0, ic);
                    }
                    1 => {
                        out.push(Shape::Fill([ix + 6.0, iy, 2.0, 14.0], ic));
                        out.push(Shape::Fill([ix, iy + 6.0, 14.0, 2.0], ic));
                        rounded(&mut out, [ix + 4.0, iy + 4.0, 6.0, 6.0], 3.0, argb(0xFFFFFF, 0.9));
                    }
                    _ => {
                        let (cx, cy) = (ix + 7.0, iy + 7.0);
                        let pt = |a: f32, r: f32| [cx + a.cos() * r, cy + a.sin() * r];
                        for k in 0..5 {
                            let a = -std::f32::consts::FRAC_PI_2 + k as f32 * std::f32::consts::TAU / 5.0;
                            out.push(Shape::Quad([[cx, cy], pt(a - 0.63, 3.0), pt(a, 7.5), pt(a + 0.63, 3.0)], ic));
                        }
                    }
                }
                label(&mut out, name, [rr[0] + 12.0, rr[1] + 36.0], 0xFFFF_FFFF, true, 1.25);
                for (n, l) in wrap(text, rr[2] - 22.0, ((th - 54.0) / 10.0).floor().max(1.0) as usize, width).iter().enumerate() {
                    label(&mut out, l, [rr[0] + 12.0, rr[1] + 52.0 + n as f32 * 10.0], argb(MUTED, 1.0), false, 1.0);
                }
                hits.push((r, Action::Go(*page)));
            }
            let done = [x0 + (w - 84.0) / 2.0, y0 + h - 26.0, 84.0, 17.0];
            let t = hover(key(5, 0), done);
            button(&mut out, &mut hits, done, "Done", Action::Close, mouse, width, t);
            label(&mut out, "Right Shift", [x0 + 18.0, y0 + h - 21.0], argb(GREY, 1.0), false, 1.0);
        }
        Page::Cosmetics => {
            let card = [x0 + 18.0, top + 10.0, w - 36.0, 86.0];
            rounded(&mut out, card, 5.0, CARD);
            out.push(Shape::Fill([card[0], card[1] + 6.0, 2.0, card[3] - 12.0], solid));
            label(&mut out, "Your cosmetics", [card[0] + 12.0, card[1] + 10.0], 0xFFFF_FFFF, true, 1.25);
            let lines = [
                "They still show to players on the Java version of Arsenal.",
                "Horizon can't draw capes, hats or pets on players yet, so the Wardrobe isn't here.",
                "Change what you wear on the launcher's Cosmetics page.",
            ];
            let mut y = card[1] + 28.0;
            for l in lines {
                for part in wrap(l, card[2] - 24.0, 2, width) {
                    label(&mut out, &part, [card[0] + 12.0, y], argb(MUTED, 1.0), false, 1.0);
                    y += 11.0;
                }
            }
            let back = [x0 + 18.0, card[1] + card[3] + 10.0, 76.0, 17.0];
            let t = hover(key(6, 0), back);
            button(&mut out, &mut hits, back, "< Home", Action::Go(Page::Home), mouse, width, t);
        }
        Page::Browse => {
            let mut tx = x0 + 14.0;
            let mut tabs = Vec::new();
            for (name, _) in BROWSE_TABS.iter() {
                let r = [tx, top + 4.0, width(name) + 18.0, 17.0];
                tabs.push(r);
                tx += r[2] + 4.0;
            }
            // The marker slides to the chosen tab.
            if let Some(sel) = tabs.get(m.tab) {
                let x = ease(key(7, 0), sel[0], sel[0], animate);
                let tw = ease(key(7, 1), sel[2], sel[2], animate);
                rounded(&mut out, [x, sel[1], tw, sel[3]], 5.0, CARD_HOVER);
                rounded(&mut out, [x + 6.0, sel[1] + sel[3] - 2.0, tw - 12.0, 2.0], 1.0, solid);
            }
            for (i, r) in tabs.iter().enumerate() {
                let t = hover(key(8, i), *r);
                let selected = m.tab == i;
                if !selected && t > 0.01 {
                    rounded(&mut out, *r, 5.0, argb(0xFFFFFF, 0.07 * t));
                }
                let c = if selected { 0xFFFF_FFFF } else { mix_argb(argb(MUTED, 1.0), 0xFFFF_FFFF, t) };
                label(&mut out, BROWSE_TABS[i].0, [r[0] + 9.0, r[1] + 5.0], c, false, 1.0);
                hits.push((*r, Action::Tab(i)));
            }
            if let Some(n) = &m.notice {
                let text = fit(n, x0 + w - tx - 22.0, width);
                let r = [tx + 6.0, top + 4.0, width(&text) + 14.0, 17.0];
                rounded(&mut out, r, 5.0, argb(accent, 0.18));
                label(&mut out, &text, [r[0] + 7.0, r[1] + 5.0], solid, false, 1.0);
            }
            content = out.len();
            let gy = top + 28.0;
            match browse {
                Fetch::Loading => {
                    let dots = if cfg!(test) { 3 } else { (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis()) / 300 % 4) as usize };
                    label(&mut out, &format!("Loading from the Arsenal website{}", ".".repeat(dots)), [x0 + 16.0, gy + 6.0], argb(MUTED, 1.0), false, 1.0);
                }
                Fetch::Failed => {
                    label(&mut out, "Couldn't reach the Arsenal website.", [x0 + 16.0, gy + 6.0], argb(MUTED, 1.0), false, 1.0);
                    let r = [x0 + 16.0, gy + 22.0, 80.0, 17.0];
                    let t = hover(key(9, 0), r);
                    button(&mut out, &mut hits, r, "Try again", Action::Retry, mouse, width, t);
                }
                Fetch::Ready(items) if items.is_empty() => label(&mut out, "Nothing here yet.", [x0 + 16.0, gy + 6.0], argb(MUTED, 1.0), false, 1.0),
                Fetch::Ready(items) => {
                    let (cols, rows) = grid(w, h);
                    let first_row = m.browse_scroll.min(items.len().div_ceil(cols).saturating_sub(rows));
                    for (n, item) in items.iter().enumerate().skip(first_row * cols).take(cols * rows) {
                        let k = n - first_row * cols;
                        let r = [x0 + 14.0 + (k % cols) as f32 * (CARD_W + 6.0), gy + (k / cols) as f32 * (CARD_H + 6.0), CARD_W, CARD_H];
                        let usable = !matches!(item.look, Look::Preset);
                        let t = if usable { hover(key(10, n), r) } else { 0.0 };
                        let rr = [r[0], r[1] - 2.0 * t, r[2], r[3]];
                        if t > 0.01 {
                            rounded(&mut out, [rr[0] - 2.0, rr[1] - 2.0, rr[2] + 4.0, rr[3] + 4.0], 7.0, argb(accent, 0.14 * t));
                        }
                        rounded(&mut out, rr, 5.0, mix_argb(CARD, CARD_HOVER, t));
                        let preview = [rr[0] + 4.0, rr[1] + 4.0, rr[2] - 8.0, 38.0];
                        rounded(&mut out, preview, 3.0, 0xFF0A_0A0E);
                        match &item.look {
                            Look::Crosshair(c) => {
                                let span = c.rows.len().max(c.rows.iter().map(|x| x.len()).max().unwrap_or(1)) as f32;
                                let cell = (32.0 / span).min(3.0);
                                out.extend(crosshair_shapes(c, [preview[0] + preview[2] / 2.0, preview[1] + preview[3] / 2.0], cell));
                            }
                            Look::Theme(th) => {
                                let p = [preview[0] + 8.0, preview[1] + 9.0, preview[2] - 16.0, 20.0];
                                rounded(&mut out, p, 3.0, th.background);
                                out.push(Shape::Fill([p[0], p[1] + 3.0, 2.0, p[3] - 6.0], 0xFF00_0000 | th.accent));
                                out.push(Shape::Text("FPS".into(), [p[0] + 6.0, p[1] + 6.0], th.accent, true));
                                out.push(Shape::Text("144".into(), [p[0] + 8.0 + width("FPS"), p[1] + 6.0], th.text, true));
                            }
                            Look::Preset => label(&mut out, "Java only", [preview[0] + 6.0, preview[1] + 15.0], argb(GREY, 1.0), false, 1.0),
                        }
                        label(&mut out, &fit(&item.title, rr[2] - 10.0, width), [rr[0] + 5.0, rr[1] + 47.0], if usable { 0xFFFF_FFFF } else { argb(GREY, 1.0) }, false, 1.0);
                        label(&mut out, &fit(&format!("by {}", item.author), rr[2] - 10.0, width), [rr[0] + 5.0, rr[1] + 59.0], argb(GREY, 1.0), false, 1.0);
                        if usable {
                            hits.push((r, Action::Apply(n)));
                        }
                    }
                }
            }
        }
        Page::Modules => {
            let side = [x0 + 6.0, top + 2.0, SIDE - 4.0, h - HEADER - 8.0];
            rounded(&mut out, side, 4.0, SIDEBAR_BG);
            // The search box.
            let sr = [side[0] + 4.0, side[1] + 5.0, side[2] - 8.0, 16.0];
            let t = hover(key(22, 0), sr);
            let focus = m.search_focus || !m.search.is_empty();
            rounded(&mut out, sr, 4.0, if focus { argb(accent, 0.16) } else { mix_argb(0x10FF_FFFF, 0x1CFF_FFFF, t) });
            if focus {
                out.push(Shape::Fill([sr[0] + 3.0, sr[1] + sr[3] - 1.0, sr[2] - 6.0, 1.0], solid));
            }
            // A magnifier.
            rounded(&mut out, [sr[0] + 5.0, sr[1] + 4.0, 6.0, 6.0], 3.0, argb(MUTED, 0.9));
            rounded(&mut out, [sr[0] + 6.0, sr[1] + 5.0, 4.0, 4.0], 2.0, 0xFF16_161C);
            out.push(Shape::Quad([[sr[0] + 10.0, sr[1] + 9.0], [sr[0] + 11.0, sr[1] + 9.0], [sr[0] + 13.0, sr[1] + 12.0], [sr[0] + 12.0, sr[1] + 12.0]], argb(MUTED, 0.9)));
            if m.search.is_empty() {
                label(&mut out, "Search...", [sr[0] + 16.0, sr[1] + 4.5], argb(GREY, 1.0), false, 1.0);
            } else {
                let text = fit(&m.search, sr[2] - 24.0, width);
                label(&mut out, &text, [sr[0] + 16.0, sr[1] + 4.5], 0xFFFF_FFFF, false, 1.0);
            }
            if m.search_focus {
                let blink = cfg!(test) || std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis()) / 500 % 2 == 0;
                if blink {
                    let cx = sr[0] + 16.0 + width(&fit(&m.search, sr[2] - 24.0, width)) + 1.0;
                    out.push(Shape::Fill([cx, sr[1] + 4.0, 1.0, 9.0], 0xFFFF_FFFF));
                }
            }
            hits.push((sr, Action::Search));
            let selected = if !m.search.is_empty() { usize::MAX } else { m.detail.and_then(|d| JAVA_MODULES.get(d)).map_or(m.category, |j| j.category) };
            // Favorites first, then the categories.
            let row_of = |i: usize| if i == FAVORITES { 0 } else { i + 1 };
            let cat_r = |i: usize| [side[0] + 4.0, side[1] + 26.0 + row_of(i) as f32 * 18.0, side[2] - 8.0, 16.0];
            // The marker slides to the chosen category.
            if selected != usize::MAX {
                let target = cat_r(selected);
                let y = ease(key(11, 0), target[1], target[1], animate);
                rounded(&mut out, [target[0], y, target[2], target[3]], 4.0, CARD_HOVER);
                rounded(&mut out, [target[0], y + 4.0, 2.0, target[3] - 8.0], 1.0, solid);
            }
            {
                let r = cat_r(FAVORITES);
                let t = hover(key(12, FAVORITES), r);
                if selected != FAVORITES && t > 0.01 {
                    rounded(&mut out, r, 4.0, argb(0xFFFFFF, 0.06 * t));
                }
                star(&mut out, [r[0] + 11.0, r[1] + 8.0], 4.5, if selected == FAVORITES { solid } else { argb(0xF5B83D, 0.9) });
                let c = if selected == FAVORITES { 0xFFFF_FFFF } else { mix_argb(argb(MUTED, 1.0), 0xFFFF_FFFF, t) };
                label(&mut out, "Favorites", [r[0] + 19.0, r[1] + 4.5], c, false, 1.0);
                if !s.favorites.is_empty() {
                    let n = s.favorites.len().to_string();
                    label(&mut out, &n, [r[0] + r[2] - width(&n) - 6.0, r[1] + 4.5], argb(GREY, 1.0), false, 1.0);
                }
                hits.push((r, Action::Category(FAVORITES)));
            }
            for (i, name) in CATEGORIES.iter().enumerate() {
                let r = cat_r(i);
                let t = hover(key(12, i), r);
                if i != selected && t > 0.01 {
                    rounded(&mut out, r, 4.0, argb(0xFFFFFF, 0.06 * t));
                }
                let list = category_modules(i);
                let on = list.iter().filter(|&&j| on_horizon(JAVA_MODULES[j].id).is_some_and(|(f, _)| s.v[f] == 1)).count();
                let count = if on > 0 { on.to_string() } else { String::new() };
                let c = if i == selected { 0xFFFF_FFFF } else { mix_argb(argb(MUTED, 1.0), 0xFFFF_FFFF, t) };
                label(&mut out, &fit(name, r[2] - 26.0, width), [r[0] + 8.0, r[1] + 4.5], c, false, 1.0);
                if !count.is_empty() {
                    label(&mut out, &count, [r[0] + r[2] - width(&count) - 6.0, r[1] + 4.5], if i == selected { solid } else { argb(GREY, 1.0) }, false, 1.0);
                }
                hits.push((r, Action::Category(i)));
            }
            {
                let r = cat_r(NEW_TAB);
                let t = hover(key(12, NEW_TAB), r);
                if selected != NEW_TAB && t > 0.01 {
                    rounded(&mut out, r, 4.0, argb(0xFFFFFF, 0.06 * t));
                }
                let c = if selected == NEW_TAB { 0xFFFF_FFFF } else { mix_argb(argb(MUTED, 1.0), 0xFFFF_FFFF, t) };
                // A small spark.
                let (sx, sy) = (r[0] + 11.0, r[1] + 8.0);
                out.push(Shape::Quad([[sx, sy - 4.5], [sx + 1.2, sy], [sx, sy + 4.5], [sx - 1.2, sy]], 0xFF55_FFFF));
                out.push(Shape::Quad([[sx - 4.5, sy], [sx, sy - 1.2], [sx + 4.5, sy], [sx, sy + 1.2]], 0xFF55_FFFF));
                label(&mut out, "New", [r[0] + 19.0, r[1] + 4.5], c, false, 1.0);
                hits.push((r, Action::Category(NEW_TAB)));
                // Hide risky.
                let hr = [side[0] + 4.0, side[1] + side[3] - 20.0, side[2] - 8.0, 16.0];
                let ht = hover(key(29, 0), hr);
                rounded(&mut out, [hr[0] + 3.0, hr[1] + 3.0, 10.0, 10.0], 2.0, if s.hide_risky { solid } else { mix_argb(0x30FF_FFFF, 0x50FF_FFFF, ht) });
                if s.hide_risky {
                    out.push(Shape::Quad([[hr[0] + 5.0, hr[1] + 8.0], [hr[0] + 6.5, hr[1] + 7.0], [hr[0] + 8.0, hr[1] + 9.5], [hr[0] + 7.0, hr[1] + 11.0]], 0xFF10_1014));
                    out.push(Shape::Quad([[hr[0] + 7.0, hr[1] + 11.0], [hr[0] + 8.0, hr[1] + 9.5], [hr[0] + 11.0, hr[1] + 5.0], [hr[0] + 12.0, hr[1] + 6.0]], 0xFF10_1014));
                }
                label(&mut out, "Hide risky", [hr[0] + 18.0, hr[1] + 4.5], mix_argb(argb(MUTED, 1.0), 0xFFFF_FFFF, ht), false, 1.0);
                hits.push((hr, Action::HideRisky));
            }
            content = out.len();
            let cx = x0 + SIDE + 10.0;
            let cw = w - SIDE - 22.0;
            match m.detail.and_then(|d| JAVA_MODULES.get(d)) {
                Some(module) => {
                    let back = [cx, top + 4.0, width("< Back") + 14.0, 16.0];
                    let t = hover(key(13, 0), back);
                    button(&mut out, &mut hits, back, "< Back", Action::Back, mouse, width, t);
                    label(&mut out, &fit(module.name, cw - back[2] - 12.0, width), [back[0] + back[2] + 10.0, top + 7.0], solid, true, 1.25);
                    let mut y = top + 26.0;
                    for l in wrap(module.description, cw, 2, width) {
                        label(&mut out, &l, [cx, y], argb(MUTED, 1.0), false, 1.0);
                        y += 10.0;
                    }
                    y += 4.0;
                    if risky(module.id) {
                        label(&mut out, "Some servers may not allow this. Use at your own risk!", [cx, y], 0xFFFF_C83D, false, 1.0);
                        y += 12.0;
                    }
                    if on_horizon(module.id).is_some() {
                        let i = JAVA_MODULES.iter().position(|j| j.id == module.id).unwrap_or(0);
                        let kr = [cx, y, (cw - 70.0).min(220.0), 15.0];
                        let kt = hover(key(30, 0), kr);
                        let waiting = m.binding == Some(i);
                        rounded(&mut out, kr, 4.0, if waiting { argb(accent, 0.3) } else { mix_argb(CARD, CARD_HOVER, kt) });
                        let bound = s.keys.iter().find(|(id, _)| id == module.id).map(|e| key_name(&e.1));
                        let text = if waiting { "Key: press a key (Escape: none)".to_owned() } else { format!("Key: {}", bound.unwrap_or_else(|| "None".into())) };
                        label(&mut out, &fit(&text, kr[2] - 12.0, width), [kr[0] + 7.0, kr[1] + 4.0], 0xFFFF_FFFF, false, 1.0);
                        hits.push((kr, Action::BindKey(i)));
                        let rr = [kr[0] + kr[2] + 6.0, y, 58.0, 15.0];
                        let rt = hover(key(30, 1), rr);
                        rounded(&mut out, rr, 4.0, mix_argb(CARD, CARD_HOVER, rt));
                        label(&mut out, "Reset", [rr[0] + (rr[2] - width("Reset")) / 2.0, rr[1] + 4.0], 0xFFFF_FFFF, false, 1.0);
                        hits.push((rr, Action::ResetModule(i)));
                        y += 18.0;
                        let clash = key_clash(s, module.id);
                        if !clash.is_empty() {
                            label(&mut out, &fit(&format!("Also on this key: {}", clash.join(", ")), cw, width), [cx, y], 0xFFFF_C83D, false, 1.0);
                            y += 12.0;
                        }
                    }
                    match on_horizon(module.id) {
                        Some((toggle, fields)) => {
                            if let Some(note) = horizon_note(module.id) {
                                let lines = wrap(note, cw - 18.0, 3, width);
                                let r = [cx, y, cw, 8.0 + lines.len() as f32 * 10.0];
                                rounded(&mut out, r, 4.0, argb(accent, 0.10));
                                out.push(Shape::Fill([r[0], r[1] + 3.0, 2.0, r[3] - 6.0], solid));
                                for (n, l) in lines.iter().enumerate() {
                                    label(&mut out, l, [r[0] + 9.0, r[1] + 5.0 + n as f32 * 10.0], 0xFFE0_E0E8, false, 1.0);
                                }
                                y += r[3] + 6.0;
                            }
                            for &f in std::iter::once(&toggle).chain(fields.iter()) {
                                if y + 20.0 > y0 + h - 4.0 {
                                    break;
                                }
                                let r = [cx, y, cw, 19.0];
                                let t = hover(key(14, f), r);
                                rounded(&mut out, r, 4.0, mix_argb(CARD, CARD_HOVER, t));
                                let name = if f == toggle { "Enabled" } else { FIELDS[f].label };
                                label(&mut out, &fit(name, cw - 110.0, width), [r[0] + 8.0, r[1] + 6.0], 0xFFFF_FFFF, false, 1.0);
                                if FIELDS[f].options == ONOFF {
                                    let on = ease(key(15, f), s.v[f] as f32, s.v[f] as f32, animate);
                                    switch(&mut out, [r[0] + r[2] - 30.0, r[1] + 4.0], on, accent);
                                } else {
                                    let value = fit(FIELDS[f].options[s.v[f]].1, 90.0, width);
                                    let vw = width(&value) + 22.0;
                                    let chip = [r[0] + r[2] - vw - 6.0, r[1] + 3.0, vw, 13.0];
                                    rounded(&mut out, chip, 4.0, mix_argb(0x20FF_FFFF, argb(accent, 0.35), t));
                                    label(&mut out, &value, [chip[0] + 11.0, chip[1] + 3.0], 0xFFFF_FFFF, false, 1.0);
                                    label(&mut out, ">", [chip[0] + chip[2] - 8.0, chip[1] + 3.0], argb(MUTED, 1.0), false, 1.0);
                                }
                                hits.push((r, Action::Setting(f)));
                                y += 22.0;
                            }
                            if module.id == "custom_crosshair" || module.id == "hud_theme" {
                                let r = [cx, y + 4.0, width("Open Browse") + 18.0, 17.0];
                                let t = hover(key(16, 0), r);
                                button(&mut out, &mut hits, r, "Open Browse", Action::Go(Page::Browse), mouse, width, t);
                            }
                        }
                        None => {
                            let lines = wrap(why_not(module.id), cw - 18.0, 3, width);
                            let r = [cx, y, cw, 20.0 + lines.len() as f32 * 10.0];
                            rounded(&mut out, r, 4.0, CARD);
                            label(&mut out, "Only in the Java version of Arsenal for now:", [r[0] + 9.0, r[1] + 6.0], 0xFFFF_FFFF, false, 1.0);
                            for (n, l) in lines.iter().enumerate() {
                                label(&mut out, l, [r[0] + 9.0, r[1] + 18.0 + n as f32 * 10.0], argb(MUTED, 1.0), false, 1.0);
                            }
                        }
                    }
                }
                None => {
                    let list = shown_modules(m, s);
                    let on = list.iter().filter(|&&j| on_horizon(JAVA_MODULES[j].id).is_some_and(|(f, _)| s.v[f] == 1)).count();
                    let here = list.iter().filter(|&&j| on_horizon(JAVA_MODULES[j].id).is_some()).count();
                    let title = if !m.search.trim().is_empty() {
                        format!("Results for \"{}\"", fit(m.search.trim(), cw / 2.0, width))
                    } else if m.category == FAVORITES {
                        "Favorites".to_owned()
                    } else if m.category == NEW_TAB {
                        "New on Horizon".to_owned()
                    } else {
                        CATEGORIES.get(m.category).copied().unwrap_or("").to_owned()
                    };
                    label(&mut out, &title, [cx + 2.0, top + 7.0], 0xFFFF_FFFF, true, 1.25);
                    let info = format!("{on} on  ·  {here} of {} on Horizon", list.len());
                    if list.is_empty() {
                        let empty = if !m.search.trim().is_empty() { "No module matches. Try fewer words." } else if m.category == FAVORITES { "Star a module with its star to keep it here (and in Quick Menu, V)." } else { "" };
                        for (n, l) in wrap(empty, cw - 8.0, 2, width).iter().enumerate() {
                            label(&mut out, l, [cx + 4.0, top + LIST_TOP + 6.0 + n as f32 * 10.0], argb(MUTED, 1.0), false, 1.0);
                        }
                    }
                    label(&mut out, &info, [cx + cw - width(&info) - 2.0, top + 9.0], argb(GREY, 1.0), false, 1.0);
                    let shown = rows_shown(screen);
                    let first = m.scroll.min(list.len().saturating_sub(shown));
                    let ly = top + LIST_TOP;
                    for (row, &i) in list.iter().skip(first).take(shown).enumerate() {
                        let module = &JAVA_MODULES[i];
                        let r = [cx, ly + row as f32 * ROW, cw, ROW - 4.0];
                        let here = on_horizon(module.id);
                        let t = hover(key(17, i), r);
                        rounded(&mut out, r, 4.0, mix_argb(CARD, CARD_HOVER, t));
                        if t > 0.01 && here.is_some() {
                            out.push(Shape::Fill([r[0], r[1] + 5.0, 2.0, (r[3] - 10.0) * t], solid));
                        }
                        let right = 62.0;
                        // The favourite star.
                        let fav = s.favorites.iter().any(|f| f == module.id);
                        let sr = [r[0] + r[2] - 52.0, r[1] + 5.0, 14.0, 14.0];
                        let st = hover(key(23, i), sr);
                        if fav || t > 0.05 {
                            let c = if fav { 0xFFF5_B83D } else { mix_argb(argb(GREY, 0.5 * t), argb(0xF5B83D, 0.8), st) };
                            star(&mut out, [sr[0] + 7.0, sr[1] + 7.0], 5.0 + st * 0.8, c);
                        }
                        let (name_c, desc_c) = if here.is_some() { (0xFFFF_FFFF, argb(MUTED, 1.0)) } else { (argb(GREY, 1.0), argb(GREY, 0.8)) };
                        label(&mut out, &fit(module.name, cw - right - 14.0, width), [r[0] + 9.0, r[1] + 3.5], name_c, false, 1.0);
                        label(&mut out, &fit(module.description, cw - right - 14.0, width), [r[0] + 9.0, r[1] + 13.5], desc_c, false, 1.0);
                        if risky(module.id) {
                            let nx = r[0] + 9.0 + width(&fit(module.name, cw - right - 14.0, width)) + 4.0;
                            out.push(Shape::Quad([[nx + 3.5, r[1] + 3.5], [nx + 7.0, r[1] + 10.0], [nx, r[1] + 10.0], [nx + 3.5, r[1] + 3.5]], 0xFFFF_C83D));
                        }
                        hits.push((r, Action::Open(i)));
                        hits.push((sr, Action::Favorite(i)));
                        match here {
                            Some((toggle, _)) => {
                                let on = ease(key(18, i), s.v[toggle] as f32, s.v[toggle] as f32, animate);
                                switch(&mut out, [r[0] + r[2] - 30.0, r[1] + 6.0], on, accent);
                                hits.push(([r[0] + r[2] - 36.0, r[1], 36.0, r[3]], Action::Toggle(i)));
                            }
                            None => {
                                let tag = "Java only";
                                let tr = [r[0] + r[2] - width(tag) - 16.0, r[1] + 6.0, width(tag) + 10.0, 12.0];
                                rounded(&mut out, tr, 4.0, 0x14FF_FFFF);
                                label(&mut out, tag, [tr[0] + 5.0, tr[1] + 2.5], argb(GREY, 1.0), false, 1.0);
                            }
                        }
                    }
                    if list.len() > shown {
                        let track = [cx + cw + 4.0, ly, 3.0, shown as f32 * ROW - 4.0];
                        rounded(&mut out, track, 1.5, 0x14FF_FFFF);
                        let part = shown as f32 / list.len() as f32;
                        let at = ease(key(19, m.category), first as f32 / list.len() as f32, 0.0, animate);
                        rounded(&mut out, [track[0], track[1] + track[3] * at, 3.0, (track[3] * part).max(8.0)], 1.5, solid);
                    }
                }
            }
        }
    }
    (out, hits, content)
}


/// Edits between `a` and `b` (Levenshtein).
fn edits(a: &[char], b: &[char]) -> usize {
    let mut row: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.iter().enumerate() {
        let mut prev = row[0];
        row[0] = i + 1;
        for (j, cb) in b.iter().enumerate() {
            let cur = row[j + 1];
            row[j + 1] = (prev + usize::from(ca != cb)).min(row[j] + 1).min(row[j + 1] + 1);
            prev = cur;
        }
    }
    row[b.len()]
}

/// How well `query` matches `text` (as the Java mod's search): 2 every word is in it, 1 with typos, 0 no match.
pub fn text_match(query: &str, text: &str) -> u8 {
    let text = text.to_lowercase();
    let tokens: Vec<Vec<char>> = text.split(|c: char| !c.is_alphanumeric()).filter(|t| !t.is_empty()).map(|t| t.chars().collect()).collect();
    let mut best = 2;
    for word in query.to_lowercase().split_whitespace() {
        if text.contains(word) {
            continue;
        }
        let w: Vec<char> = word.chars().collect();
        let allowed = match w.len() { 0..3 => 0, 3..8 => 1, _ => 2 };
        let close = allowed > 0 && tokens.iter().any(|t| {
            (w.len().saturating_sub(1)..=w.len() + 1).any(|n| n <= t.len() && n > 0 && edits(&w, &t[..n]) <= allowed)
        });
        if !close {
            return 0;
        }
        best = 1;
    }
    best
}

/// What a module is found by: its name, description, category, id and its settings' names.
fn haystack(i: usize) -> String {
    let j = &JAVA_MODULES[i];
    let mut out = format!("{} {} {} {}", j.name, j.description, CATEGORIES.get(j.category).copied().unwrap_or(""), j.id.replace('_', " "));
    if let Some((f, fields)) = on_horizon(j.id) {
        for x in std::iter::once(&f).chain(fields.iter()) {
            out.push(' ');
            out.push_str(FIELDS[*x].label);
        }
    }
    out
}

/// The modules `query` finds in every category: names first, then other matches, then typos.
pub fn search_modules(query: &str) -> Vec<usize> {
    let mut found: Vec<(u8, bool, String, usize)> = (0..JAVA_MODULES.len()).filter_map(|i| {
        let quality = text_match(query, &haystack(i));
        (quality > 0).then(|| {
            let rank = if text_match(query, JAVA_MODULES[i].name) == 2 { 0 } else if quality == 2 { 1 } else { 2 };
            (rank, on_horizon(JAVA_MODULES[i].id).is_none(), JAVA_MODULES[i].name.to_ascii_lowercase(), i)
        })
    }).collect();
    found.sort();
    found.into_iter().map(|f| f.3).collect()
}

/// The modules the list shows: the search's results, the favourites, or a category's.
pub fn shown_modules(m: &Menu, s: &Settings) -> Vec<usize> {
    if !m.search.trim().is_empty() {
        search_modules(m.search.trim())
    } else if m.category == FAVORITES {
        s.favorites.iter().filter_map(|f| JAVA_MODULES.iter().position(|j| j.id == f)).collect()
    } else if m.category == NEW_TAB {
        new_modules()
    } else {
        category_modules(m.category)
    }
    .into_iter().filter(|&i| !s.hide_risky || !risky(JAVA_MODULES[i].id)).collect()
}

/// A character typed on the menu: into the search box (any letter on the Modules page starts a search).
pub fn menu_typed(m: &mut Menu, c: char) -> bool {
    if m.binding.is_some() || c.is_control() {
        return false;
    }
    if m.page == Page::Profiles {
        if m.profile_name.chars().count() < 32 {
            m.profile_name.push(c);
        }
        m.name_focus = true;
        return true;
    }
    if m.page != Page::Modules || m.detail.is_some() {
        return false;
    }
    if m.search.chars().count() < 40 {
        m.search.push(c);
    }
    m.search_focus = true;
    m.scroll = 0;
    true
}

/// A key on the menu: Backspace and Escape in the search, Escape and Ctrl+Z in the HUD editor. True when it was the
/// menu's (Escape then doesn't close it). `s` changes on Undo (to be stored).
pub fn menu_key(m: &mut Menu, s: &mut Settings, key: &str, ctrl: bool) -> (bool, bool) {
    // Setting a module's key: the next key is it (Escape takes it off).
    if let Some(i) = m.binding.take() {
        let id = JAVA_MODULES[i].id;
        s.keys.retain(|(k, _)| k != id);
        if key != "key.keyboard.escape" {
            s.keys.push((id.to_owned(), key.to_owned()));
        }
        return (true, true);
    }
    match (m.page, key) {
        (Page::Profiles, "key.keyboard.backspace") if !m.profile_name.is_empty() => {
            m.profile_name.pop();
            (true, false)
        }
        (Page::Profiles, "key.keyboard.enter") if !m.profile_name.trim().is_empty() => (true, apply(Action::ProfileSave, m, s, &Fetch::Loading)),
        (Page::Profiles, "key.keyboard.escape") => {
            *m = Menu { page: Page::Home, tab: m.tab, ..Menu::default() };
            (true, false)
        }
        (Page::HudEdit, "key.keyboard.escape") => {
            *m = Menu { page: Page::Home, tab: m.tab, ..Menu::default() };
            (true, false)
        }
        (Page::HudEdit, "key.keyboard.z") if ctrl => (true, hud_undo(m, s)),
        (Page::Modules, "key.keyboard.backspace") if m.search_focus || !m.search.is_empty() => {
            if ctrl {
                m.search.clear();
            } else {
                m.search.pop();
            }
            m.scroll = 0;
            (true, false)
        }
        (Page::Modules, "key.keyboard.escape") if m.search_focus || !m.search.is_empty() => {
            m.search.clear();
            m.search_focus = false;
            (true, false)
        }
        (Page::Modules, "key.keyboard.escape") if m.detail.is_some() => {
            m.detail = None;
            (true, false)
        }
        _ => (false, false),
    }
}

// ---- The HUD editor ----

fn set_layout(s: &mut Settings, id: &str, v: Option<[f32; 5]>) {
    s.layout.retain(|(i, _)| i != id);
    if let Some(v) = v {
        s.layout.push((id.to_owned(), v));
    }
}

fn push_undo(m: &mut Menu, before: String) {
    if m.undo.last() != Some(&before) {
        m.undo.push(before);
        if m.undo.len() > 50 {
            m.undo.remove(0);
        }
    }
}

/// Ctrl+Z or Undo: the layout before the last change.
pub fn hud_undo(m: &mut Menu, s: &mut Settings) -> bool {
    match m.undo.pop() {
        Some(before) => {
            s.layout = parse_layout(&before);
            true
        }
        None => false,
    }
}

/// The HUD element under the pointer (the last drawn wins).
fn hud_at(boxes: &[(&'static str, [f32; 4], [f32; 4])], p: [f32; 2]) -> Option<(&'static str, [f32; 4], [f32; 4])> {
    boxes.iter().rev().find(|b| inside([b.1[0] - 2.0, b.1[1] - 2.0, b.1[2] + 4.0, b.1[3] + 4.0], p)).copied()
}

/// A press in the HUD editor: left picks up an element, right puts it back where it started. True when the layout changed.
pub fn hud_press(m: &mut Menu, s: &mut Settings, boxes: &[(&'static str, [f32; 4], [f32; 4])], mouse: [f32; 2], button: i32) -> bool {
    let Some((id, rect, _)) = hud_at(boxes, mouse) else {
        m.picked = None;
        return false;
    };
    m.picked = Some(id);
    if button == 1 {
        push_undo(m, layout_text(&s.layout));
        set_layout(s, id, None);
        return true;
    }
    m.drag = Some(Drag { id, grab: [mouse[0] - rect[0], mouse[1] - rect[1]], before: layout_text(&s.layout), moved: false });
    false
}

/// The pointer moved while holding an element: it follows, snapping (within 4 pixels) to the screen's edges and middle
/// and to the other elements' edges unless Shift is held. True when the layout changed.
pub fn hud_drag(m: &mut Menu, s: &mut Settings, boxes: &[(&'static str, [f32; 4], [f32; 4])], mouse: [f32; 2], screen: [f32; 2], shift: bool) -> bool {
    let Some(drag) = m.drag.clone() else { return false };
    let Some(&(_, rect, home)) = boxes.iter().find(|b| b.0 == drag.id) else { return false };
    let mut x = mouse[0] - drag.grab[0];
    let mut y = mouse[1] - drag.grab[1];
    let mut guides = Vec::new();
    if !shift {
        let others: Vec<[f32; 4]> = boxes.iter().filter(|b| b.0 != drag.id).map(|b| b.1).collect();
        let snap = |v: f32, size: f32, full: f32, edges: Vec<f32>| -> (f32, Option<f32>) {
            let mut cands = vec![(0.0, 0.0), (full - size, full), (full / 2.0 - size / 2.0, full / 2.0)];
            for e in edges {
                cands.push((e, e));
                cands.push((e - size, e));
            }
            cands.into_iter().map(|(c, line)| ((c - v).abs(), c, line)).filter(|c| c.0 <= 4.0)
                .min_by(|a, b| a.0.total_cmp(&b.0)).map_or((v, None), |c| (c.1, Some(c.2)))
        };
        let (nx, gx) = snap(x, rect[2], screen[0], others.iter().flat_map(|o| [o[0], o[0] + o[2]]).collect());
        let (ny, gy) = snap(y, rect[3], screen[1], others.iter().flat_map(|o| [o[1], o[1] + o[3]]).collect());
        x = nx;
        y = ny;
        if let Some(g) = gx {
            guides.push([g, 0.0, 1.0, screen[1]]);
        }
        if let Some(g) = gy {
            guides.push([0.0, g, screen[0], 1.0]);
        }
    }
    let x = x.clamp(0.0, (screen[0] - rect[2]).max(0.0));
    let y = y.clamp(0.0, (screen[1] - rect[3]).max(0.0));
    if !drag.moved {
        push_undo(m, drag.before.clone());
    }
    m.drag = Some(Drag { moved: true, ..drag });
    m.guides = guides;
    let _ = home;
    let scale = s.layout.iter().find(|(i, _)| i == drag.id).map_or(1.0, |e| e.1[4]);
    set_layout(s, drag.id, Some(anchored([x, y], [rect[2], rect[3]], scale, screen)));
    true
}

/// The button let go: the element stays.
pub fn hud_release(m: &mut Menu) {
    m.drag = None;
    m.guides.clear();
}

/// The wheel over an element sizes it (0.5 to 3, from its top left). True when the layout changed.
pub fn hud_scroll(m: &mut Menu, s: &mut Settings, boxes: &[(&'static str, [f32; 4], [f32; 4])], mouse: [f32; 2], steps: f64, screen: [f32; 2]) -> bool {
    let Some((id, rect, home)) = hud_at(boxes, mouse) else { return false };
    let now = s.layout.iter().find(|(i, _)| i == id).map_or(1.0, |e| e.1[4]);
    let scale = (((now + steps.signum() as f32 * 0.1) * 10.0).round() / 10.0).clamp(0.5, 3.0);
    if scale == now {
        return false;
    }
    push_undo(m, layout_text(&s.layout));
    m.picked = Some(id);
    // It keeps its top left; the anchor is worked out for the new size.
    set_layout(s, id, Some(anchored([rect[0], rect[1]], [home[2] * scale, home[3] * scale], scale, screen)));
    true
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
            *m = Menu { page, tab: m.tab, undo: std::mem::take(&mut m.undo), ..Menu::default() };
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
        Action::Favorite(i) => {
            let id = JAVA_MODULES[i].id;
            if let Some(at) = s.favorites.iter().position(|f| f == id) {
                s.favorites.remove(at);
            } else {
                s.favorites.push(id.to_owned());
            }
            true
        }
        Action::Search => {
            m.search_focus = true;
            false
        }
        Action::UndoHud => hud_undo(m, s),
        Action::HudTheme => {
            let Some(id) = m.picked else { return false };
            let now = s.hud_themes.iter().position(|(e, _)| e == id);
            let next = match now.and_then(|i| HUD_THEMES.iter().position(|t| t.0 == s.hud_themes[i].1)) {
                Some(t) if t + 1 < HUD_THEMES.len() => Some(HUD_THEMES[t + 1].0),
                Some(_) => None,
                None => Some(HUD_THEMES[0].0),
            };
            s.hud_themes.retain(|(e, _)| e != id);
            if let Some(t) = next {
                s.hud_themes.push((id.to_owned(), t.to_owned()));
            }
            true
        }
        Action::ServerLayout => {
            let Some(k) = current_server() else { return false };
            if let Some(at) = s.server_layouts.iter().position(|e| e.0 == k) {
                s.server_layouts.remove(at);
                s.layout = s.main_layout.clone();
            } else {
                s.main_layout = s.layout.clone();
                s.server_layouts.push((k, s.layout.clone()));
            }
            true
        }
        Action::BindKey(i) => {
            m.binding = Some(i);
            false
        }
        Action::ResetModule(i) => match on_horizon(JAVA_MODULES[i].id) {
            Some((f, fields)) => {
                let d = Settings::default();
                for x in std::iter::once(&f).chain(fields.iter()) {
                    s.v[*x] = d.v[*x];
                }
                s.keys.retain(|(id, _)| id != JAVA_MODULES[i].id);
                true
            }
            None => false,
        },
        Action::HideRisky => {
            s.hide_risky = !s.hide_risky;
            m.scroll = 0;
            true
        }
        Action::ProfileName => {
            m.name_focus = true;
            false
        }
        Action::ProfileSave => {
            let name = m.profile_name.trim().chars().take(32).collect::<String>();
            if name.is_empty() || (s.profiles.len() >= 20 && !s.profiles.iter().any(|p| p.0 == name)) {
                m.notice = Some(if name.is_empty() { "Type a name first.".into() } else { "20 profiles is the most: delete one first.".into() });
                return false;
            }
            let now = profile_now(s);
            match s.profiles.iter_mut().find(|p| p.0 == name) {
                Some(p) => p.1 = now,
                None => s.profiles.push((name.clone(), now)),
            }
            m.notice = Some(format!("Saved \"{name}\"."));
            m.profile_name.clear();
            m.name_focus = false;
            true
        }
        Action::ProfileLoad(i) => {
            let Some(name) = s.profiles.get(i).map(|p| p.0.clone()) else { return false };
            match load_profile(s, &name) {
                Some(loaded) => {
                    *s = loaded;
                    m.notice = Some(format!("Loaded \"{name}\"."));
                    true
                }
                None => false,
            }
        }
        Action::ProfileUpdate(i) => {
            let now = profile_now(s);
            match s.profiles.get_mut(i) {
                Some(p) => {
                    p.1 = now;
                    m.notice = Some(format!("Updated \"{}\".", p.0));
                    true
                }
                None => false,
            }
        }
        Action::ProfileDelete(i) => {
            let Some(name) = s.profiles.get(i).map(|p| p.0.clone()) else { return false };
            if m.confirm.as_deref() != Some(name.as_str()) {
                m.confirm = Some(name);
                return false;
            }
            m.confirm = None;
            s.profiles.remove(i);
            s.auto_profiles.retain(|(_, n)| *n != name);
            m.notice = Some(format!("Deleted \"{name}\"."));
            true
        }
        Action::ProfileAuto(i) => {
            let (Some(k), Some(name)) = (current_server(), s.profiles.get(i).map(|p| p.0.clone())) else { return false };
            let was = s.auto_profiles.iter().any(|(sk, n)| *sk == k && *n == name);
            s.auto_profiles.retain(|(sk, _)| *sk != k);
            if !was {
                s.auto_profiles.push((k.clone(), name.clone()));
            }
            m.notice = Some(if was { format!("{k} loads no profile now.") } else { format!("\"{name}\" loads whenever you join {k}.") });
            true
        }
        Action::CopyCode => {
            CLIP.with_borrow_mut(|c| *c = Some(Clip::Copy(share_code(s))));
            m.notice = Some("Share code copied.".into());
            false
        }
        Action::PasteCode => {
            CLIP.with_borrow_mut(|c| *c = Some(Clip::Paste));
            false
        }
        Action::ResetAll => {
            if m.confirm.as_deref() != Some("\u{0}reset") {
                m.confirm = Some("\u{0}reset".into());
                return false;
            }
            m.confirm = None;
            let keep = (s.profiles.clone(), s.auto_profiles.clone(), s.favorites.clone());
            *s = Settings::default();
            (s.profiles, s.auto_profiles, s.favorites) = keep;
            m.notice = Some("Every setting is back to its default.".into());
            true
        }
        Action::ResetHud => {
            if s.layout.is_empty() {
                return false;
            }
            push_undo(m, layout_text(&s.layout));
            s.layout.clear();
            true
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
pub fn menu_scroll(m: &mut Menu, s: &Settings, steps: f64, screen: [f32; 4], browse: &Fetch) {
    let step = -(steps.signum() as i64);
    match m.page {
        Page::Modules if m.detail.is_none() => {
            let max = shown_modules(m, s).len().saturating_sub(rows_shown(screen));
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
    fn version_1_13_modules() {
        let mut s = Settings::default();
        let l = looks(&s, Some("Steve"));
        assert_eq!((l.hotbar, l.swing, l.tag_bg, l.chat_keep), (0, 100, 0x3F00_0000, 100), "all off leaves Horizon's own");
        assert_eq!(l.chat, [None, None, None]);
        for f in [HOTBAR_STYLE, SWING_ANIM, NAME_HIDER, MENTIONS, CHAT_TWEAKS, BIG_ITEMS, SOUND_TWEAKS, SMALL_CRYSTALS] {
            s.v[f] = 1;
        }
        let l = looks(&s, Some("Steve"));
        assert!(l.hotbar >> 24 != 0 && l.hotbar_accent != 0);
        assert_eq!(l.swing, 150);
        assert_eq!(l.name_hide, Some(("Steve".into(), "You".into())));
        assert_eq!(l.mention, "steve");
        assert_eq!(l.chat, [Some(true), Some(true), None]);
        assert!(l.big_items.iter().any(|(i, _)| i == "minecraft:totem_of_undying"));
        assert_eq!(l.volumes[0], ("explode".into(), 0.5));
        assert_eq!(l.crystal_size, 50);
        s.v[CHAT_TWEAKS] = 0;
        assert_eq!(looks(&s, None).chat, [Some(false), Some(false), None], "turning off puts Horizon's back once");
        assert_eq!(looks(&s, None).chat, [None, None, None]);

        // Performance Mode leaves the stored settings alone.
        let mut raw = Settings::default();
        raw.v[BLOOM] = 1;
        raw.v[PERF] = 1;
        let p = performance(raw.clone());
        assert_eq!((p.v[BLOOM], p.v[PARTICLES], raw.v[BLOOM]), (0, 1, 1));
        let (t, value, message) = toggled(&raw, PERF);
        assert_eq!((t.v[PERF], value, message.as_str()), (0, "false", "Performance Mode: OFF"));

        // Quick Menu: hold, scroll, let go.
        let mut q = Settings::default();
        q.v[QUICK_MENU] = 1;
        assert_eq!(more_key(&q, "key_quick_menu", true, true, None), None);
        assert_eq!(more_key(&q, "key_quick_menu", false, true, None), None, "a tap switches nothing");
        assert_eq!(more_key(&q, "key_quick_menu", true, true, None), None);
        assert!(quick_wheel(-1.0) && quick_wheel(-1.0));
        assert_eq!(more_key(&q, "key_quick_menu", false, true, None), Some(QUICK[1]));
        assert!(!quick_wheel(1.0));
        assert_eq!(more_key(&q, "key_performance", true, true, None), Some(PERF));
        // Starred modules make the wheel, and the mouse points at one (up is the first, right the next quarter).
        q.favorites = vec!["zoom".into(), "fullbright".into(), "java_only_nope".into()];
        let list = quick_list(&q);
        assert!(!list.is_empty() && list.len() <= 2);
        assert_eq!(quick_pick(4, Some((0.0, -30.0)), 2), 0);
        assert_eq!(quick_pick(4, Some((30.0, 0.0)), 2), 1);
        assert_eq!(quick_pick(4, Some((1.0, 1.0)), 2), 2, "a small pull keeps the wheel's pick");

        // Notes and profiles.
        let mut n = Settings::default();
        n.v[NOTES] = 1;
        n.v[PROFILES] = 1;
        assert_eq!(server_key("note", "Play.Example.net:25565"), "note_play.example.net_25565");
        let none = |_: &str| None;
        assert_eq!(command(&n, &n, "note bring pearls", Some("a.net"), &none), Command::Store("note_a.net".into(), "bring pearls".into(), "Note kept for a.net.".into()));
        assert!(matches!(command(&n, &n, "note", None, &none), Command::Say(_)));
        let kept = profile_of(&n);
        assert!(profile_pairs(&kept).iter().any(|(k, v)| k == "server_notes" && v == "true"));
        assert!(profile_pairs("server_notes=true;nonsense=1").len() == 1);
        let stored = |k: &str| match k { "note_a.net" => Some("hi".to_string()), "profile_a.net" => Some("bloom=true".to_string()), _ => None };
        assert_eq!(server_tick(&n, Some("a.net"), &stored), vec![Joined::Profile("bloom=true".into()), Joined::Note("Note for a.net: hi".into())]);
        assert!(server_tick(&n, Some("a.net"), &stored).is_empty(), "only on joining");
        assert!(matches!(command(&n, &n, "crash", None, &none), Command::SendCrash));

        // Lyrics, Spotify's title, releases, kill effects, crash text and Party.
        let lrc = parse_lrc("[00:01.50] one\n[00:04.00] two\nnot a line");
        assert_eq!(lrc, vec![(1.5, "one".to_string()), (4.0, "two".to_string())]);
        assert_eq!(lyric_at(&lrc, 2.0), Some(("one", Some("two"))));
        assert_eq!(lyric_at(&lrc, 0.5), None);
        assert_eq!(song_of("Daft Punk - One More Time"), Some("Daft Punk - One More Time".into()));
        assert_eq!(song_of("Spotify Premium"), None);
        assert_eq!(release_version("2026-11-07-2ebf00a-arsenal-1.11.0"), Some("1.11.0".into()));
        assert_eq!(release_version("nightly"), None);
        assert!(newer("1.13.1", "1.13.0") && !newer("1.9.0", "1.13.0"));
        assert_eq!((effect_style("lightning"), effect_style("firework"), effect_style("soul_fire")), (2, 0, 1));
        assert_eq!(scrub(r"failed at D:\Games\x.rs here"), "failed at <path> here");
        assert_eq!(percent("AC/DC & me"), "AC%2FDC%20%26%20me");
        let friend = Other { id: 1, uuid: 0, name: "Alex".into(), pos: [0.0, 64.0, 10.0], health: 18.0, absorption: 0.0, hurt_time: 0, using: 0, using_off_hand: false, equipment: vec![] };
        let p = party(std::slice::from_ref(&friend), &["alex".into()], [0.0, 64.0, 0.0], 0.0);
        assert_eq!(p.len(), 1);
        assert!((p[0].2 - 10.0).abs() < 1e-6 && p[0].3.abs() < 1e-3, "straight ahead: {p:?}");
        assert!(route(&Settings::default(), "hi".into()).is_some());
    }

    #[test]
    fn the_menu_remake_draws_round_and_wraps() {
        let mut out = Vec::new();
        rounded(&mut out, [0.0, 0.0, 20.0, 10.0], 3.0, 0x80FF_FFFF);
        let area: f32 = out.iter().map(|s| if let Shape::Fill(r, _) = s { r[2] * r[3] } else { 0.0 }).sum();
        assert!(area < 200.0 && area > 185.0, "corners cut, nothing drawn twice: {area}");
        assert_eq!(mix_argb(0x0000_0000, 0xFFFF_FFFF, 0.5), 0x8080_8080);
        assert_eq!(argb(0x123456, 1.0), 0xFF12_3456);
        let lines = wrap("one two three four five six", 6.0 * 9.0, 2, &width);
        assert_eq!(lines.len(), 2);
        assert!(lines[1].ends_with("...") || lines[1].split(' ').count() <= 2);
        // A frame of every page has the same clicks as its layout, and fades nothing out once settled.
        let screen = [0.0, 0.0, 640.0, 360.0];
        let s = Settings::default();
        for page in [Page::Home, Page::Modules, Page::Browse, Page::Cosmetics, Page::HudEdit] {
            let m = Menu { page, ..Menu::default() };
            let frame = menu_frame(screen, [0.0, 0.0], &m, &s, &Fetch::Loading, &width);
            let (layout, _) = menu_layout(screen, [0.0, 0.0], &m, &s, &Fetch::Loading, &width);
            assert_eq!(frame.len(), layout.len());
        }
    }

    #[test]
    fn search_favorites_and_the_hud_editor() {
        assert_eq!(text_match("zoom", "Zoom: hold a key"), 2);
        assert_eq!(text_match("zom", "Zoom: hold a key"), 1, "a typo");
        assert_eq!(text_match("totem pop", "Totem Pop Counter"), 2);
        assert_eq!(text_match("lightning", "Zoom"), 0);
        let found = search_modules("totem");
        assert!(!found.is_empty() && JAVA_MODULES[found[0]].name.to_lowercase().contains("totem"));
        let mut m = Menu { page: Page::Modules, ..Menu::default() };
        let mut s = Settings::default();
        assert!(menu_typed(&mut m, 'z') && menu_typed(&mut m, 'o'));
        assert_eq!(m.search, "zo");
        assert_eq!(menu_key(&mut m, &mut s, "key.keyboard.backspace", false), (true, false));
        assert_eq!(m.search, "z");
        assert_eq!(menu_key(&mut m, &mut s, "key.keyboard.escape", false), (true, false));
        assert!(m.search.is_empty() && !m.search_focus);
        assert_eq!(menu_key(&mut m, &mut s, "key.keyboard.escape", false), (false, false), "then Escape closes the menu");
        // Starring: the Favorites row lists it.
        let zoom = JAVA_MODULES.iter().position(|j| j.id == "zoom").unwrap();
        assert!(apply(Action::Favorite(zoom), &mut m, &mut s, &Fetch::Loading));
        m.category = FAVORITES;
        assert_eq!(shown_modules(&m, &s), vec![zoom]);
        assert!(apply(Action::Favorite(zoom), &mut m, &mut s, &Fetch::Loading));
        assert!(shown_modules(&m, &s).is_empty());
        let screen = [0.0, 0.0, 640.0, 360.0];
        let (_, hits) = menu_layout(screen, [0.0, 0.0], &Menu { page: Page::Modules, ..Menu::default() }, &s, &Fetch::Loading, &width);
        assert!(hits.iter().any(|(_, a)| matches!(a, Action::Favorite(_))) && hits.iter().any(|(_, a)| *a == Action::Search));
        // The editor: drag snaps to the middle, right-click puts it back, Undo brings the move back.
        let boxes = vec![("armor", [100.0, 50.0, 20.0, 20.0], [100.0, 50.0, 20.0, 20.0])];
        let mut e = Menu { page: Page::HudEdit, ..Menu::default() };
        assert!(!hud_press(&mut e, &mut s, &boxes, [105.0, 55.0], 0));
        assert!(hud_drag(&mut e, &mut s, &boxes, [312.0, 55.0], [640.0, 360.0], false));
        assert_eq!(s.layout, vec![("armor".to_string(), [1.0, 0.0, 0.0, 50.0, 1.0])], "snapped to the middle (310 = 320 - 10), anchored there");
        assert_eq!(e.guides.len(), 1);
        hud_release(&mut e);
        let moved = vec![("armor", [310.0, 50.0, 20.0, 20.0], [100.0, 50.0, 20.0, 20.0])];
        assert!(hud_scroll(&mut e, &mut s, &moved, [315.0, 55.0], 1.0, [640.0, 360.0]));
        assert_eq!(s.layout[0].1[4], 1.1);
        assert_eq!(s.layout[0].1[2], 1.0, "it keeps its top left as it grows");
        assert!(hud_press(&mut e, &mut s, &moved, [315.0, 55.0], 1));
        assert!(s.layout.is_empty());
        assert!(hud_undo(&mut e, &mut s));
        assert_eq!(s.layout[0].1[4], 1.1);
        let (shapes, hits) = menu_layout(screen, [0.0, 0.0], &e, &s, &Fetch::Loading, &width);
        assert!(hits.iter().any(|(_, a)| *a == Action::Go(Page::Home)) && !shapes.is_empty());
        assert_eq!(menu_key(&mut e, &mut s, "key.keyboard.escape", false), (true, false));
        assert_eq!(e.page, Page::Home);
    }

    #[test]
    fn hud_elements_move_and_stay_on_screen() {
        let mut s = Settings::default();
        assert_eq!(parse_layout("armor=2,1,10,-5,1.5;nope=1,1,1,1,1;potions=a,b,c,d,e;old=1,2,3"), vec![("armor".to_string(), [2.0, 1.0, 10.0, -5.0, 1.5])]);
        assert_eq!(layout_text(&parse_layout("armor=2,1,10,-5,1.5")), "armor=2,1,10,-5,1.5");
        s.layout = vec![("armor".into(), [0.0, 0.0, -1000.0, 50.0, 2.0])];
        let mut out = vec![Shape::Fill([0.0; 4], 0), Shape::Fill([100.0, 50.0, 10.0, 10.0], 0xFF00_0000), Shape::Item("x".into(), [100.0, 50.0])];
        hud_begin();
        place(&s, &mut out, 1, "armor", 320.0, 200.0, &width);
        assert_eq!(out[1], Shape::Fill([0.0, 50.0, 20.0, 20.0], 0xFF00_0000), "scaled from its corner and kept on screen");
        assert_eq!(out[2], Shape::Icon("x".into(), [0.0, 50.0], 32.0));
        assert_eq!(hud_boxes(), vec![("armor", [0.0, 50.0, 32.0, 32.0], [100.0, 50.0, 16.0, 16.0])]);
        let mut raw = Settings::default();
        raw.favorites = vec!["zoom".into()];
        raw.layout = vec![("clock".into(), [0.0, 0.0, 3.0, 4.0, 1.0])];
        let stored = raw.stored();
        let back = Settings::read(|k| stored.iter().find(|(key, _)| *key == k).map(|(_, v)| v.clone()));
        assert_eq!((back.favorites, back.layout), (raw.favorites, raw.layout));
    }

    #[test]
    fn skyblock_album_and_cloud() {
        let sidebar: Vec<String> = ["§e§lSKYBLOCK §b§lCO-OP", "Early Spring 3rd", "10:20pm ☽", " ⏣ §bVillage", "", "Purse: §61,234,567.8", "Bits: §b2,500",
            "Objective", "Talk to Jerry", "Slayer Quest", "Revenant Horror IV", "(1,234/2,400) Combat XP"].iter().map(|s| s.to_string()).collect();
        let tab: Vec<String> = ["Pet:\n[Lvl 100] Golden Dragon", "Crypts: 6", "Cookie Buff\n3d 4h 5m", "God Potion: 1h 30m"].iter().map(|s| s.to_string()).collect();
        let sb = read_skyblock(&sidebar, &tab).unwrap();
        assert_eq!(sb.area, "Village");
        assert_eq!(sb.purse, Some(("Purse".into(), 1_234_567.8)));
        assert_eq!(sb.bits, Some(2500.0));
        assert_eq!((sb.date.as_str(), sb.time.as_str()), ("Early Spring 3rd", "10:20pm"));
        assert_eq!(sb.objective, "Talk to Jerry");
        assert_eq!(sb.slayer, Some(("Revenant Horror IV".into(), "(1,234/2,400) Combat XP".into())));
        assert_eq!(sb.pet, "[Lvl 100] Golden Dragon");
        assert_eq!(sb.crypts, Some(6));
        assert_eq!(buff_times(&tab), (Some(3 * 86400 + 4 * 3600 + 5 * 60), Some(5400)));
        assert!(read_skyblock(&["Bedwars".to_string()], &[]).is_none());
        assert!(on_hypixel(Some("mc.hypixel.net:25565")) && on_hypixel(Some("hypixel.net")) && !on_hypixel(Some("nothypixel.net")));
        assert_eq!((coins(1.25e9), coins(4_560_000.0), coins(12_345.0), coins(1234.0), coins(12.5)), ("1.2B".into(), "4.56M".into(), "12.3k".into(), "1,234".into(), "12.5".into()));
        assert_eq!(sb_duration("12:34"), Some(754));
        assert_eq!(sb_clean("§aHi  §bthere ⚽"), "Hi there");
        let mut s = Settings::default();
        s.v[SKYBLOCK] = 1;
        assert!(skyblock_tick(&s, Some("mc.hypixel.net"), &sidebar, &tab).is_none());
        skyblock_chat(&["This ability is on cooldown for 3.5s.".to_string()]);
        let shapes = skyblock_shapes(&s, 640.0, 360.0, &width);
        assert!(shapes.iter().any(|x| matches!(x, Shape::Text(t, ..) if t.starts_with("Purse: 1.23M"))));
        assert!(shapes.iter().any(|x| matches!(x, Shape::Text(t, ..) if t.starts_with("Ability: "))));
        assert!(chat_hidden(&s).iter().any(|w| w == "kill combo"), "the spam filter in SkyBlock");
        // Album Colors: a red cover makes a red accent; a grey one none.
        let red: Vec<u8> = (0..256).flat_map(|_| [220u8, 30, 40]).collect();
        let (accent, sky) = album_palette(&red).unwrap();
        assert!((accent >> 16) & 255 > 200 && (accent & 255) < 160, "{accent:06X}");
        assert!(sky != 0);
        let grey: Vec<u8> = (0..256).flat_map(|_| [128u8, 128, 128]).collect();
        assert!(album_palette(&grey).is_none());
        let m = parse_media(r#"{"title":"One","artist":"Daft Punk","album":"x","durationMs":1,"positionMs":5000,"playing":true,"art":"AAAA"}"#).unwrap();
        assert_eq!((m.title.as_str(), m.position_ms, m.playing, m.art.len()), ("One", 5000, true, 3));
        assert!(parse_media(r#"{"error":"winrt"}"#).is_none());
        // Cloud Backup: the Java setup stays, Horizon's goes next to it; a restore prefers Horizon's, else the Java modules.
        let java = r#"{"configVersion":1,"modules":{"zoom":{"enabled":true},"nope":{"enabled":true}},"friends":{"a":1}}"#;
        let mut raw = Settings::default();
        raw.v[BLOOM] = 1;
        let merged = merged_backup(Some(java), &raw);
        let v: serde_json::Value = serde_json::from_str(&merged).unwrap();
        assert_eq!(v["friends"]["a"], 1);
        assert_eq!(v["horizon"]["settings"]["bloom"], "true");
        assert!(restored_from(&merged).contains(&("bloom".to_string(), "true".to_string())));
        assert_eq!(restored_from(java), vec![(FIELDS[on_horizon("zoom").unwrap().0].key.to_string(), "true".to_string())]);
        let none = |_: &str, _: &[String]| None;
        cloud_ask(true);
        let out = cloud_tick(&raw, &raw, &none);
        assert!(out.say.unwrap().contains("Minecraft account"));
        assert_eq!(command(&raw, &raw, "cloud save", None, &|_| None), Command::Cloud(true));
    }

    #[test]
    fn keys_profiles_codes_and_servers() {
        assert_eq!(server_of("EU.Play.Example.NET:25565"), "example.net");
        assert_eq!(server_of("mc.foo.co.uk"), "foo.co.uk");
        assert_eq!(server_of("1.2.3.4"), "1.2.3.4");
        assert_eq!(server_of("play.x.net:25566"), "play.x.net:25566");
        assert_eq!(key_name("key.keyboard.left.shift"), "Left Shift");
        // A module's key.
        let mut m = Menu { page: Page::Modules, ..Menu::default() };
        let mut s = Settings::default();
        let zoom = JAVA_MODULES.iter().position(|j| j.id == "zoom").unwrap();
        apply(Action::BindKey(zoom), &mut m, &mut s, &Fetch::Loading);
        assert_eq!(menu_key(&mut m, &mut s, "key.keyboard.h", false), (true, true));
        assert_eq!(module_key(&s, "key.keyboard.h").map(|e| e.1), Some(JAVA_MODULES[zoom].name));
        let fb = JAVA_MODULES.iter().position(|j| j.id == "fullbright").unwrap();
        apply(Action::BindKey(fb), &mut m, &mut s, &Fetch::Loading);
        menu_key(&mut m, &mut s, "key.keyboard.h", false);
        assert_eq!(key_clash(&s, "zoom"), vec![JAVA_MODULES[fb].name]);
        // Profiles: save, change, load back; delete asks twice.
        let mut p = Menu { page: Page::Profiles, ..Menu::default() };
        for c in "PvP".chars() {
            menu_typed(&mut p, c);
        }
        s.v[BLOOM] = 1;
        assert!(apply(Action::ProfileSave, &mut p, &mut s, &Fetch::Loading));
        s.v[BLOOM] = 0;
        assert!(apply(Action::ProfileLoad(0), &mut p, &mut s, &Fetch::Loading));
        assert_eq!((s.v[BLOOM], s.profiles.len()), (1, 1));
        assert!(!apply(Action::ProfileDelete(0), &mut p, &mut s, &Fetch::Loading));
        assert!(apply(Action::ProfileDelete(0), &mut p, &mut s, &Fetch::Loading));
        assert!(s.profiles.is_empty());
        // Settings keep all of it.
        s.profiles = vec![("A".into(), vec![("bloom".into(), "true".into())])];
        s.hud_themes = vec![("armor".into(), "NEON".into())];
        s.hide_risky = true;
        let stored = s.stored();
        let back = Settings::read(|k| stored.iter().find(|(key, _)| *key == k).map(|(_, v)| v.clone()));
        assert_eq!((back.keys, back.profiles, back.hud_themes, back.hide_risky), (s.keys.clone(), s.profiles.clone(), s.hud_themes.clone(), true));
        // Hide risky leaves Free Look out.
        let free = JAVA_MODULES.iter().position(|j| j.id == "free_look").unwrap();
        let all = Menu { search: "free look".into(), ..Menu::default() };
        assert!(!shown_modules(&all, &s).contains(&free));
        assert!(!new_modules().is_empty());
        // Share codes round trip, and a Java code's module switches load.
        let mut raw = Settings::default();
        raw.v[MIST] = 1;
        let code = share_code(&raw);
        assert!(code.starts_with("A2:"));
        let pairs = read_share_code(&code).unwrap();
        assert!(pairs.contains(&("mist".to_string(), "true".to_string())));
        let java = r#"{"modules":{"zoom":{"enabled":false,"favorite":true,"key":"key.keyboard.c"}},"onlyChanges":true}"#;
        let pairs = read_share_code(java).unwrap();
        assert!(pairs.contains(&("favorites".to_string(), "zoom".to_string())) && pairs.contains(&("module_keys".to_string(), "zoom=key.keyboard.c".to_string())));
        assert!(read_share_code("hello").is_none());
        // A theme for one element.
        let mut t = Settings::default();
        t.hud_themes = vec![("armor".into(), "NEON".into())];
        let bg = t.panel_colors().0;
        let mut shapes = vec![Shape::Fill([0.0, 0.0, 5.0, 5.0], bg)];
        theme_element(&t, &mut shapes, "armor");
        assert_eq!(shapes[0], Shape::Fill([0.0, 0.0, 5.0, 5.0], 0xB005_0510));
        // A server's own layout.
        set_server(Some("play.example.net"));
        let mut e = Menu { page: Page::HudEdit, picked: Some("armor"), ..Menu::default() };
        let mut h = Settings::default();
        assert!(apply(Action::ServerLayout, &mut e, &mut h, &Fetch::Loading));
        assert_eq!(h.server_layouts.len(), 1);
        assert!(apply(Action::HudTheme, &mut e, &mut h, &Fetch::Loading));
        assert_eq!(h.hud_themes, vec![("armor".to_string(), "CLASSIC".to_string())]);
        set_server(None);
    }

    #[test]
    fn cosmetics_on_players() {
        assert_eq!(blade_model("butterfly_knife").as_deref(), Some("arsenal:blade_butterfly"));
        assert_eq!(blade_model("katana").as_deref(), Some("arsenal:blade_katana"));
        assert!(blade_model("nope").is_none());
        let own = own_wear(r#"{"equipped":"galaxy","equippedBlade":"katana","equippedHat":"crown"}"#);
        assert_eq!(own, Wear { cape: Some("galaxy".into()), blade: Some("katana".into()), hat: Some("crown".into()), accessory: None, pet: None });
        assert_eq!(own_wear(r#"{"equipped":"made_up"}"#), Wear::default());
        let h = vec![(7u128, "aa".to_string()), (8, "bb".to_string())];
        let others = others_wear(r#"{"capes":{"aa":"lava"},"blades":{"bb":"kunai"}}"#, &h);
        assert_eq!(others[0].1.cape.as_deref(), Some("lava"));
        assert_eq!(others[1].1.blade.as_deref(), Some("kunai"));
        let s = Settings::default();
        assert_eq!(blade_pose(&s, true, true), Some((1.0, 0.0)));
        assert_eq!(blade_pose(&s, false, true), None);
        let d = dressing(&s, Some(1), std::path::Path::new("nowhere"), std::path::Path::new("nowhere"));
        assert!(d.capes.is_empty() && !d.pack);
        assert!(CAPE_IDS.len() >= 54 && BLADE_IDS.len() == 36);
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
        assert!(shapes.iter().any(|sh| matches!(sh, Shape::Styled(t, ..) if t == "ARSENAL")));
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
        // Every Java module runs on Horizon now; one that didn't would switch nothing but say why.
        assert!((0..JAVA_MODULES.len()).all(|i| on_horizon(JAVA_MODULES[i].id).is_some()), "all 149 modules on Horizon");
        let _ = &hits;
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
        let s = Settings::default();
        let mut m = Menu { page: Page::Modules, ..Menu::default() };
        let len = category_modules(0).len();
        let max = len.saturating_sub(rows_shown(screen));
        menu_scroll(&mut m, &s, 1.0, screen, &Fetch::Loading);
        assert_eq!(m.scroll, 0);
        for _ in 0..200 {
            menu_scroll(&mut m, &s, -1.0, screen, &Fetch::Loading);
        }
        assert_eq!(m.scroll, max);
        assert_eq!(fit("A long description", 60.0, &width), "A long...");
    }
}
