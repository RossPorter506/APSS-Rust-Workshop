#![allow(dead_code, unused_variables, clippy::to_string_trait_impl)]

// The real power of traits comes from defining functions that can accept any type that implements a trait:
// This is Rust's version of 'polymorphism'.

// Polymorphism comes in two types: Static and dynamic polymorphism. 
// Static polymorphism has the compiler at compile time creating a copy of the function for each type it's 
// called with ('monomorphisation'). 
// This increases code size, but has no runtime overhead.

// Dynamic polymorphism instead uses pointers and function tables to use the same function with all different typed objects. 
// It has a runtime performance cost because you have to follow pointers to call each method, etc.. 

// We'll discuss static polymorphism here.

struct Point {
    x: i32,
    y: i32,
}
impl ToString for Point {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

// All I know about 'thing' is that it implements ToString, so that's all we can do with it.
// If we need to add additional requirements we can add other traits via: impl ToString + Clone + ...
fn print_something(thing: impl ToString) {
	println!("{}", thing.to_string());
    // Sidenote: If all you wanted to do is print a type without needing the debug ':?' thing, 
    // you would just implement Display rather than implementing ToString and doing this.
}

fn main() {
    // The compiler will create a copy of the function for each type that uses it at compile-time. 
    // This is called monomorphization. It has zero runtime overhead compared to manual implementations, but 
    // increases executable size, especially if the function is used with many different types.
    print_something(Point{x: 5, y:2});
    print_something(10_i32);

    // The impl keyword can also be used in the return position. This can be used to hide implementation 
    // details from the caller, particularly useful for writing libraries where such details may change 
    // between versions, or incorporating mock implementations into your program for testing purposes:
    fn return_something_stringable() -> impl ToString {
        0_u32
    }
    
    // I don't know what type it is, but I can call to_string() on it
    let stringable = return_something_stringable();
    println!("{}", stringable.to_string());
}

// Static polymorphism will be touched on again in the Generics example.
