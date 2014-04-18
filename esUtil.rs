#![crate_id = "esUtil#0.1"]
#![crate_type = "lib"]
extern crate std;
use std::libc::{c_uint, c_uchar, c_void, c_char, int8_t, c_short, c_int, uint8_t, c_ushort};

// GL related types
pub type GLvoid = c_void;
pub type GLchar = c_char;
pub type GLenum = c_uint;
pub type GLboolean = c_uchar;
pub type GLbitfield = c_uint;
pub type GLbyte = int8_t;
pub type GLshort = c_short;
pub type GLint = c_int;
pub type GLsizei = c_int;
pub type GLubyte = uint8_t;
pub type GLushort = c_ushort;
pub type GLuint = c_uint;
pub type GLfloat = f32;
pub type GLclampf = f32;

// EGL related types
pub type EGLNativeWindowType = *c_void;
pub type EGLSurface = *c_void;
pub type EGLDisplay = *c_void;
pub type EGLContext = *c_void;
pub type FuncPointer = *c_void;

// win32 related types
type LPCWSTR = *u16;

pub struct ESContext {
    userData: *c_void,
    width: GLint,
    height: GLint,
    hWnd: EGLNativeWindowType,
    eglDisplay: EGLDisplay,
    eglContext: EGLContext,
    eglSurface: EGLSurface,
    drawFunc: FuncPointer,
    keyFunc: FuncPointer,
    updateFunc: FuncPointer
}
    /// Callbacks
    // void (ESCALLBACK *drawFunc) ( void* );
    // void (ESCALLBACK *keyFunc) ( void*, unsigned char, int, int );
    // void (ESCALLBACK *updateFunc) ( void*, float deltaTime );

#[cfg(target_os = "win32", target_arch = "x86")]
#[link(name="es_util")]
extern "cdecl" {
    fn esInitContext(context: *ESContext);
    fn esCreateWindow(context: *ESContext, title: LPCWSTR, width: GLint, height: GLint, flags: GLuint);
    fn esMainLoop(context: *ESContext);
}


pub fn initContext(context: &ESContext) {
    unsafe { esInitContext(context); }
}

pub fn createWindow(context: &ESContext, title: ~str, width: i32, height: i32, flags: u32) {
    let mut t = title.to_utf16();
    // Null terminate before passing on.
    t.push(0u16); 
    unsafe { esCreateWindow(context, t.as_ptr(), width, height, flags); }
}

pub fn mainLoop(context: &ESContext) {
    unsafe { esMainLoop(context); }
}