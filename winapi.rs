#![crate_id = "winapi#0.1"]
#![crate_type = "lib"]
extern crate libc;
use libc::types::common::c95::{c_void};
use libc::types::os::arch::c95::{c_int};
use std::ptr::null;

// win32 related types
type LPCWSTR = *u16;
pub type HMODULE = *c_void;

pub struct WindowContext {
    pub width: c_int,
    pub height: c_int,
    pub handle: *c_void
}

#[cfg(target_os = "win32", target_arch = "x86")]
#[link(name="winapi")]
extern {
    fn createWindow(context: *WindowContext, title: LPCWSTR) -> bool;
    fn mainLoop(context: *WindowContext);
}

#[cfg(target_os = "win32", target_arch = "x86")]
#[link(name="kernel32")]
extern "stdcall" {
    fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
}

pub fn get_module_handle(name: Option<~str>) -> HMODULE {
    match name {
        Some(n) => return std::os::win32::as_utf16_p(n, |buf| unsafe { GetModuleHandleW(buf) }),
        None => unsafe { return GetModuleHandleW(null())}
    }
}

pub fn create_window(context: *WindowContext, title: ~str) -> bool {
    return std::os::win32::as_utf16_p(title, |buf| unsafe { createWindow(context, buf) });
}

pub fn main_loop(context: *WindowContext) {
    unsafe { mainLoop(context); }
}