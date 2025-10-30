#![allow(dead_code, unused_variables)]


fn main() {
    // simple_trait();
    // supertrait();
    // associated_types();
}

// Rust does not support inheritance in the standard way to avoid the complexity and fragility it introduces. 
// Inheritance on objects tightly couples types and can cause issues with shared mutable state, 
// implicit behavior, and deep class hierarchies. 
// Rust instead favours composition (structs containing other types) and traits (interfaces defining shared behavior). 
// This gives code reuse and polymorphism without hidden relationships, making programs safer, clearer, and easier to maintain.

// If you are familiar with interfaces from other languages (like Java), traits will be familiar, though Rust's traits 
// are more powerful (for example new traits can be defined over existing types).

struct Point {
    x: i32, 
    y: i32 
}

// A trait is a list of function prototypes that any implementer should fill out. For example:
trait ToString {
	fn to_string(&self) -> String;
}
// (Note: This trait is actually in the std library already, so no need to reinvent the wheel here.)

// To implement a trait on an object use an impl block:
impl ToString for Point {
	fn to_string(&self) -> String {
        // format! acts like println! but returns a String instead 
	    format!("({}, {})", self.x, self.y)
    }
}

fn simple_trait() {
    // Since Point implements ToString, all Point objects can call .to_string()!
    let p = Point {x: 10, y: 20};
    println!("{}", p.to_string())
}





trait Print : ToString { // Implementing Print requires ToString be implemented already
    // Traits can also have default implementations. Because we require ToString, we can 
    // basically implement this generally without knowing anything else about the object!
    fn print(&self) {
        println!("{}", self.to_string())
    }
}

// Because all the required methods are already implemented (due
// to the default implementation above), we can just leave the impl body empty
impl Print for Point {}

fn supertrait() {
    let p = Point {x: 10, y: 20};
    p.print();
}



// Traits can also make reference to 'associated types' that the implementer can specify.
// For example, here we define a trait that allows for overflow-free multiplication by 
// returning a different type that is large enough to store all possible values:
// e.g. multiplying two 16-bit values and returning a 32-bit value. 
trait WideMultiply {
    type Wide; // The implementer specifies what type 'Wide' is.

    //                            vvvv - This is the type of whatever implements the trait
    fn wide_multiply(self, other: Self) -> Self::Wide;
    //                                     ^^^^^^^^^^ - This refers to the associated type specified above
}

impl WideMultiply for i16 {
    type Wide = i32;
    
    fn wide_multiply(self, other: Self) -> Self::Wide {
        (self as i32) * (other as i32)
    }
}

fn associated_types() {
    let a: i32 = 10.wide_multiply(20);
    println!()
}
