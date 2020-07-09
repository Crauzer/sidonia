use crate::core::{
    msvc::map::StdMap,
    riot::{
        ai_hero::RiotAiHero,
        audio::vo::{
            component::{RiotAudioIVoComponent, RiotAudioVoComponentEventType},
            meta_tags::RiotAudioVoMetaTags,
        },
        net::RiotOnNetworkRegister,
    },
};

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
    vo_meta_tags: RiotAudioVoMetaTags,
}

#[repr(C)]
pub struct RiotHeroVoComponentPingEvent {
    command: RiotAudioVoComponentEventType,
    timestamp: f32,
}
