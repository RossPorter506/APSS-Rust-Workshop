#![allow(dead_code, unused_variables)]

// Foreign traits or types are those defined outside the current project. 
// Local traits or types are those defined inside the project. 
// You may implement a local trait on a local type, foreign traits on a local type, 
// a local trait on a foreign type, but *not* a foreign trait on a foreign type, as this may lead to conflicting implementations.

// For example, let's implement our own (local) trait on a foreign type (from the standard library):
trait Print {
    fn print(&self);
}

impl Print for f64 {
    fn print(&self) {
        println!("{self}");
    }
}

// If however we tried to implement the std library's ToString trait on the std library's f64 type, compile error!
// impl std::string::ToString for f64 {/* ... */} // Compile error! Only traits defined in the current crate can be implemented on foreign types

// There is a way to get around this, using a 'wrapper' type and the Deref and DerefMut traits. This will be discussed briefly later.

fn main() {

}
