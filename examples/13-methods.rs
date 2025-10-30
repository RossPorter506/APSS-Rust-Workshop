#![allow(dead_code, unused_variables, clippy::match_like_matches_macro)]

// Rust allows for object-oriented design, however it may be slightly different to 
// what you're used to, coming from C++ or similar.

fn main() {
    // methods();
    // struct_methods();
    // enum_methods();
    // shorthand();
}

fn methods() {
    // In Rust, methods are just functions that are associated with a type.
    // This means they also interact with the ownership and borrowing system in exactly the same way.

    // For example, the integer types provide functions for various arithmetic operations.
    // They are called by specifying the type followed by '::'
    let ten = i32::isqrt(100);
    println!("{ten}");

    // What would the function signature of this look like? We could write it as:
    //                        vvv - Returns an i32
    // fn isqrt(self: i32) -> i32
    //          ^^^^^^^^^ - Takes an i32

    // Because methods almost always take an instance of the type as a parameter, Rust has
    // shorthand for this: If we name the first parameter 'self' the type is assumed to be the same type
    // the method is a defined on (in this case, i32). 
    // Likewise, if we want to refer to the type that the method is defined on, we can use 'Self' (instead 
    // of 'i32'). So we could just as well write the function signature as:
    //                       vvvv - The parent type itself, 'i32' in this case.
    // pub fn isqrt(self) -> Self
    //              ^^^^ - A value of type Self (i.e. i32), '100' in this case.
    // And indeed, if you mouse over isqrt() this is very similar to the actual definition!

    
    // Another shorthand available is the 'dot' syntax (you might be familiar with from other languages), 
    // which just passes the dotted value as the first argument. This also removes the need to specify 
    // the type the method is defined on:
    let also_ten = 100_i32.isqrt();
    println!("{also_ten}");

    // If a method doesn't require an instance of the parent type we call it a 'static' method. 
    // If it does, we call it an 'instance' or 'non-static' method.
}







#[derive(Debug)] // This lets us debug print this struct using the ':?' syntax. More on this in the 'common traits' example.
struct Point {
	x: i32,
	y: i32,
}

// We can implement methods on Point here. 
impl Point {
	// Static methods
	//    vv Notice no 'self' here
    fn new() -> Point { 
        Point{x: 0, y: 0}
    }

    // Non-static methods
    //                      vvvvv Borrow an instance of a Point for the duration of the method 
    fn distance_from_origin(&self) -> i32 {
        (self.x*self.x + self.y*self.y).isqrt()
    }

    //                 vvvv *Mutably* borrow an instance of Point
    fn update_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    //        vvvv Mutably borrow the object
    fn zero_x(&mut self) {
        self.x = 0;
    }

    //            vvvv Note: No borrow here. This method *transfers ownership* of the object to the method
    fn into_tuple(self) -> (i32, i32) {
        (self.x, self.y)
    }// <-- self goes out of scope here, as it wasn't borrowed. We've effectively 'consumed' the Point object.
}

fn struct_methods() {
    // To specify a method we say the type name followed by '::'
    let mut p = Point::new(); 
    println!("{p:?}");

    // We can call any methods this way, including non-static ones.
    let dist = Point::distance_from_origin(&p);
    // But Rust provides the more familiar 'dot' syntax as syntactic sugar: The item being 'dotted' becomes the first parameter.
    let dist = p.distance_from_origin();

    // Since the dot syntax is a bit shorter, typically non-static methods are called this way.
    p.update_position(10, 20);
    println!("{p:?}");

    println!("{}", p.distance_from_origin());

    p.zero_x();
    println!("{p:?}");

    let (x, y) = p.into_tuple(); 
    
    // p.distance_from_origin(); // Compile error! Use of moved value.
    // Because Point doesn't implement Copy, we consumed 'p' when we called into_tuple() and have nothing left now.
}

enum DayOfWeek {
	Mon, Tues, Wed, Thur, Fri, Sat, Sun,
}
impl DayOfWeek {
    fn is_weekday(&self) -> bool {
        use DayOfWeek::*;
        match self {
            Sat | Sun => false,
            _ => true,
        }
    }
}

fn enum_methods() {
    let d = DayOfWeek::Mon;
    println!("Monday is a weekday: {}", d.is_weekday());
}
