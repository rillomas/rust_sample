extern crate primitive;

use primitive::Translatable;
use primitive::Rotatable;
use handle::LineHandle;
use handle::AreaHandle;

mod handle;

/// Call translate for given translatable
fn translate<T: Translatable>(tr: &T) {
    tr.translate();
}

/// Call rotate for given rotatable
fn rotate<T: Rotatable>(r: &T) {
    r.rotate();
}

fn changeName(l: &mut LineHandle, name: ~str){
    l.name = name;
}

fn main() {
    // println!("Hello world");
    let mut lh = LineHandle { name: ~"line"};
    let ah = AreaHandle { name: ~"area"};

    lh.translate();
    changeName(&mut lh, ~"red line");
    lh.translate();
    translate(&lh);
    translate(&ah);
    rotate(&ah);
}

