use winapi::shared::minwindef::LPVOID;

#[repr(C)]
pub struct RiotTickInterface {
    vtbl: LPVOID,
}

#[repr(C)]
pub struct RiotClockInterface {
    vtbl: LPVOID,
    master: *mut RiotClockInterface,
    multiplier: f64,
    is_controllable: bool,
}

#[repr(C)]
pub struct RiotClockController {
    tick_interface: RiotTickInterface,
    clock_interface: *mut RiotClockInterface,
}

#[repr(C)]
pub struct RiotGameClock {
    controller: *mut RiotClockController,
    current_time: f32,
    time_delta: f32,
    system_time_diff: f32,
    is_initialized: bool,
    min_multiplier: f32,
    max_multiplier: f32,
    multiplier_control: i32,
    paused_mutliplier: f32,
}

impl RiotGameClock {
    pub fn controller(&self) -> Option<&'static RiotClockController> {
        unsafe { self.controller.as_ref::<'static>() }
    }
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }
    pub fn current_time(&self) -> f32 {
        self.current_time
    }
    pub fn time_delta(&self) -> f32 {
        self.time_delta
    }
}
