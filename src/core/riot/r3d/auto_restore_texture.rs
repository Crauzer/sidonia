use crate::core::riot::r3d::texture::R3dTexture;

pub type R3dAutoRestoreTextureSet = [R3dAutoRestoreTexture; 16];

pub struct R3dAutoRestoreTexture {
    stage: u32,
    texture: *mut R3dTexture,
}
