use crate::core::{
    msvc::{map::StdMap, string::StdString, unique_ptr::StdUniquePtr, vector::StdVector},
    riot::{
        baked_environment::RiotBakedEnvironmentRenderer,
        box3d::RiotBox3D,
        color::RiotColorValue,
        r3d::{
            auto_restore_texture::R3dAutoRestoreTextureSet, color::R3dColor, matrix44::R3dMatrix44, texture::R3dTexture,
            vector3::R3dVector3,
        },
        sphere::RiotSphere,
        x3d::{index_buffer::X3dIIndexBuffer, vertex_buffer::X3dIVertexBuffer, vertex_declaration::X3dIVertexDeclaration},
    },
};
use std::{ffi::CStr, ops::Deref};
use winapi::{
    ctypes::c_char,
    shared::minwindef::{LPCVOID, LPVOID},
    um::winnt::PVOID,
};

// Renderer

#[repr(C)]
pub struct RiotSimpleEnvironmentRenderer {
    flags: u32,
    vertex_declarations: [*mut X3dIVertexDeclaration; 7],
    circle_texture: *mut R3dTexture,
    height_scale: *mut R3dTexture,
    composite_color_light: *mut R3dTexture,
    fog_lighting: *mut R3dTexture,
    watermark_texture: *mut R3dTexture,
    display_list: StdVector<RiotSimpleEnvironmentDisplayListElement>,
    camera_position: R3dVector3,
    settings_age: u32,
    shadow_settings_age: u32,
    current_mode: RiotSimpleEnvironmentRendererMode,
    auto_restore_texture_set: *mut R3dAutoRestoreTextureSet,
    last_material_type_rendered: RiotSimpleEnvironmentMaterialType,
    last_vertex_declaration_index: u32,
    last_vertex_buffer_index: u32,
    last_index_buffer_index: u32,
    last_material_index: u32,
    are_shader_indices_initialized: bool,
    disable_can_concatenate: bool,
    last_clipping_required: bool,
    pixel_shader_variants: StdMap<i32, RiotSimpleEnvironmentShaderVariants>,
    baked_environment_renderer: StdUniquePtr<RiotBakedEnvironmentRenderer>,
}

#[repr(u32)]
#[derive(Debug)]
pub enum RiotSimpleEnvironmentRendererMode {
    LIT = 0x0,
    ShadowMap = 0x1,
    ZPass = 0x2,
    WallOfGrass = 0x3,
    WallOfGrassZPass = 0x4,
    PlanarProjection = 0x5,
    CubeProjection = 0x6,
    VertexAlpha = 0x7,
    ShadowMapAlphaTest = 0x8,
    PrecomputePerPixelLightingPointLight = 0x9,
    PrecomputePerPixelLightingSunSky = 0xA,
    PointLightShadowRender = 0xB,
    DirExpShadowRender = 0xC,
    AoVisibility = 0xD,
    GenRelighting = 0xE,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum RiotSimpleEnvironmentMaterialType {
    Default = 0x0,
    Decal = 0x1,
    WallOfGrass = 0x2,
    FourBlend = 0x3,
    AntiBrush = 0x4,
}

#[repr(C)]
#[derive(Debug)]
pub struct RiotSimpleEnvironmentDisplayListElement {
    material_index: u32,
    vertex_buffer_index: u32,
    index_buffer_index: u32,
    min_index: u32,
    vertex_count: u32,
    start_index: u32,
    primitive_count: u32,
    vertex_declaration_index: u32,
    camera_distance: f32,
    is_clipping_required: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct RiotSimpleEnvironmentShaderVariants {
    death_screen_variant_index: i32,
}

// Asset

#[repr(C)]
pub struct RiotSimpleEnvironmentAsset {
    /*
      std::__1::string m_FileName;
    const Riot::PackageInterface *m_Package;
    std::__1::vector<Riot::Eternity::SimpleEnvironment::Material *,std::__1::allocator<Riot::Eternity::SimpleEnvironment::Material *> > m_Materials;
    std::__1::vector<Riot::X3D::IVertexBuffer *,std::__1::allocator<Riot::X3D::IVertexBuffer *> > m_VertexBuffers;
    std::__1::vector<Riot::X3D::IIndexBuffer *,std::__1::allocator<Riot::X3D::IIndexBuffer *> > m_IndexBuffers;
    std::__1::vector<Riot::Eternity::SimpleEnvironment::File::MESH,std::__1::allocator<Riot::Eternity::SimpleEnvironment::File::MESH> > m_Meshes;
    std::__1::vector<Riot::Eternity::SimpleEnvironment::File::NODE,std::__1::allocator<Riot::Eternity::SimpleEnvironment::File::NODE> > m_Nodes;

       */
    file_name: StdString,
    package_interface: LPCVOID, // const Riot::PackageInterface*
    materials: StdVector<*mut RiotSimpleEnvironmentMaterial>,
    vertex_buffers: StdVector<*mut X3dIVertexBuffer>,
    index_buffers: StdVector<*mut X3dIIndexBuffer>,
    meshes: StdVector<RiotSimpleEnvironmentMesh>,
    nodes: StdVector<()>,
}

bitflags! {
    pub struct RiotSimpleEnvironmentMaterialFlags: u32 {
        const GROUND = 1 << 0;
        const NO_SHADOW = 1 << 1;
        const VERTEX_ALPHA = 1 << 2;
        const LIGHTMAPPED = 1 << 3;
        const DUAL_VERTEX_COLOR = 1 << 4;
        const BACKGROUND = 1 << 5;
        const BK_WITH_FOG = 1 << 6;
    }
}

#[repr(C)]
pub struct RiotSimpleEnvironmentMaterial {
    // Riot::Eternity::SimpleEnvironment::File::MATERIAL
    name: [u8; 260],
    material_type: RiotSimpleEnvironmentMaterialType,
    flags: RiotSimpleEnvironmentMaterialFlags,
    channels: [RiotSimpleEnvironmentChannel; 8],
    // Riot::Eternity::SimpleEnvironment::File::MATERIAL
    textures: [*mut R3dTexture; 8],
}

#[repr(C)]
pub struct RiotSimpleEnvironmentChannel {
    color: RiotColorValue,
    texture_name: [u8; 260],
    transform: R3dMatrix44,
}

#[repr(C)]
pub struct RiotSimpleEnvironmentMesh {
    quality: i32,
    flags: u32,
    bounding_sphere: RiotSphere,
    bounding_box: RiotBox3D,
    material: u32,
    simple_geometry: RiotSimpleEnvironmentIndexedPrimitive,
    complex_geometry: RiotSimpleEnvironmentIndexedPrimitive,
}

#[repr(C)]
pub struct RiotSimpleEnvironmentIndexedPrimitive {
    vertex_buffer: u32,
    first_vertex: u32,
    vertex_count: u32,
    index_buffer: u32,
    first_index: u32,
    index_count: u32,
}

impl RiotSimpleEnvironmentAsset {
    pub fn name(&self) -> String {
        self.file_name.to_string()
    }
    pub fn package_interface(&self) -> LPCVOID {
        self.package_interface
    }
    pub fn materials(&self) -> Vec<Option<&'static RiotSimpleEnvironmentMaterial>> {
        unsafe {
            self.materials
                .deref()
                .iter()
                .map(|x| x.as_ref::<'static>())
                .collect::<Vec<Option<&'static RiotSimpleEnvironmentMaterial>>>()
        }
    }
    pub fn materials_mut(&mut self) -> Vec<Option<&'static mut RiotSimpleEnvironmentMaterial>> {
        unsafe {
            self.materials
                .deref()
                .iter()
                .map(|x| x.as_mut::<'static>())
                .collect::<Vec<Option<&'static mut RiotSimpleEnvironmentMaterial>>>()
        }
    }
}

impl RiotSimpleEnvironmentMaterial {
    pub fn name(&self) -> &str {
        unsafe { CStr::from_bytes_with_nul_unchecked(self.name.as_ref()).to_str().unwrap() }
    }
    pub fn material_type(&self) -> RiotSimpleEnvironmentMaterialType {
        self.material_type
    }
    pub fn flags(&self) -> RiotSimpleEnvironmentMaterialFlags {
        self.flags
    }
    pub fn channels(&self) -> &[RiotSimpleEnvironmentChannel; 8] {
        &self.channels
    }
    pub fn channels_mut(&mut self) -> &mut [RiotSimpleEnvironmentChannel; 8] {
        &mut self.channels
    }
    pub fn textures(&self) -> Vec<Option<&'static R3dTexture>> {
        unsafe {
            self.textures
                .iter()
                .map(|x| x.as_ref::<'static>())
                .collect::<Vec<Option<&'static R3dTexture>>>()
        }
    }
}

impl RiotSimpleEnvironmentChannel {
    pub fn color(&self) -> RiotColorValue {
        self.color
    }
    pub fn color_mut(&mut self) -> &mut RiotColorValue {
        &mut self.color
    }
    pub fn texture_name(&self) -> &str {
        unsafe { CStr::from_bytes_with_nul_unchecked(self.texture_name.as_ref()).to_str().unwrap() }
    }
    pub fn transform(&self) -> R3dMatrix44 {
        self.transform
    }

    pub fn set_color(&mut self, color: RiotColorValue) {
        self.color = color;
    }
    pub fn set_transform(&mut self, transform: R3dMatrix44) {
        self.transform = transform;
    }
}

impl RiotSimpleEnvironmentMesh {
    pub fn quality(&self) -> i32 {
        self.quality
    }
    pub fn flags(&self) -> u32 {
        self.flags
    }
    pub fn bounding_sphere(&self) -> RiotSphere {
        self.bounding_sphere
    }
    pub fn bounding_box(&self) -> RiotBox3D {
        self.bounding_box
    }
    pub fn material(&self) -> u32 {
        self.material
    }
}
