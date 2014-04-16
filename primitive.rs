#![crate_id = "primitive#0.1"]
#![crate_type = "lib"]
/// Module containing primitives

pub struct Point { 
    x: f32,
    y: f32
}

pub struct Size {
    width: f32,
    height: f32 
}

/// Somthing that can be rotated
pub trait Rotatable {
    fn rotate(&self);
}

/// Somthing that can be translated
pub trait Translatable {
    fn translate(&self);
}

