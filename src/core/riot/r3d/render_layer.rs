use crate::core::riot::{
    r3d::{
        color::R3dColor, matrix44::R3dMatrix44, plane::R3dPlane, screen_buffer::R3dScreenBuffer, texture::R3dTexture, vector3::R3dVector3,
    },
    x3d::{
        caps::X3dCaps,
        d3d9::{device::X3dD3d9Device, main::X3dD3d9Main},
        device::X3dIDevice,
        device_type::X3dDeviceType,
        format::X3dFormat,
        present_parameters::X3dPresentParameters,
        surface::X3dISurface,
    },
};
use num_derive::{FromPrimitive, ToPrimitive};
use winapi::shared::windef::HWND;

#[repr(C)]
#[derive(Debug)]
pub struct R3dRenderLayer {
    is_initialized: u32,
    hlib_win: HWND,
    light_type_dx8: u32,
    device_type: X3dDeviceType,
    r3d_main: *mut X3dD3d9Main,
    r3d_device1: *mut X3dD3d9Device,
    r3d_device2: *mut X3dIDevice,
    default_back_buffer: *mut X3dISurface,
    default_depth_stencil: *mut X3dISurface,
    current_render_target: *mut R3dScreenBuffer,
    is_driver_up_to_date: bool,
    limit_gpu_usage: bool,
    unk1: u32,
    unk2: u32,
    present_parameters: X3dPresentParameters,
    end_of_frame_behavior: R3dEndOfFrameBehavior,
    end_of_frame_behavior_period: u32,
    caps: X3dCaps,
    camera_position: R3dVector3,
    camera_matrix: R3dMatrix44,
    projection_matrix: R3dMatrix44,
    vec_frustum: [R3dVector3; 8],
    plane_frustum: [R3dPlane; 6],
    inverse_world_matrix: R3dMatrix44,
    near_clip: f32,
    far_clip: f32,
    first_texture: *mut R3dTexture,
    best_texture_format: X3dFormat,
    start_x: i32,
    start_y: i32,
    current_bpp: i32,
    screen_matrix: R3dMatrix44,
    view_x: u32,
    view_y: u32,
    view_width: u32,
    view_height: u32,
    use_32_bpp_textures: u32,
    force_32_bpp_textures: u32,
    constant_color_value: R3dColor,
    ambient_color: R3dColor,
    back_clear_color: R3dColor,
    stats: R3dRenderLayerStats,
    error: R3dRenderLayerError,
}

#[repr(C)]
#[derive(Debug)]
pub struct R3dRenderLayerStats {
    texture_memory: i32,
    buffer_memory: i32,
    screen_buffer_memory: i32,
    material_change_count: i32,
    mode_changes_count: i32,
    texture_changes_count: i32,
    triangles_rendered_count: i32,
    average_strip_length: i32,
    draw_count: i32,
}

#[repr(u32)]
#[derive(Debug)]
pub enum R3dRenderLayerError {
    None = 0,
    NoShader2 = 1,
    FailedToCreateDevice = 2,
}

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum R3dEndOfFrameBehavior {
    None = 0,
    Flush = 1,
}

impl R3dRenderLayer {
    pub fn is_initialized(&self) -> bool {
        self.is_initialized != 0
    }

    pub fn d3d9_main_mut(&mut self) -> Option<&'static mut X3dD3d9Main> {
        unsafe { self.r3d_main.as_mut::<'static>() }
    }
    pub fn stats(&self) -> &R3dRenderLayerStats {
        &self.stats
    }
}

impl R3dRenderLayerStats {
    pub fn texture_memory(&self) -> i32 {
        self.texture_memory
    }
    pub fn buffer_memory(&self) -> i32 {
        self.buffer_memory
    }
    pub fn screen_buffer_memory(&self) -> i32 {
        self.screen_buffer_memory
    }
    pub fn material_change_count(&self) -> i32 {
        self.material_change_count
    }
    pub fn mode_changes_count(&self) -> i32 {
        self.mode_changes_count
    }
    pub fn texture_changes_count(&self) -> i32 {
        self.texture_changes_count
    }
    pub fn triangles_rendered_count(&self) -> i32 {
        self.triangles_rendered_count
    }
    pub fn average_strip_length(&self) -> i32 {
        self.triangles_rendered_count
    }
    pub fn draw_count(&self) -> i32 {
        self.triangles_rendered_count
    }
}
