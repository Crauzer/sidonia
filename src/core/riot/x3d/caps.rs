
#[repr(C)]
#[derive(Debug)]
pub struct X3dCaps {
    max_texture_width: u32,
    max_texture_height: u32,
    pixel_shader_version: u32,
    pixel_shader_1x_max_value: f32,
    decl_types: u32,
    stencil_caps: u32,
    primitive_misc_caps: u32,
    caps2: u32,
    simultaneous_rts_count: u32,
    max_poly_count_per_draw_call: u32,
    max_vertex_count_per_draw_call: u32
}