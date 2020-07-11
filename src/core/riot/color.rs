#[repr(C)]
#[derive(Copy, Clone)]
pub struct RiotColorValue {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl From<[u8; 4]> for RiotColorValue {
    fn from(rgba: [u8; 4]) -> Self {
        RiotColorValue {
            r: rgba[0] as f32 / 255.0,
            g: rgba[0] as f32 / 255.0,
            b: rgba[0] as f32 / 255.0,
            a: rgba[0] as f32 / 255.0
        }
    }
}

impl From<[f32; 4]> for RiotColorValue {
    fn from(rgba: [f32; 4]) -> Self {
        RiotColorValue {
            r: rgba[0],
            g: rgba[0],
            b: rgba[0],
            a: rgba[0]
        }
    }
}

impl Into<[u8; 4]> for RiotColorValue {
    fn into(self) -> [u8; 4] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8
        ]
    }
}

impl Into<[f32; 4]> for RiotColorValue {
    fn into(self) -> [f32; 4] {
        [
            self.r,
            self.g,
            self.b,
            self.a
        ]
    }
}
