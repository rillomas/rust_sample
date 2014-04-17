extern crate std;
// extern crate primitive;
extern crate esUtil;

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
        width: 300,
        height: 300,
        hWnd: null(),
        eglDisplay: null(),
        eglContext: null(),
        eglSurface: null(),
        drawFunc: null(),
        keyFunc: null(),
        updateFunc: null()
    };
    unsafe { esUtil::esInitContext(&context); }
}

