use winapi::shared::minwindef::LPVOID;
use crate::core::utilities::memory;
use std::mem;

pub const GAME_STATE: u32 = 0x02D788D4;
pub const GAME_CLOCK: u32 = 0x02D791A8;
pub const RENDERER: u32 = 0x02D79078;
pub const RENDER_PIPELINE: u32 = 0x02D78C68;
pub const SCENE: u32 = 0x02D78F48;
pub const HUD_MANAGER: u32 = 0x02D78D24;
pub const PLAYER: u32 = 0x02D77F48;
pub const SIMPLE_ENVIRONMENT_ASSET: u32 = 0x02D78C2C;
pub const SUN: u32 = 0x02D77E70;
pub const WORLD_LIGHT_SYSTEM: u32 = 0x014C7F40;

pub unsafe fn fetch_global_ptr<T>(address: u32) -> *mut T {
    *mem::transmute::<LPVOID, *mut *mut T>(memory::convert_file_offset_to_ptr(address))
}

pub unsafe fn fetch_global<T: Sized>(address: u32) -> *mut T {
    mem::transmute::<LPVOID, *mut T>(memory::convert_file_offset_to_ptr(address))
}