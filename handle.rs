extern crate primitive;

use primitive::Translatable;
use primitive::Rotatable;

pub struct LineHandle {
	name: ~str
}
pub struct AreaHandle {
	name: ~str
}

impl Translatable for LineHandle {
    fn translate(&self) {
        println!("{}: Translating line handle", self.name);
    }
}

impl Translatable for AreaHandle {
    fn translate(&self) {
        println!("{}: Translating area handle", self.name);
    }
}

impl Rotatable for AreaHandle {
    fn rotate(&self) {
        println!("{}: Rotating area handle", self.name);
    }
}

