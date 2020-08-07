use crate::process::Process;
use std::{env, ffi::CString, mem, path::Path, ptr};
use winapi::{
    shared::minwindef::{HMODULE, MAX_PATH},
    um::{
        handleapi::CloseHandle,
        libloaderapi::{FreeLibrary, GetModuleFileNameA, GetModuleHandleA, GetProcAddress},
        minwinbase::LPTHREAD_START_ROUTINE,
        processthreadsapi::CreateRemoteThread,
        psapi::{EnumProcessModules, GetModuleFileNameExA},
        winnt::{CHAR, WCHAR},
    },
};

mod process;

fn main() {
    unsafe {
        let mut process: Option<Process> = None;
        while let None = process {
            process = Process::find_by_exe("League of Legends");
        }

        println!("Found League: {:#?}", process);

        let process = process.unwrap();
        inject_core(&process);
    }
}
unsafe fn inject_core(process: &Process) {
    // Get LoadLibrary from kernel32
    let load_library_fn = GetProcAddress(
        GetModuleHandleA(b"kernel32.dll\0".as_ptr().cast()),
        b"LoadLibraryA\0".as_ptr().cast(),
    );

    // Construct core dll path
    let core_dll_path = env::current_dir().unwrap().join(Path::new("sidonia_core.dll"));
    println!("{:#?}", core_dll_path);
    let core_dll_path = CString::new(core_dll_path.to_str().unwrap()).unwrap();
    let core_dll_path = core_dll_path.as_bytes();

    // Allocate memory in the target process for the core dll name
    let dll_name_ptr = process
        .allocate_memory(core_dll_path.len() + 1)
        .expect("Failed to allocate memory for sidonia_core.dll name");

    println!("dll_name_ptr: {:#?}", dll_name_ptr);

    // Write the core dll name to the target process
    process.write_buffer(dll_name_ptr, core_dll_path, core_dll_path.len() + 1);

    println!("dll written");

    // Inject
    let core_thread_start_routine: LPTHREAD_START_ROUTINE = mem::transmute(load_library_fn);
    let core_thread_handle = CreateRemoteThread(
        process.handle(),
        ptr::null_mut(),
        0,
        core_thread_start_routine,
        dll_name_ptr,
        0,
        ptr::null_mut(),
    );

    if core_thread_handle.is_null() {
        println!("failed to create remote thread");
    } else {
        println!("remote thread created: {:#?}", core_thread_handle);
        CloseHandle(core_thread_handle);
    }
}
