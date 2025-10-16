#![allow(unused_variables, unused_assignments, dead_code, clippy::no_effect)]

fn main() {
    // arrays();
    // vectors();
    // strings();
    // slices();
    // tuples();
}

fn arrays() {
    /* Arrays */
    let arr_of_i32: [i32; 10] = [0,1,2,3,4,5,6,7,8,9];
    let arr_of_u16: [u16; 10] = [0; 10]; // Bulk initialization

    // Array accessing can be performed with square brackets like in C. 
    // If the index is out of bounds the program panics. 
    // Arrays must be indexed with a usize.

}

fn vectors() {
    /* Vectors */
    // A Vector is a heap-allocated collection of values that may grow or shrink.
    // Vectors can be constructed using the vec! macro shorthand.
    let v1: Vec<u8> = Vec::new(); 		    // Empty Vec
    let mut v2: Vec<u8> = vec![1,2,3,4];	// Vec from array
    v2.push(5);                             // Add a number to the end of v2
}

fn strings() {
    /* Strings */
    // Rust text is unicode by default. This means 'char' is a single valid unicode character (up to four bytes, so effectively a u32). 
    // All string types use UTF-8 encoding however, so there is no size overhead for encoding pure ASCII strings when compared with, say, C.

    // Rust has two basic text types: &str and String:
    /* &str */
    // A string slice represents a non-growable collection of characters, similar to an array. You can think of it like a [char; N].

    // Note the '&' in the type name - this is because we are always technically *borrowing* a string slice, since the actual data is 
    // baked into the executable's text section. As a result its 'owned' by the program itself rather than any variable, so 
    // we merely get a handle to that actual memory location, hence a borrow. See Slices for more information.
    let fixed: &str = "hello";
    // Because &str is really just a handle to a memory location, they don't have many operations defined on them. 
    // For instance, you can't add two &str together.


    /* String */
    // String represents a growable string of characters allocated on the heap, like a Vec<char>.
    // Because strings are growable, they can have elements appended or prepended, and &str can be added to them:
    let s = String::from("Hello");
    println!("{}", s+" world!"); 	// Prints "Hello world!" 

    // Note that since a String owns its underlying data, that makes adding two Strings tricky with regards to ownership. Who owns the resulting string?
    let s1 = String::from("Hello");
    let s2 = String::from(" world!");
    // let s3 = s1 + s2; 	// Compile error! Where is s1 + s2 stored?

    // s1+s2 needs to be stored somewhere on the heap. The compiler could choose one of s1 or s2 to extend and copy the other into it, but which? 
    // It's an arbitrary choice, and also requires the compiler to silently implicitly cast one of the Strings to a &str and hope that's what the user wanted. 
    // Rust doesn't like to implicitly cast types without the user's knowledge.
    // Alternatively, it could silently make a new String and cast both s1 and s2 to &str and write them into the new String. 
    // This is less arbitrary than just picking one to extend, but requires twice as many implicit casts, plus an additional heap allocation!
    // Both options aren't great and have potential downsides, so rather than picking one, the compiler leaves it up to the user to specify 
    // which they want, leaving this ambiguous case as a compile error.

    // Instead, we make it clear whether s1 or s2 will be extended by converting the other into a &str:
    let s1 = String::from("Hello");
    let s2 = String::from(" world!");
    let s3 = s1 + s2.as_str(); 
    // Note that s3 takes ownership of s1 here, so it will not be available afterwards. Remember, you can always call .clone() to produce 
    // a copy rather than moving s1 into s3:
    // let s3 = s1.clone() + s2.as_str(); // s1 continues to be usable afterwards because the data was cloned (copied), instead of being moved out.


    /* ASCII Text */
    // Simple ASCII text is supported by the u8 type, which can be assigned ASCII values using the ‘byte’ prefix:
    let ascii_h: u8 = b'H';

    // To make an ASCII-equivalent to a &str you could use an array of u8's:
    let ascii_str: &[u8; 23] = b"This like an ASCII &str";

    // To make an ASCII-equivalent to a String you might use a Vec<u8>:
    let mut ascii_string: Vec<u8> = Vec::from(b"Hello");
    ascii_string.extend(b" world!");
    // Note that attempting to print a Vec<u8> will print the numerical values of the elements.
    // External libraries are available for ASCII string types
}

fn slices() {
    /* Slices */
    // Slices are a way to take a dynamically-sized 'view' into some contiguous region of memory.
    // For example, let's say I have some large array, but I only want to pass a small piece of that array into a function. 
    // What I could do is provide a pointer to the start of the section I want to provide, and list how many elements there are after the start. 
    // This is a slice: An array pointer plus a length. (Sometimes this is known as a 'fat pointer').

    // Because a slice really only exists as a reference to some other piece of memory it's *always* either a borrow or a mutable borrow,
    // and so the syntax for a slice of T's is &[T] or &mut [T] respectively.
    let arr = [10_i32; 100];
    let immut_slice: &[i32] = &arr[0..10]; // Get a slice of the first ten elements (up to but not including index 10)
    println!("{}", immut_slice[5]);

    // Slices can be very useful, as any contiguous collection of elements (array, Vec, String, etc.) can easily produce a slice. 
    // This makes them great for taking as a function parameter - if a function takes a slice as an argument the user could easily use 
    // the function with an array, vector or String type. Function syntax will be discussed in the next section.
    
    fn print_bytes(elems: &[u8]) {
        // ...
    }

    let arr = [0u8; 10];
    let vec: Vec<u8> = vec![1,2,3,4];
    let string = String::from("Hello world!");

    print_bytes(arr.as_slice());
    print_bytes(vec.as_slice());
    print_bytes(string.as_bytes());
}

fn tuples() {
    /* Tuples */

    // Tuples are finite collections of elements of possibly disparate types. 
    // A tuple is declared using round brackets. These are typically used as temporary containers. 
    // For example, returning multiple items from a function, assigning multiple variables at once:

    fn multiple_returns() -> (i32, bool) { 
        todo!();
    }
    let (x,y) = multiple_returns();

    // Tuples can be indexed using the following notation:
    let x = multiple_returns();
    x.0; // i32
    x.1; // bool

    // Rust often uses the 'empty tuple' () as a placeholder for 'nothing'. For example, functions that return nothing actually return the empty tuple.
}
