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

use crate::core::{utilities::logging, Core};
use std::{
    ptr,
    sync::{Arc, Mutex},
};
use winapi::{
    shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
    um::{libloaderapi::DisableThreadLibraryCalls, processthreadsapi::CreateThread, winnt},
};

pub mod core;

static mut CORE: Option<Arc<Mutex<Core>>> = None;

#[no_mangle]
pub unsafe extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: DWORD, _reserved: LPVOID) -> BOOL {
    match call_reason {
        winnt::DLL_PROCESS_ATTACH => {
            DisableThreadLibraryCalls(dll_module);
            CreateThread(ptr::null_mut(), 0, Some(initialize), ptr::null_mut(), 0, ptr::null_mut());

            TRUE
        }
        _ => TRUE,
    }
}

pub unsafe extern "system" fn initialize(lp_thread_parameter: LPVOID) -> u32 {
    logging::initialize().expect("Failed to initialize logging");

    CORE = Some(Arc::new(Mutex::new(Core::initialize())));
    log::info!("Initialized Core");

    loop {}

    return 0;
}

#[cfg(test)]
mod tests {}
