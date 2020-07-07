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

impl From<[f32; 3]> for R3dVector3 {
    fn from(value: [f32; 3]) -> Self {
        R3dVector3 {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}
