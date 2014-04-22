#![crate_id = "winapi#0.1"]
#![crate_type = "lib"]
extern crate libc;
use libc::types::common::c95::{c_void};
use libc::types::os::arch::c95::{c_int};

// win32 related types
type LPCWSTR = *u16;

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

pub fn create_window(context: *WindowContext, title: ~str) -> bool {
    let mut t = title.to_utf16();
    // Null terminate before passing on.
    t.push(0u16); 
	unsafe { return createWindow(context, t.as_ptr()); }
}

pub fn main_loop(context: *WindowContext) {
	unsafe { mainLoop(context); }
}