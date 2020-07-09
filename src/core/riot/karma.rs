use crate::core::msvc::map::StdMap;

#[repr(C)]
pub struct RiotKarma {
    karma_values: StdMap<i32, f32>,
    karma_decay: StdMap<i32, f32>,
}
