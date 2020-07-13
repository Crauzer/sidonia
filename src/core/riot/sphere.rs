use crate::core::riot::r3d::vector3::R3dVector3;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RiotSphere {
    pub center: R3dVector3,
    pub radius: f32,
}
