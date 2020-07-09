use winapi::shared::minwindef::LPVOID;

// T: f64 | f32
pub struct RiotBasicTimer {
    timer_interface: RiotITimer,
    start_ticks: f64,
    elapsed_ticks: f64,
    is_active: bool,
}

#[repr(C)]
pub struct RiotITimer {
    vtable: LPVOID,
}
