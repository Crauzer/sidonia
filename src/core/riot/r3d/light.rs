use winapi::shared::minwindef::LPVOID;

use crate::core::riot::{
    box3d::RiotBox3D,
    r3d::{texture::R3dTexture, vector3::R3dVector3},
};

#[repr(C)]
#[derive(Debug)]
pub struct R3dLight {
    pub position: R3dVector3,
    flags: u32,
    next: *mut R3dLight,
    bounding_box: RiotBox3D,
    pub intensity: f32,
    light_type: u32,
    pub radius1: f32,
    pub radius2: f32,
    r_tmp: f32,
    light_system: LPVOID,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub r2: f32,
    pub g2: f32,
    pub b2: f32,
    pub direction: R3dVector3,
    pub spot_angle: f32,
    pub falloff_angle: f32,
    pub attribute1: u32,
    pub attribute2: u32,
    pub attribute3: u32,
    is_local_light: i32,
    pub shadow_index: i32,
    pub update_key: i32,
    project_map: *mut R3dTexture,
}

impl R3dLight {
    pub fn flags(&self) -> u32 {
        self.flags
    }
    pub fn boounding_box(&self) -> RiotBox3D {
        self.bounding_box
    }
    pub fn light_type(&self) -> u32 {
        self.light_type
    }
    pub fn r_tmp(&self) -> f32 {
        self.r_tmp
    }
    pub fn light_system(&self) -> LPVOID {
        self.light_system
    }
    pub fn is_local_light(&self) -> bool {
        self.is_local_light != 0
    }
    pub fn project_map(&self) -> Option<&'static R3dTexture> {
        unsafe { self.project_map.as_ref::<'static>() }
    }
}
