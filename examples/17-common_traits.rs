#![allow(dead_code, unused_variables, clippy::to_string_trait_impl)]

fn main() {
    clone();
    copy();
    debug_display();
    default();
    deref();
    eq();
    ord();
    into_from();
}


fn clone() {
    // Types that could feasibly create a duplicate of itself should implement Clone. The operation may or may not be expensive.

    // An example of a type that should be Clone: An array (whose elements are also Clone)
    // An example of a type that should not be Clone: A type that represents some limited physical resource, like a file handle or a microcontroller pin.

    #[derive(Clone)] 
    struct Point {x: i32, y: i32}

    let p1 = Point {x: 10, y: 20};
    let p2 = p1.clone();
}

fn copy() {
    // Types that can be inexpensively duplicated by simply copying bits should implement Copy. 
    // For example, for a u32 a copy is exactly as expensive as a move. Rust implements Copy on most primitive types. 
    // The Rust compiler will insert copy operations whenever ownership would otherwise be transferred.
    // Copy requires Clone

    // An example of a type that should be Copy: A simple Enum
    // An example of a type that should not be Copy: A large array of values

    #[derive(Debug, Copy, Clone)] 
    enum MaybeBool {
        True, 
        False, 
        Maybe
    }

    let a = MaybeBool::True;
    let b = a;

    println!("{a:?}, {b:?}");
}

fn debug_display() {
    // Debug and Display
    // The Debug trait is used to print types in a programmer-facing manner, for debugging purposes.

    // Debug can be automatically derived if all fields also implement Debug.
    #[derive(Debug)] 
    struct Point {x: i32, y: i32}
    
    // The automatic implementation simply prints the name of the object followed by each field, e.g.:
    // 'Point { x: 2, y: 5 }'
    let a = Point {x:2, y:5};
    println!("{a:?}");

    // Display is similar, but designed for explicitly non-debug printing. Display is not automatically derivable. An implementation may look like:
    use std::fmt::{Display, Formatter, Error};
    impl Display for Point {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let a = Point {x:2, y:5};
    println!("{a}"); 		// Prints as: (2, 5)
}

fn default() {
    // The Default trait allows an object to be constructed with default values. This can be automatically derived if all fields implement Default. 
    // The default value for numerical types is zero, false for booleans, the empty string for textual types, null for char, etc.

    #[derive(Debug, Default)] // Automatically derive Default
    struct Point {
        x: i32,
        y: i32,
    }

    // Alternatively implement it yourself to set a custom default:
    // impl Default for Point {
    //     fn default() -> Self {
    //         Point{x: 2, y: 5}
    //     }
    // }

    let p = Point::default();
    println!("{p:?}");
}

fn deref() {
    // Deref and DerefMut
    // In some cases it may make sense for a type to be invisibly coerced to another type. If this is the case, then consider implementing Deref and DerefMut.
    // Implementing Deref<Target=U> for some type T allows 'dereferencing' using the * operator, but also allows the compiler to coerce &T to &U automatically,
    // and allows you to call all immutable methods of U on T:

    struct Wrapper{i: i32}

    impl std::ops::Deref for Wrapper {
        type Target = i32;
        fn deref(&self) -> &Self::Target {
            &self.i
        }
    }

    let w = Wrapper{i: 10};
    let _ = w.is_positive();

    // DerefMut is much the same, but is related to mutable references rather than immutable ones. 
    // Implementing DerefMut<Target=U> on some type T allows 'dereferencing' using the * operator to get the inner value, 
    // but also allows the compiler to coerce &mut T to &mut U automatically, and allows you to call all mutable methods of U on T.

    // Deref and DerefMut should be used sparingly, and only when you want some type to behave identically to another. 
    // Because all the methods of the inner value are made available, the wrapper cannot maintain any encapsulation or maintain any extra invariants on the inner data.
}

fn eq() {
    // PartialEq and Eq
    // Implementing PartialEq allows comparison between elements of a type using '=='
    // The PartialEq trait should be defined on types where equality is well-behaved enough to be a partial equivalence relation. That is: 
    // If a == b then b == a 
    // If a == b and b == c then a == c.
    // PartialEq can be automatically derived if all fields implement PartialEq

    #[derive(PartialEq)]
    struct Wrapper{val: f64}

    let (x, y) = (Wrapper{val: 10.0}, Wrapper{val: 20.0});

    if x != y {
        println!("x is not equal to y");
    }

    // Eq represents a full equivalence relation, which adds the additional restriction that each element must be equal to itself. In other words:
    // If a == b then b == a 
    // If a == b and b == c then a == c.
    // a == a for all elements a
    // Eq requires PartialEq.
    // Eq can be automatically derived if all fields implement Eq.
    // Eq does not add any additional functionality, but can be used to mark functions or algorithms that require types 
    // with a full equivalence relation to work properly (e.g. sorting).

    // The distinction between PartialEq and Eq may seem redundant, but floating point types can only implement PartialEq, since NaN is not equal to itself.

    #[derive(PartialEq, Eq)]
    struct Wrapper2{val: bool}

    let (a, b) = (Wrapper2{val: true}, Wrapper2{val: true});

    if a == b {
        println!("a and b are equal");
    }
}

fn ord() {
    // PartialOrd and Ord
    // Implementing PartialOrd enables comparisons using <, <=, > or >= 
    // PartialOrd should be implemented on types where ordering is well-behaved enough to be a partial order, that is:
    // If a ~ b and b ~ c then a ~ c, where ~ could be <, ==, or >
    // If a < b then b > a
    // If a > b then b < a
    // PartialOrd requires PartialEq
    // PartialOrd is automatically derivable if all fields implement PartialOrd.
    #[derive(PartialOrd, PartialEq)]
    struct Wrapper{val: i32}

    let (x, y) = (Wrapper{val: 10}, Wrapper{val: 20});

    if x < y {
        println!("x is less than y")
    }

    // Ord should be implemented on types that fulfil a total order, which is a partial order plus the requirement 
    // that each element must be somehow related to every other element. That is:
    // If a ~ b and b ~ c then a ~ c, where ~ could be <, ==, or >
    // If a < b then b > a
    // If a > b then b < a
    // Either a > b or a < b or a == b 
    // Ord requires PartialOrd and Eq
    // Ord is automatically derivable if all fields implement Ord
    // Ord does not add any additional functionality, but can be used to mark functions or algorithms that require 
    // types with a total ordering to work properly (e.g. sorting).
}

fn into_from() {
    // Into and From
    // As discussed in the casting example, the From and Into traits are used to cast between types:
    let x: i32 = 10;
    let y: i64 = x.into(); 	// Type conversion inferred by compiler
    let y = i64::from(x); // Type supplied by user

    // The Rust compiler has a blanket implementation of Into for any type that implements From, so 
    // users should implement From to get the equivalent Into implementation for free. For this reason we will only look at From here.

    // From takes one generic argument T, which represents the type being cast from. 
    // The trait is implemented on the type being cast to. For example, below we implement From<(i32, i32)> on Point, 
    // so we can call Point::from(t) where t has type (i32, i32).

    struct Point {
        x: i32,
        y: i32,
    }
    impl From<(i32, i32)> for Point {
        fn from(tuple: (i32, i32)) -> Self {
                Point {x: tuple.0, y: tuple.1}
        }
    }
    let t: (i32, i32) = (10, 0);
    let p: Point = t.into();
    let p = Point::from(t);
}