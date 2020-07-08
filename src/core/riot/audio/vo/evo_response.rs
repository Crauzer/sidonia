use crate::core::msvc::string::StdString;
use crate::core::riot::RiotMetadata;

#[repr(C)]
pub struct RiotAudioVoEvoRespone {
    meta_flags: RiotMetadata,
    response: StdString
}