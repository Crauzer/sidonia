use crate::process::Process;
use std::ffi::CString;
use std::{mem, ptr, env};
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::processthreadsapi::CreateRemoteThread;
use std::path::Path;
use winapi::um::minwinbase::LPTHREAD_START_ROUTINE;
use winapi::um::handleapi::CloseHandle;

mod process;

fn main() { unsafe {
    let mut process: Option<Process> = None;
    while let None = process {
        process = Process::find_by_exe("League of Legends");
    }

    println!("Found League: {:#?}", process);

    let process = process.unwrap();
    inject_core(&process);
} }

unsafe fn inject_core(process: &Process) {
    let load_library_fn = GetProcAddress(
        GetModuleHandleA(b"kernel32.dll\0".as_ptr().cast()), b"LoadLibraryA\0".as_ptr().cast());

    println!("load_library_fn: {:#?}", load_library_fn);

    let core_dll_path = env::current_dir().unwrap().join(Path::new("sidonia_core.dll"));
    println!("{:#?}", core_dll_path);
    let core_dll_path = CString::new(core_dll_path.to_str().unwrap()).unwrap();
    let core_dll_path = core_dll_path.as_bytes();

    let dll_name_ptr = process
        .allocate_memory(core_dll_path.len() + 1)
        .expect("Failed to allocate memory for sidonia_core.dll name");

    println!("dll_name_ptr: {:#?}", dll_name_ptr);

    process.write_buffer(dll_name_ptr, core_dll_path, core_dll_path.len() + 1);

    println!("dll written");

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
