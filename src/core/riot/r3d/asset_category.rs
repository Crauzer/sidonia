use crate::core::msvc::string::StdString;
use crate::core::msvc::unordered_set::StdUnorderedSet;
use crate::core::riot::mutex::RiotMutex;
use crate::core::riot::r3d::texture::R3dTexture;
use crate::core::riot::x3d::vertex_buffer::X3dIVertexBuffer;
use crate::core::riot::x3d::index_buffer::X3dIIndexBuffer;

#[repr(C)]
pub struct R3dAssetCategory {
    name: StdString,
    scale_coef: u32,
    max_size: u32,
    variant: StdString,
    size_of_textures: u32,
    texture_count: u32,
    size_of_vertex_buffers: u32,
    size_of_index_buffers: u32,
    textures: StdUnorderedSet<*mut R3dTexture>,
    vertex_buffers: StdUnorderedSet<*mut X3dIVertexBuffer>,
    index_buffers: StdUnorderedSet<*mut X3dIIndexBuffer>,
    file_mutex: RiotMutex
}