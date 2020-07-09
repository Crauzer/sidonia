use crate::core::{
    msvc::{map::StdMap, unique_ptr::StdUniquePtr, vector::StdVector},
    riot::{
        baked_environment::RiotBakedEnvironmentRenderer,
        r3d::{auto_restore_texture::R3dAutoRestoreTextureSet, texture::R3dTexture, vector3::R3dVector3},
        x3d::vertex_declaration::X3dIVertexDeclaration,
    },
};

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
#[derive(Debug)]
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
