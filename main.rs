// extern mod handle;
mod primitive;
mod handle;

/// Call translate for given translatable
// fn translate<T:primitive::Translatable>(tr: &T) {
// fn translate(tr: &primitive::Translatable) {
//     tr.translate();
// }

/// Call rotate for given rotatable
// fn rotate<T:primitive::Rotatable>(r: &T) {
// fn rotate(r: &primitive::Rotatable){
//     r.rotate();
// }

fn main() {
    let lh = handle::LineHandle;
    let ah = handle::AreaHandle;

    lh.translate();
    ah.translate();
    ah.rotate();
    // translate(&lh);
    // translate(&ah);
    // rotate(&ah);
}

