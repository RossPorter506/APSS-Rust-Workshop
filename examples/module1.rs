use crate::module2::Module2Struct;

// Create a struct that can be created outside of this file, but 
// the internal fields are private and can't be read. 
pub struct Module1Struct {
	a: i32,
	b: String,
    c: bool,
}

// Likewise, this function can be called from other files when imported
pub fn convert(other: Module2Struct) -> Module1Struct {
	Module1Struct{a: other.a, b: other.b, c: false}
}
