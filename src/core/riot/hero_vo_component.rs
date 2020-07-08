use crate::core::riot::net::RiotOnNetworkRegister;
use crate::core::riot::ai_hero::RiotAiHero;
use crate::core::msvc::map::StdMap;
use crate::core::riot::audio::vo::component::{RiotAudioIVoComponent, RiotAudioVoComponentEventType};
use crate::core::riot::audio::vo::meta_tags::RiotAudioVoMetaTags;

#[repr(C)]
pub struct RiotHeroVoComponent {
    vo_component_interface: RiotAudioIVoComponent,
    on_network_register: RiotOnNetworkRegister<!>,
    parent: *mut RiotAiHero,
    event_chances: StdMap<u32, f32>,
    last_ping_event: RiotHeroVoComponentPingEvent,
    event_last_timestamps: StdMap<u32, f32>,
    event_cooldowns: StdMap<u32, f32>,
    last_event_finished_timestamp: f32,
    vo_meta_tags: RiotAudioVoMetaTags
}

#[repr(C)]
pub struct RiotHeroVoComponentPingEvent {
    command: RiotAudioVoComponentEventType,
    timestamp: f32
}