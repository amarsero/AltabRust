mod windows_api;
use crate::os::windows_api::WindowsApi;

pub fn get_os() -> impl OS {
    return WindowsApi {
        
    };
}

pub trait OS {
    fn directory_enumerate_files(path: &str);
}