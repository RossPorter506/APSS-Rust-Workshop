#![allow(dead_code, unused_variables)]




fn main() {
    // Rust supports the concept of generic types, similar to C++'s template system. 
    // For example, suppose we want to write a function that adds two values:
    fn add(a:i32, b:i32) -> i32 {
        a + b
    }

    // This is functional, but what if we want to use u32's? Are we really going to write an 
    // identical function that takes and returns u32's? Instead, we use Rust's generic syntax:
    fn add2<T>(a:T, b:T) -> T {
        a + b // Compile error!
    }
    // This doesn't work because addition may not be defined for this generic type T (e.g. if T = char).

    // We need to fix our function by adding a 'Trait bound' to tell the compiler that actually we only 
    // want this to be generic over types that have implemented the Add trait.
    fn add3<T: std::ops::Add<Output=T>>(a:T, b:T) -> T {
        a + b
    }

    // Besides functions, structs may also be generic:
    struct Point<T> {
        x: T,
        y: T,
    }
    let p_float = Point::<f32>{x: 1.0, y: 0.0};
    let p_int   = Point::<i64>{x: 123, y: 456};

    // impl blocks may also be applied to specific instances of Point. This allows some subtypes of Point 
    // to access special functions depending on the type:
    impl Point<f32> {
        fn is_nan(&self) -> bool {
            self.x.is_nan() || self.y.is_nan()
        }
    }

    p_float.is_nan();
    // p_int.is_nan(); // Compile error! is_nan() doesn't exist for Point<i64>

    // Again, we don't want to have to implement this for f64 too, so we could use the Float trait from the
    // num crate which is defined for all floating point types (f32, f64):
    // impl<F: num::Float> Point<F> {
    //     fn is_nan(&self)  -> bool {
    //         self.x.is_nan() || self.y.is_nan()
    //     }
    // }
}
