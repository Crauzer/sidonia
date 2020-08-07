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
use std::{ptr, sync::{Arc, Mutex}, thread};
use winapi::{
    shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
    um::{libloaderapi::DisableThreadLibraryCalls, processthreadsapi::CreateThread, winnt},
};
use winapi::um::libloaderapi::FreeLibraryAndExitThread;
use winapi::_core::time::Duration;
use winapi::shared::minwindef::HMODULE;

pub mod core;

static mut CORE: Option<Arc<Mutex<Core>>> = None;

#[no_mangle]
pub unsafe extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: DWORD, _reserved: LPVOID) -> BOOL {
    match call_reason {
        winnt::DLL_PROCESS_ATTACH => {
            DisableThreadLibraryCalls(dll_module);
            CreateThread(ptr::null_mut(), 0, Some(initialize), ptr::null_mut(), 0, ptr::null_mut());

            TRUE
        },
        winnt::DLL_PROCESS_DETACH => {
            if let Some(core) = CORE.as_mut() {
                let mut core = core.lock().unwrap();

                core.exit();
            }

            FreeLibraryAndExitThread(dll_module, 1);

            TRUE
        },
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
            let core = core.lock().unwrap();

            match core.status() {
                CoreStatus::Exit => {
                    break;
                },
                _ => {}
            }
        }

        thread::sleep(Duration::from_millis(500));
    }

    FreeLibraryAndExitThread(lp_thread_parameter as HMODULE, 1);

    return 0;
}

#[cfg(test)]
mod tests {}
