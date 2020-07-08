use crate::core::riot::ai_hero::RiotAiHero;
use crate::core::msvc::vector::StdVector;
use crate::core::msvc::string::StdString;

#[repr(C)]
pub struct RiotContextualEmoteComponent {
    owner: *mut RiotAiHero,
    emote_play_data: [StdVector<RiotContextualEmotePlayData>; 21] // per-category
}

#[repr(C)]
pub struct RiotContextualEmotePlayData {
    emote_type: RiotContextualEmoteType,
    parameter: StdString,
    hashed_parameter: u32,
    animation_name: StdString,
    vo_event_name: StdString
}

#[repr(u32)]
pub enum RiotContextualEmoteType {
    DefaultRun = 0x0,
    DefaultIdle = 0x1,
    Flee = 0x2,
    Chase = 0x3,
    GameStart = 0x4,
    KilledChampionGeneric = 0x5,
    KilledChampionMultikill = 0x6,
    KilledChampionSpecific = 0x7,
    KilledNeutralMinion = 0x8,
    KilledTurret = 0x9,
    ProximityFriendlyMinion = 0xA,
    ProximityEnemyMinion = 0xB,
    ProximityNeutralMinion = 0xC,
    ProximityTurret = 0xD,
    LeaveCombatLowHealth = 0xE,
    InBrush = 0xF,
    ChampionSpell1 = 0x10,
    ChampionSpell2 = 0x11,
    ChampionSpell3 = 0x12,
    ChampionSpell4 = 0x13,
    CustomEvent = 0x14,
}