use std::time::Duration;
use std::{mem, ptr, thread};
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{DWORD, HMODULE, LPCVOID, LPVOID, MAX_PATH};
use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::{ReadProcessMemory, VirtualAllocEx, VirtualProtectEx, WriteProcessMemory};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::{EnumProcessModules, EnumProcesses, GetModuleFileNameExW};
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS};
use winapi::um::winnt::{
    HANDLE, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE, PAGE_READWRITE, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ,
    PROCESS_VM_WRITE, SYNCHRONIZE, WCHAR,
};

#[derive(Debug)]
pub struct Process {
    pid: DWORD,
    handle: HANDLE,
    name: String,
    base: u32,
}

impl Process {
    pub fn new(pid: DWORD, handle: HANDLE, name: &str) -> Option<Process> {
        let base = Process::find_process_base(handle, name);
        if let Some(base) = base {
            Some(Process {
                pid,
                handle,
                name: name.to_string(),
                base,
            })
        } else {
            None
        }
    }

    pub fn handle(&self) -> HANDLE { self.handle }
    pub fn base(&self) -> u32 { self.base }

    pub fn find_by_exe(name: &str) -> Option<Self> {
        unsafe {
            let snapshot_handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if snapshot_handle.is_null() {
                return None;
            }

            let mut process_entry: PROCESSENTRY32 = std::mem::zeroed();
            process_entry.dwSize = mem::size_of::<PROCESSENTRY32>() as u32;

            let mut modules = [mem::zeroed::<HMODULE>(); 1024];
            let mut bytes_needed = 0;

            let mut i = Process32First(snapshot_handle, &mut process_entry);
            while i != 0 {
                let process_exe_file = String::from_utf8(process_entry.szExeFile.iter().map(|x| *x as u8).collect());

                if process_exe_file.unwrap().contains(name) {
                    let process_handle = OpenProcess(
                        PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_QUERY_INFORMATION | SYNCHRONIZE,
                        0,
                        process_entry.th32ProcessID,
                    );
                    if process_handle.is_null() {
                        return None;
                    }

                    if EnumProcessModules(
                        process_handle,
                        modules.as_mut_ptr(),
                        mem::size_of::<HMODULE>() as u32,
                        &mut bytes_needed,
                    ) != 0
                    {
                        return Process::new(process_entry.th32ProcessID, process_handle, name);
                    }
                }

                i = Process32Next(snapshot_handle, &mut process_entry)
            }

            None
        }
    }

    pub fn find_process_base(handle: HANDLE, mod_name: &str) -> Option<u32> {
        unsafe {
            let mut modules = [mem::zeroed::<HMODULE>(); 1024];
            let mut bytes_needed = 0;

            if EnumProcessModules(handle, modules.as_mut_ptr(), mem::size_of::<HMODULE>() as u32, &mut bytes_needed) != 0 {
                let mut i = 0;
                while i < bytes_needed / mem::size_of::<HMODULE>() as u32 {
                    let mut module_name = [WCHAR::default(); MAX_PATH];
                    if GetModuleFileNameExW(
                        handle,
                        modules[i as usize],
                        module_name.as_mut_ptr().cast(),
                        (mem::size_of_val(&module_name) / mem::size_of::<WCHAR>()) as u32,
                    ) != 0
                    {
                        let module_name = String::from_utf16(&module_name).unwrap();
                        if module_name.contains(mod_name) {
                            return Some(modules[i as usize] as u32);
                        }
                    }

                    i += 1;
                }
            }

            Option::None
        }
    }

    pub fn dump(&self) -> Vec<u8> {
        unsafe {
            let mut dumped_data = vec![0u8; 0x4000000];
            let mut total_read = 0u64;

            while total_read != 0x4000000 {
                ReadProcessMemory(
                    self.handle,
                    (self.base() as u64 + total_read) as *const c_void,
                    dumped_data.as_mut_ptr().add(total_read as usize) as *mut c_void,
                    0x1000,
                    &mut 0,
                );
                total_read += 0x1000;
            }

            dumped_data
        }
    }

    pub fn allocate_memory(&self, size: usize) -> Option<LPVOID> {
        unsafe {
            let ptr = VirtualAllocEx(self.handle, ptr::null_mut(), size, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);

            if ptr != (0 as *mut c_void) {
                Some(ptr)
            } else {
                None
            }
        }
    }
    pub fn allocate_struct<T>(&self) -> Option<LPVOID> {
        let size = mem::size_of::<T>();
        self.allocate_memory(size)
    }

    pub fn read_buffer(&self, ptr: LPVOID, size: usize) -> Option<Vec<u8>> {
        unsafe {
            let mut buffer = vec![0u8; size];

            if ReadProcessMemory(self.handle, ptr, buffer.as_mut_ptr().cast(), size, ptr::null_mut()) != 0 {
                Some(buffer)
            } else {
                None
            }
        }
    }
    pub fn read<T>(&self, ptr: LPVOID) -> Option<T> {
        unsafe {
            let size = mem::size_of::<T>();
            let mut data = vec![0u8; size];

            if ReadProcessMemory(self.handle, ptr, data.as_mut_ptr().cast(), size, ptr::null_mut()) != 0 {
                Some(ptr::read_unaligned::<T>(data.as_ptr().cast()))
            } else {
                None
            }
        }
    }

    pub fn write_buffer(&self, address: LPVOID, buffer: &[u8], size: usize) -> bool {
        unsafe { WriteProcessMemory(self.handle, address, buffer.as_ptr().cast(), size, &mut 0) != 0 }
    }
    pub fn write<T>(&self, address: LPVOID, to_write: &T) -> bool {
        unsafe {
            let size = mem::size_of::<T>();

            WriteProcessMemory(self.handle, address, to_write as *const _ as LPCVOID, size, ptr::null_mut()) != 0
        }
    }

    pub fn mark_memory_executable(&self, ptr: LPVOID, size: usize) -> bool {
        unsafe { VirtualProtectEx(self.handle, ptr, size, PAGE_EXECUTE, ptr::null_mut()) != 0 }
    }

    pub fn wait_ptr_non_zero(&self, ptr: LPVOID) -> u32 {
        unsafe {
            let mut buffer = [0u8; 4];
            let sleep_duration = Duration::from_millis(1);

            while u32::from_le_bytes(buffer) == 0 {
                thread::sleep(sleep_duration);

                ReadProcessMemory(self.handle, ptr, buffer.as_mut_ptr().cast(), 4usize, ptr::null_mut());
            }

            u32::from_le_bytes(buffer)
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) { unsafe {
        CloseHandle(self.handle);
    } }
}