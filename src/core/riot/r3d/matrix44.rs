use glam::Mat4;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R3dMatrix44 {
    matrix: [[f32; 4]; 4],
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

impl From<R3dMatrix44> for glam::Mat4 {
    fn from(matrix: R3dMatrix44) -> Self {
        Mat4::from_cols_array_2d(&matrix.matrix).transpose()
    }
}
