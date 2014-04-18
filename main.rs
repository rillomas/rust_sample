extern crate std;
// extern crate primitive;
extern crate esUtil;
extern crate gl2;
use std::ptr::null;


// use primitive::Translatable;
// use primitive::Rotatable;
// use primitive::Point;
// use primitive::Size;

// use handle::LineHandle;
// use handle::AreaHandle;

// mod handle;

// /// Call translate for given translatable
// fn translate<T: Translatable>(tr: &T) {
//     tr.translate();
// }

// /// Call rotate for given rotatable
// fn rotate<T: Rotatable>(r: &T) {
//     r.rotate();
// }

// fn changeName(l: &mut LineHandle, name: ~str){
//     l.name = name;
// }

extern "cdecl" fn drawHandler(context: *esUtil::ESContext) {
    unsafe {
        // Set the viewport
        let deref = *context;
        gl2::viewport(0,0,deref.width, deref.height);
    }
    // Clear the color buffer
    gl2::clear(gl2::COLOR_BUFFER_BIT);
}

fn main() {
    // println!("Hello world");
    // let mut lh = LineHandle {
    //     name: ~"line",
    //     start: Point{ x: 0.0f32, y: 0.0f32},
    //     end: Point{ x: 0.0f32, y: 0.0f32}
    // };
    // let ah = AreaHandle {
    //     name: ~"area",
    //     center: Point{ x: 0.0f32, y: 0.0f32},
    //     size: Size{ width: 1.0f32, height: 1.0f32}
    // };

    // lh.translate();
    // changeName(&mut lh, ~"red line");
    // lh.translate();
    // translate(&lh);
    // translate(&ah);
    // rotate(&ah);

    // Currently must build by the following
    // rustc .\main.rs -L . -C link-args="-les_util -llibEGL  -llibGLESv2 -lgdi32"
    let context = esUtil::ESContext {
        userData: null(),
        width: 0,
        height: 0,
        hWnd: null(),
        eglDisplay: null(),
        eglContext: null(),
        eglSurface: null(),
        drawFunc: None,
        keyFunc: None,
        updateFunc: None
    };
    esUtil::initContext(&context);
    esUtil::createWindow(&context, ~"OpenGL ESをテスト中", 320, 340, 0);

    gl2::clear_color (1.0f32, 1.0f32, 1.0f32, 0.0f32);
    esUtil::registerDrawFunc(&context, drawHandler);
    // gl2::clear_color(1.0f32, 1.0f32, 1.0f32, 0.0f32);
    esUtil::mainLoop(&context);
}
