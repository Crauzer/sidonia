#![feature(llvm_asm)]
#![feature(abi_thiscall)]
#![feature(c_variadic)]
#![feature(move_ref_pattern)]

#[macro_use]
extern crate imgui;

use crate::core::{Core, CoreExitReason};
use std::{ptr, mem, fs};
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, HINSTANCE, LPVOID, TRUE};
use winapi::um::processthreadsapi::CreateThread;
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::_core::time::Duration;
use winapi::um::winnt;
use winapi::um::minwinbase::LPTHREAD_START_ROUTINE;
use winapi::um::winuser::{MessageBoxA, MB_OK, MB_ICONINFORMATION};
use crate::core::utilities::message_box::show_message;
use crate::core::utilities::logging;
use std::sync::{RwLock, Mutex, Arc};

pub mod core;

static mut CORE: Option<Arc<Mutex<Core>>> = None;

#[no_mangle]
pub unsafe extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: DWORD, _reserved: LPVOID) -> BOOL {
    match call_reason {
        winnt::DLL_PROCESS_ATTACH => {
            DisableThreadLibraryCalls(dll_module);
            CreateThread(
                ptr::null_mut(),
                0,
                Some(initialize),
                ptr::null_mut(),
                0,
                ptr::null_mut()
            );

            TRUE
        }
        _ => TRUE,
    }
}

pub unsafe extern "system" fn initialize(lp_thread_parameter: LPVOID) -> u32 {
    logging::initialize().expect("Failed to initialize logging");

    CORE = Some(Arc::new(Mutex::new(Core::initialize())));
    log::info!("Initialized Core");

    loop {

    }

    return 0;
}

#[cfg(test)]
mod tests {}
