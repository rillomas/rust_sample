#![crate_id = "esUtil#0.1"]
#![crate_type = "lib"]
extern crate std;
extern crate gl2;
use std::libc::c_void;

// EGL related types
pub type EGLNativeWindowType = *c_void;
pub type EGLSurface = *c_void;
pub type EGLDisplay = *c_void;
pub type EGLContext = *c_void;
pub type FuncPointer = *c_void;

// Nullable Function Pointers
pub type DrawFunc = Option<extern "cdecl" fn(context: *ESContext)>;

// win32 related types
type LPCWSTR = *u16;

pub struct ESContext {
    userData: *c_void,
    width: gl2::GLint,
    height: gl2::GLint,
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
    fn esCreateWindow(context: *ESContext, title: LPCWSTR, width: gl2::GLint, height: gl2::GLint, flags: gl2::GLuint);
    fn esMainLoop(context: *ESContext);
    fn esRegisterDrawFunc(context: *ESContext, func: DrawFunc);
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

pub fn registerDrawFunc(context: *ESContext, func: DrawFunc) {
    unsafe { esRegisterDrawFunc(context, func); }
}

pub fn mainLoop(context: &ESContext) {
    unsafe { esMainLoop(context); }
}