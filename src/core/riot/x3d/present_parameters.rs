use crate::core::riot::x3d::{format::X3dFormat, multisample_type::X3dMultiSampleType, swap_effect_type::X3dSwapEffectType};
use winapi::shared::windef::HWND;

#[repr(C)]
#[derive(Debug)]
pub struct X3dPresentParameters {
    back_buffer_width: u32,
    back_buffer_height: u32,
    back_buffer_format: X3dFormat,
    back_buffer_count: u32,
    multisample_type: X3dMultiSampleType,
    multisample_quality: u32,
    swap_effect_type: X3dSwapEffectType,
    device_window: HWND,
    is_windowed: bool,
    enable_auto_depth_stencil: bool,
    auto_depth_stencil_format: X3dFormat,
    flags: u32,
    fullscreen_refresh_rate_hz: u32,
    presentation_interval: i32,
}
