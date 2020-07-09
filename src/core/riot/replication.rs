use std::os::raw::c_char;
use winapi::shared::minwindef::LPVOID;

#[repr(C)]
#[derive(Debug)]
pub struct RiotReplicationManager {
    vtable: LPVOID,
    client_only_rep_data1: RiotCReplData32,
    local_rep_data1: RiotCReplData32,
    local_rep_data2: RiotCReplData32,
    map_rep_data: RiotCReplData32,
    on_visible_rep_data: RiotCReplData32,
    global_rep_data: RiotCReplData32,
    sync_id_client_only: u32,
    sync_id_rep_data1: u32,
    sync_id_rep_data2: u32,
    sync_id_map_rep_data: u32,
    sync_on_visible_rep_data: u32,
    sync_id_global_rep_data: u32,
    is_high_priority: bool,
    base: LPVOID,
}

#[repr(C)]
#[derive(Debug)]
pub struct RiotCReplData32 {
    info: *const RiotCReplInfo32,
}

#[repr(C)]
#[derive(Debug)]
pub struct RiotCReplInfo32 {
    variable_count: u32,
    variable_names: [*const c_char; 32],
    variable_offsets: [u32; 32],
    variable_is_float_array: [bool; 32],
}

#[repr(C)]
#[derive(Debug)]
pub struct RiotReplicate<T> {
    value: T,
    index: usize,
    replication_type: RiotReplicationTypeFlags,
    replication_manager: *mut RiotReplicationManager,
}

bitflags! {
    #[repr(C)]
    struct RiotReplicationTypeFlags: u32 {
        const CLIENT_ONLY_REP_DATA = 1 << 0;
        const LOCAL_REP_DATA1 = 1 << 1;
        const LOCAL_REP_DATA2 = 1 << 2;
        const MAP_REP_DATA = 1 << 3;
        const ONVISISBLE_REP_DATA = 1 << 4;
        const GLOBAL_REP_DATA = 1 << 5;
    }
}
