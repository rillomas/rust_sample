#![crate_id = "primitive#0.1"]
#![crate_type = "lib"]
/// Module containing primitives

/// Somthing that can be rotated
pub trait Rotatable {
    fn rotate(&self);
}

/// Somthing that can be translated
pub trait Translatable {
    fn translate(&self);
}

