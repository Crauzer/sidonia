use std::mem;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R3dVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl R3dVector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        R3dVector3 { x, y, z }
    }

    pub fn zero() -> Self {
        R3dVector3 { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Into<[f32; 3]> for R3dVector3 {
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl From<[f32; 3]> for R3dVector3 {
    fn from(value: [f32; 3]) -> Self {
        R3dVector3 {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl AsRef<[f32; 3]> for R3dVector3 {
    fn as_ref(&self) -> &[f32; 3] {
        unsafe { mem::transmute(self) }
    }
}

impl AsMut<[f32; 3]> for R3dVector3 {
    fn as_mut(&mut self) -> &mut [f32; 3] {
        unsafe { mem::transmute(self) }
    }
}
