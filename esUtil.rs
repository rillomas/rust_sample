#![crate_id = "esUtil#0.1"]
#![crate_type = "lib"]
extern crate std;
extern crate gl2;
extern crate egl;
use std::libc::{c_void, c_uchar, c_int, c_uint, c_float};

pub static WINDOW_RGB: c_uint = 0;
pub static WINDOW_ALPHA: c_uint = 1;
pub static WINDOW_DEPTH: c_uint = 2;
pub static WINDOW_STENCIL: c_uint = 4;
pub static WINDOW_MULTISAMPLE: c_uint = 8;
pub static WINDOW_POST_SUB_BUFFER_SUPPORTED: c_uint = 16;


// Callbacks
pub type DrawFunc = extern "cdecl" fn(*ESContext);
pub type KeyFunc = extern "cdecl" fn(*ESContext, c_uchar, c_int, c_int);
pub type UpdateFunc = extern "cdecl" fn(*ESContext, c_float);

// win32 related types
type LPCWSTR = *u16;

pub struct ESContext {
    userData: *c_void,
    width: gl2::GLint,
    height: gl2::GLint,
    hWnd: egl::EGLNativeWindowType,
    display: egl::EGLDisplay,
    context: egl::EGLContext,
    surface: egl::EGLSurface,
    drawFunc: Option<DrawFunc>,
    keyFunc: Option<KeyFunc>,
    updateFunc: Option<UpdateFunc>
}

#[cfg(target_os = "win32", target_arch = "x86")]
#[link(name="es_util")]
extern "cdecl" {
    fn esInitContext(context: *ESContext);
    fn esCreateWindow(context: *ESContext, title: LPCWSTR, width: gl2::GLint, height: gl2::GLint, flags: gl2::GLuint) -> gl2::GLboolean;
    fn esMainLoop(context: *ESContext);
    fn esRegisterDrawFunc(context: *ESContext, func: DrawFunc);
    fn esRegisterKeyFunc(context: *ESContext, func: KeyFunc);
    fn esRegisterUpdateFunc(context: *ESContext, func: UpdateFunc);
}


pub fn initContext(context: &ESContext) {
    unsafe { esInitContext(context); }
}

pub fn createWindow(context: &ESContext, title: ~str, width: i32, height: i32, flags: u32) -> bool{
    let mut t = title.to_utf16();
    // Null terminate before passing on.
    t.push(0u16); 
    unsafe {
        let res = esCreateWindow(context, t.as_ptr(), width, height, flags);
        return res == gl2::TRUE;
    }
}

pub fn registerDrawFunc(context: *ESContext, func: DrawFunc) {
    unsafe { esRegisterDrawFunc(context, func); }
}

pub fn registerKeyFunc(context: *ESContext, func: KeyFunc) {
    unsafe { esRegisterKeyFunc(context, func); }
}

pub fn registerUpdateFunc(context: *ESContext, func: UpdateFunc) {
    unsafe { esRegisterUpdateFunc(context, func); }
}

pub fn mainLoop(context: &ESContext) {
    unsafe { esMainLoop(context); }
}