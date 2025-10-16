#![allow(unused_variables, unused_assignments, dead_code)]

// One of Rust's foundational features that makes it unique is the concept of ownership and the borrowing rules.
// This is what allows Rust the memory-safety of a garbage-collected language while maintaining C-like performance.

// Other languages either:
// - Trust you to manually manage memory (C, C++) which is fast but error-prone, or
// - Use a garbage collector (Java, Python), which is safe but adds runtime cost and time unpredictability.

// Rust does neither. It enforces correctness at compile time. The compiler tracks which variable "owns" a value, who can borrow it, and for how long. 
// That’s the price: A bit more thinking up front. The reward: Memory safety and concurrency safety with zero runtime overhead.
// As a bonus, it can also be used to model systems that are unrepresentable in other languages! More on this in later examples...

// Rust’s ownership and borrowing rules exist to eliminate an entire class of memory bugs without needing a garbage collector:
// They prevent:
// - Use-after-free (accessing freed memory)
// - Double free (freeing the same memory twice)
// - Data races (unsynchronized mutation across threads)

fn main() {
    // In Rust, each piece of data has *exactly* one owner at all times. When the owner goes out of scope, the data is cleaned up ('dropped').
    // Why can't we have multiple owners? 
    // Only the owner can write to a value, so multiple owners could lead to a data race! e.g. two threads trying to increment the same value.


    // Let's open up an arbitrary scope first to clearly delineate this example:
    {
        // x is first defined here and owns the data '10'.
        let x = 10;

    } // <-- x goes out of scope here, so the value '10' is dropped


    // So if each piece of data has exactly one owner, what happens if we do this?
    {
        let x = 20; // x owns 20.
        let y = x;  // who owns 20 now?
    }

    // We could give y a copy of 20, and they can both own their respective version, provided it's cheap to create a copy, of course.
    // In this case, this is precisely what Rust does. 

    // ** If a type is inexpensive to construct, then Rust will automatically create a copy when we assign another variable to it. **

    // How does the compiler know if a type is cheap to copy? This is handled through the `Copy` trait, which we will discuss later.
    // Most primitive types are `Copy`.

    // What if a type *isn't* cheap to copy, though? Like a really long string? 
    // We don't want to silently copy the whole array, that would tank performance!
    // Well... why don't we just say the new variable owns the data?

    {
        // x owns the string
        let x = "This is a really long string, it really takes up a lot of memory. You wouldn't want the compiler to be silently copying all of this, it would be really bad for performance. I mean really, it just keeps going, and going and going. You'd think it would have stopped by now but it just keeps going on and on and on forever".to_string(); 
        let y = x; // ownership of the string is transferred to y.
        println!("{y}"); // we can access the data through y because it's the owner now

        // Once y took ownership of the data x no longer owns anything, So the compiler throws an error if we try to access it!
        //println!("{}", x); // Compile error! Use of moved value
    }

    // If we are okay with taking a performance hit, we can ask the compiler to copy the item for us by calling .clone(). This copies the string.
    // This is usually the easiest way to get around 'used after move' or 'borrowed after move' errors. 
    {
        // x owns the string
        let x = "This is a really long string, it really takes up a lot of memory. You wouldn't want the compiler to be silently copying all of this, it would be really bad for performance. I mean really, it just keeps going, and going and going. You'd think it would have stopped by now but it just keeps going on and on and on forever".to_string(); 
        let y = x.clone(); // A copy of x's data is created and given to y. This could be an expensive operation!
        println!("{y}");
        println!("{x}");
    }


    // Why can't we have multiple owners? 
    // Only the owner can write to a value, so multiple owners could lead to a data race! e.g. two threads trying to increment the same value
    // Okay, well what if we just want to reference the value, but we promise not to modify it? Rust has syntax for this!
    {
        let x = "Imagine a really long string again".to_string(); // x owns the string
        let y = &x; // y *borrows* the contents of x
        let z = &x; // z *borrows* the contents of x
        // Nobody may modify the data while the borrows are in scope (including x!)
    } 


    // Okay... but not being able to modify a value is pretty rough. What if we want to modify the value, but we
    // promise only one variable will write to it (and no-one else will read it)?
    // Rust also has syntax for this! We can *borrow mutably* using `&mut`.
    {
        let mut x = "Imagine a really long string again".to_string(); // x owns the mutable string
        let y = &mut x; // y *mutably borrows* the string
        y.clear(); // y can mutate the data
        // println!("{}", &x); // Compile error! We can't borrow x again, because y still has it *mutably* borrowed!
        println!("{}", y);
    }

    // To summarise:
    // At any point, every piece of data is either exclusively:
    // - Owned
    // - Borrowed an unlimited number of times (during which it can't be written to)
    // - Mutably borrowed once (only the mutable borrower may read or write to it)
}
