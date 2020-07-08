use crate::core::msvc::unordered_map::StdUnorderedMap;
use crate::core::msvc::string::StdString;

pub mod camera_logic;
pub mod game_clock;
pub mod hud_manager;
pub mod logger;
pub mod metric_stats_bucker_generator;
pub mod metric_stats_generator;
pub mod metrics;
pub mod metrics_details;
pub mod mutex;
pub mod net;
pub mod r3d;
pub mod render_pipeline;
pub mod x3d;
pub mod ai_hero;
pub mod karma;
pub mod evolution;
pub mod replication;
pub mod experience;
pub mod position_owner;
pub mod hero_vo_component;
pub mod audio;
pub mod contextual_emote_component;
pub mod region;
pub mod game_object;
pub mod avatar;
pub mod spellbook;
pub mod talent;
pub mod event_system;
pub mod timer;
pub mod issue_order;

#[repr(C)]
pub struct RiotMetadata {
    tag_map: StdUnorderedMap<StdString, StdString> // Riot::std_hash_map
}

#[repr(u32)]
#[derive(Debug)]
pub enum RiotTeam {
    All = 0xFFFFFF9C,
    Unknown = 0x0,
    Order = 0x64,
    Chaos = 0xC8,
    Neutral = 0x12C,
    Max = 0x190,
}