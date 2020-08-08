#![feature(llvm_asm)]
#![feature(abi_thiscall)]
#![feature(c_variadic)]
#![feature(move_ref_pattern)]
#![feature(never_type)]
#![feature(generic_associated_types)]
#![feature(ptr_offset_from)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

#[macro_use]
extern crate imgui;

#[macro_use]
extern crate bitflags;

use crate::core::{utilities::logging, Core, CoreStatus};
use std::{
    ptr,
    sync::{Arc, Mutex},
    thread,
};
use winapi::{
    _core::time::Duration,
    shared::minwindef::{BOOL, DWORD, HINSTANCE, HMODULE, LPVOID, TRUE},
    um::{
        libloaderapi::{DisableThreadLibraryCalls, FreeLibraryAndExitThread},
        processthreadsapi::CreateThread,
        winnt,
    },
};

pub mod core;

static mut CORE: Option<Arc<Mutex<Core>>> = None;

#[no_mangle]
pub unsafe extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: DWORD, _reserved: LPVOID) -> BOOL {
    match call_reason {
        winnt::DLL_PROCESS_ATTACH => {
            DisableThreadLibraryCalls(dll_module);
            CreateThread(ptr::null_mut(), 0, Some(initialize), dll_module as LPVOID, 0, ptr::null_mut());

            TRUE
        }
        winnt::DLL_PROCESS_DETACH => {
            log::warn!("DLL_PROCESS_DETACH: detaching");

            if let Some(core) = CORE.as_mut() {
                let mut core = core.lock().unwrap();

                if core.status() != CoreStatus::Exit {
                    log::info!("Core hasn't exited yet, calling exit routine...");
                    core.exit();
                }
            }

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
        // We need to periodically check if the core has exited
        if let Some(core) = CORE.as_mut() {
            let mut core = core.lock().unwrap();

            if core.scheduled_for_exit() {
                log::info!("Core is scheduled for exit, calling exit routine");

                core.exit();
                break;
            }
        }

        thread::sleep(Duration::from_millis(500));
    }

    CORE = None;

    log::info!("Calling FreeLibraryAndExitThread");
    log::info!("lp_thread_parameter: {:#?}", lp_thread_parameter);
    FreeLibraryAndExitThread(lp_thread_parameter as HMODULE, 1);

    return 0;
}

#[cfg(test)]
mod tests {}
