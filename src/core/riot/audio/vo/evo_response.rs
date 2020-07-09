use crate::core::{msvc::string::StdString, riot::RiotMetadata};

#[repr(C)]
pub struct RiotAudioVoEvoRespone {
    meta_flags: RiotMetadata,
    response: StdString,
}
