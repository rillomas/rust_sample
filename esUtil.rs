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

#[link(name="es_util")]
extern {
    pub fn esInitContext(context: *ESContext);
}