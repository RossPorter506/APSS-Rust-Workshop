#![allow(dead_code, unused_variables, clippy::match_like_matches_macro)]

// Rust allows for object-oriented design, however it may be slightly different to 
// what you're used to coming from C++ or similar.

fn main() {
    // struct_methods();
    // enum_methods();
}

#[derive(Debug)] // This lets us debug print this struct using the ':?' syntax. More on this later.
struct Point {
	x: i32,
	y: i32,
}

// We can implement methods on Point here. 
impl Point {
	// Static methods.
	//    vv Notice no 'self' here
    fn new() -> Point { 
        Point{x: 0, y: 0}
    }

    // Non-static methods
    //                      vvvvv Borrow the object for the duration of the method 
    fn distance_from_origin(&self) -> i32 {
        (self.x*self.x + self.y*self.y).isqrt()
    }

    //                 vvvv *Mutably* borrow the object
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
    // Static methods are called with '::'
    let mut p = Point::new(); 
    println!("{p:?}");

    // Non-static methods are called using '.' on an instance of the object
    p.update_position(10, 20);
    println!("{p:?}");

    println!("{}", p.distance_from_origin());

    p.zero_x();
    println!("{p:?}");

    let (x, y) = p.into_tuple(); 
    // p.distance_from_origin(); // Compile error! Use of moved value
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
