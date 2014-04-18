#![crate_id = "egl#0.1"]
#![crate_type = "lib"]
extern crate std;
use std::libc::{c_void};

// EGL related types
pub type EGLNativeWindowType = *c_void;
pub type EGLSurface = *c_void;
pub type EGLDisplay = *c_void;
pub type EGLContext = *c_void;
pub type FuncPointer = *c_void;


#[cfg(target_os = "win32", target_arch = "x86")]
#[link(name="libEGL")]
extern "stdcall" {
	fn eglSwapBuffers(display: EGLDisplay, surface: EGLSurface);
}

pub fn swap_buffers(display: EGLDisplay, surface: EGLSurface) {
	unsafe { eglSwapBuffers(display, surface); }
}
