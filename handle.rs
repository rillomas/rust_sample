extern crate primitive;

use primitive::Translatable;
use primitive::Rotatable;
use primitive::Point;
use primitive::Size;

/// A handle shaped like a line
pub struct LineHandle {
	name: ~str,
	start: Point,
	end: Point
}

/// A handle shaped like a rectangular area
pub struct AreaHandle {
	name: ~str,
	center: Point,
	size: Size
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

