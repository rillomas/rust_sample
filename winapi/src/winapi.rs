#![crate_id = "winapi#0.1"]
#![crate_type = "lib"]
extern crate libc;
use libc::types::common::c95::{c_void};
use libc::types::os::arch::c95::{c_int};
use libc::types::os::arch::extra::{LPCWSTR,HMODULE};
use std::ptr::null;

pub struct WindowContext {
    pub width: c_int,
    pub height: c_int,
    pub handle: *mut c_void
}

#[cfg(windows)]
#[link(name="winapi")]
#[link(name="gdi32")]
extern "stdcall" {
    fn createWindow(context: *mut WindowContext, title: LPCWSTR) -> bool;
    fn mainLoop(context: *mut WindowContext);
}

#[cfg(windows)]
#[link(name="kernel32")]
extern "system" {
    fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
}

pub fn get_module_handle(name: Option<&str>) -> HMODULE {
    match name {
        Some(n) => {
            let mut mod_name: Vec<u16> = n.utf16_units().collect();
            mod_name.push(0);
            unsafe { GetModuleHandleW(mod_name.as_ptr()) }
        },
        None => unsafe { GetModuleHandleW(null())}
    }
}

pub fn create_window(context: *mut WindowContext, title: &str) -> bool {
    let mut win_name: Vec<u16> = title.utf16_units().collect();
    win_name.push(0);
    unsafe { createWindow(context, win_name.as_ptr()) }
}

pub fn main_loop(context: *mut WindowContext) {
    unsafe { mainLoop(context) }
}