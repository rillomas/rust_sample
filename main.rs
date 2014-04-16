extern crate primitive;

use primitive::Translatable;
use primitive::Rotatable;

mod handle;

/// Call translate for given translatable
fn translate<T: Translatable>(tr: &T) {
    tr.translate();
}

/// Call rotate for given rotatable
fn rotate<T: Rotatable>(r: &T) {
    r.rotate();
}

fn main() {
    // println!("Hello world");
    let lh = handle::LineHandle { name: ~"line"};
    let ah = handle::AreaHandle { name: ~"area"};

    translate(&lh);
    translate(&ah);
    rotate(&ah);
}

