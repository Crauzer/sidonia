#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R3dVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Into<[f32; 3]> for R3dVector3 {
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}
