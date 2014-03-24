/// Module containing primitives

/// Somthing that can be rotated
pub trait Rotatable {
    fn rotate(&self);
}

/// Somthing that can be translated
pub trait Translatable {
    fn translate(&self);
}

