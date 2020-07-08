use winapi::shared::minwindef::LPVOID;
use bitflags::_core::marker::PhantomData;
use crate::core::msvc::smart_pointers::StdSharedPtr;

#[repr(C)]
#[derive(Debug)]
pub struct RiotOnNetworkRegister<T> {
    vtable: LPVOID,
    callback: LPVOID,
    phantom: PhantomData<T>
}

pub type RiotNetIdType = u32;

#[repr(C)]
#[derive(Debug)]
pub struct RiotNetId {
    id_type: RiotNetIdType,
}

#[repr(C)]
pub struct RiotNetVisibilityObject {
    last_update_time: f32,
    pimpl: StdSharedPtr<RiotNetVisibilityObjectImpl>
}

#[repr(C)]
#[derive(Debug)]
pub struct RiotNetVisibilityObjectImpl {
    on_leave_visibility_client: RiotOnNetworkRegister<!>, // T: PKT_OnLeaveVisiblityClient_s
    on_leave_locl_visibility_client: RiotOnNetworkRegister<!>, // T: PKT_OnLeaveLocalVisiblityClient_s
    on_enter_team_visibility: RiotOnNetworkRegister<!>, // T: PKT_S2C_OnEnterTeamVisiblity_s
    on_leave_team_visibility: RiotOnNetworkRegister<!>, // T: PKT_S2C_OnLeaveTeamVisiblity_s
    net_visibility_data: *mut RiotNetIVisibilityData,
    is_visible: bool,
    is_local_visible: bool,
    team_visible_flags: u32,
    force_visible: bool,
    network_id: *mut RiotNetId,
    client_id: RiotNetId
}

#[repr(C)]
#[derive(Debug)]
pub struct RiotNetIVisibilityData {
    vtable: LPVOID
}