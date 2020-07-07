use crate::core::riot::r3d::vector3::R3dVector3;

#[repr(C)]
#[derive(Debug)]
pub struct R3dCamera {
    position: R3dVector3,
    near_clip: f32,
    far_clip: f32,
    point_to: R3dVector3,
    up: R3dVector3,
    fov: f32,
    camera_type: u32,
    width: f32,
    height: f32,
    flags: u32,
}
