use crate::core::riot::r3d::vector3::R3dVector3;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RiotBox3D {
    pub min: R3dVector3,
    pub max: R3dVector3,
}

impl RiotBox3D {
    pub fn new(min: R3dVector3, max: R3dVector3) -> Self {
        RiotBox3D { min, max }
    }
}
