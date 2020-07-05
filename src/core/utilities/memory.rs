use std::ptr;
use winapi::{_core::time::Duration, shared::minwindef::LPVOID, um::libloaderapi::GetModuleHandleA};

static mut BASE: Option<u32> = None;

const IDA_BASE: u32 = 0x400000;

pub fn base() -> u32 {
    unsafe {
        if let None = BASE {
            let mut base = 0u32;

            // Handy asm to grab process base
            llvm_asm!(" mov eax, fs:[30h] # eax = TEB.ProcessEnvironmentBlock
                        mov eax, [eax+8h] # eax = (eax: PEB).ImageBaseAddress
                      " : "={eax}"(base) ::: "intel");

            BASE = Some(base);
        }

        BASE.unwrap()
    }
}

pub fn convert_file_offset_to_ptr(offset: u32) -> LPVOID {
    (offset + (base() - IDA_BASE)) as LPVOID
}

pub fn wait_for_ptr_non_zero(ptr_address: LPVOID) -> LPVOID {
    unsafe {
        let ptr = *(ptr_address as *mut LPVOID);
        while ptr.is_null() {
            std::thread::sleep(Duration::from_millis(10));
        }

        ptr
    }
}
