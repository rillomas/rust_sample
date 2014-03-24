mod primitive;
mod handle;

impl primitive::Translatable for handle::LineHandle {
    fn translate(&self) {
        println!("Translating line handle");
    }
}

impl primitive::Translatable for handle::AreaHandle {
    fn translate(&self) {
        println!("Translating area handle");
    }
}

impl primitive::Rotatable for handle::AreaHandle {
    fn rotate(&self) {
        println!("Rotating area handle");
    }
}

/// Call translate for given translatable
fn translate<T:primitive::Translatable>(tr: &T) {
    tr.translate();
}

fn rotate<T:primitive::Rotatable>(r: &T) {
    r.rotate();
}

fn main() {
    let lh = handle::LineHandle;
    let ah = handle::AreaHandle;

    translate(&lh);
    translate(&ah);
    rotate(&ah);
}

