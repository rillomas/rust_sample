extern crate primitive;

use primitive::Translatable;
use primitive::Rotatable;

mod handle;
/// Call translate for given translatable
// fn translate<T: handle::Translatable>(tr: &T) {
// fn translate<T: primitive::Translatable>(tr: &T) {
fn translate<T: Translatable>(tr: &T) {
// fn translate(tr: &primitive::Translatable) {
    tr.translate();
}

/// Call rotate for given rotatable
// fn rotate<T: handle::Rotatable>(r: &T) {
// fn rotate<T: primitive::Rotatable>(r: &T) {
fn rotate<T: Rotatable>(r: &T) {
// fn rotate(r: &primitive::Rotatable){
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

