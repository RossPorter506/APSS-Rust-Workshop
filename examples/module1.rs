use crate::module2::Module2Struct;

// Create a public struct (can be created outside of this file), but 
// the internal fields are private and can't be read.
pub struct Module1Struct {
	a: i32,
	b: String,
    c: bool,
}
// Because the fields of Module1Struct are private they can't be accessed outside of the file it was defined in,
// not even to instantiate an object of this struct! This is useful for structs that require some sort of data validation.
// e.g. we might want to ensure that the string field is always non-empty.
// In this case we can write a public constructor that anyone can use that returns a properly valid Module1Struct
impl Module1Struct {
    pub fn new(a: i32, b: String, c: bool) -> Module1Struct {
        if b.is_empty() {
            Module1Struct { a, b: "N/A".to_string(), c }
        } else {
            Module1Struct { a, b, c }
        }
    }
}

// Likewise, this function can be called from other files when imported
pub fn convert(other: Module2Struct) -> Module1Struct {
	Module1Struct{a: other.a, b: other.b, c: false}
}
