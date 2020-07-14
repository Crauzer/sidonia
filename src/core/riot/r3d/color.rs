#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R3dColor {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

impl R3dColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        R3dColor { b, g, r, a }
    }

    pub fn from_rgba(array: [f32; 4]) -> Self {
        R3dColor {
            r: (array[0] * 255.0) as u8,
            g: (array[1] * 255.0) as u8,
            b: (array[2] * 255.0) as u8,
            a: (array[3] * 255.0) as u8,
        }
    }
}
