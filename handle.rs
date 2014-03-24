// #[crate_id = "handle#0.1"];
mod primitive;

pub struct LineHandle;
pub struct AreaHandle;

impl LineHandle {
// impl primitive::Translatable for LineHandle {
    pub fn translate(&self) {
    	self.translate();
    }
}

impl AreaHandle {
// impl primitive::Translatable for AreaHandle {
    pub fn translate(&self) {
        println!("Translating area handle");
    }
}

impl AreaHandle {
// impl primitive::Rotatable for AreaHandle {
    pub fn rotate(&self) {
        println!("Rotating area handle");
    }
}

