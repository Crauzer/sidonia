use super::evo_response::RiotAudioVoEvoRespone;
use crate::core::{msvc::map::StdMap, riot::audio::vo::component::RiotAudioVoComponentEventType};

#[repr(C)]
pub struct RiotAudioVoMetaTags {
    evo_responses: StdMap<RiotAudioVoComponentEventType, RiotAudioVoEvoRespone>, // TODO: multimap ?
                                                                                 //std::__1::multimap<
                                                                                 // Audio::VO::ComponentEvent,
                                                                                 // Audio::VO::EVOResponse,
                                                                                 // std::__1::less<Audio::VO::ComponentEvent>,
                                                                                 // std::__1::allocator<std::__1::pair<const Audio::VO::ComponentEvent,Audio::VO::EVOResponse> > >
                                                                                 // Audio::VO::EVOResponseMap_t;
}
