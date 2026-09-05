use std::path::PathBuf;
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
use windows_sys::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_QUERY_LIMITED_INFORMATION,
};

pub fn get_process_path(pid: u32) -> Option<PathBuf> {
    if pid == 0 {
        return Some(PathBuf::from("System Idle Process"));
    }
    if pid == 4 {
        return Some(PathBuf::from("System"));
    }

    unsafe {
        let handle: HANDLE = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if handle.is_null() {
            return None;
        }

        let mut buffer = [0u16; 1024];
        let mut capacity = buffer.len() as u32;

        let success = QueryFullProcessImageNameW(handle, 0, buffer.as_mut_ptr(), &mut capacity);
        CloseHandle(handle);

        if success != 0 {
            let path_str = String::from_utf16_lossy(&buffer[..capacity as usize]);
            return Some(PathBuf::from(path_str));
        }
    }

    None
}
