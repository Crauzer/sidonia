use crate::core::riot::replication::RiotReplicate;
use crate::core::riot::position_owner::RiotIPositionOwner;
use crate::core::msvc::vector::StdVector;
use winapi::shared::minwindef::LPVOID;

#[repr(C)]
#[derive(Debug)]
pub struct RiotIExperienceOwner {
    vtable: LPVOID
}

#[repr(C)]
#[derive(Debug)]
pub struct RiotExperience {
    experience: RiotReplicate<f32>,
    level: RiotReplicate<i32>,
    owner: *mut RiotIPositionOwner,
    experience_needed_per_level: StdVector<f32>,
    experience_callback: *mut RiotIExperienceOwner,
    spell_training_points: u32, // TODO: T: RiotSpellTrainingPoints
    level_cap: i32,
    base_experience_multiple: f32,
    level_difference_experience_multiple: f32,
    minimum_exp_multiple: f32
}