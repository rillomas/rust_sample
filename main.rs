extern crate std;
// extern crate primitive;
// extern crate esUtil;
extern crate gl2;
extern crate egl;
extern crate winapi;
use std::ptr::null;
// use std::io;


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

// extern "cdecl" fn drawHandler(context: *esUtil::ESContext) {
//     // Set the viewport
//     unsafe { gl2::viewport(0,0,(*context).width, (*context).height); }
//     // Clear the color buffer
//     gl2::clear(gl2::COLOR_BUFFER_BIT);
//     unsafe { egl::swap_buffers((*context).display, (*context).surface); }
// }

fn main() {
    let context = winapi::WindowContext {
        width: 320,
        height: 240,
        handle: null()
    };
    let title = ~"Sample Window";
    let result = winapi::create_window(&context, title);
    if !result {
        println!("Failed to create window");
        return;
    }

    winapi::main_loop(&context);
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
    // rustc .\main.rs -L . -C link-args="-lgdi32"
    // let context = esUtil::ESContext {
    //     userData: null(),
    //     width: 0,
    //     height: 0,
    //     hWnd: null(),
    //     display: null(),
    //     context: null(),
    //     surface: null(),
    //     drawFunc: null(),
    //     keyFunc: null(),
    //     updateFunc: null()
    // };
    // esUtil::initContext(&context);
    // println!("Enter line to create window");
    // match io::stdin().read_line() {
    //     Ok(s) => println!("typed {}", s),
    //     Err(e) => println!("error: {}", e)
    // }
    // let result = esUtil::createWindow(&context, ~"OpenGL ESをテスト中", 320, 240, esUtil::WINDOW_RGB);
    // if !result {
    //     println!("Failed to create window");
    //     return;
    // }

    // gl2::clear_color(1.0f32, 1.0f32, 1.0f32, 0.0f32);
    // esUtil::registerDrawFunc(&context, drawHandler as esUtil::DrawFunc);
    // esUtil::mainLoop(&context);
}
