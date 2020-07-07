use glam::Mat4;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R3dMatrix44 {
    matrix: [[f32; 4]; 4],
}

impl Into<glam::Mat4> for R3dMatrix44 {
    fn into(self) -> Mat4 {
        Mat4::from_cols_array_2d(&self.matrix).transpose()
    }
}

impl From<glam::Mat4> for R3dMatrix44 {
    fn from(matrix: Mat4) -> Self {
        R3dMatrix44::from(matrix.transpose().to_cols_array_2d())
    }
}

impl From<[[f32; 4]; 4]> for R3dMatrix44 {
    fn from(array: [[f32; 4]; 4]) -> Self {
        R3dMatrix44 { matrix: array }
    }
}
