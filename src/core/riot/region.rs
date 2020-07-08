use winapi::shared::minwindef::LPVOID;
use crate::core::riot::r3d::vector2::R3dVector2;
use crate::core::riot::net::RiotNetId;
use crate::core::riot::RiotTeam;
use crate::core::riot::game_object::RiotGameObject;

#[repr(C)]
pub struct RiotRegion {
    vtable: LPVOID,
    origin: R3dVector2,
    line_of_sight_radius: f32,
    grass_radius: f32,
    size_multiplier: f32,
    size_additive: f32,
    lifetime: f32,
    attached_object_id: u32,
    net_id: RiotNetId,
    client_id: RiotNetId, // Riot::CLIENT_ID
    owning_team: RiotTeam,
    attached_unit: *mut RiotGameObject,
    raster: RiotRegionRasterizeData,
    vision_flags: RiotRegionVisionFlags,
    requires_line_of_sight: bool
}

#[repr(C)]
pub struct RiotRegionRasterizeData {
    rasterized_grid: *mut u8,
    is_rasterized: bool,
    is_dirty: bool
}

#[repr(C)]
pub struct RiotRegionVisionFlags {
    reveal_specific_net_id: RiotNetId,
    has_draw_fade: bool,
    grants_vision: bool,
    reveal_stealth: bool
}
