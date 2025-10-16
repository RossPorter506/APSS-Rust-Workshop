#![allow(dead_code, unused_variables, unused_assignments, clippy::needless_return)]

// One of Rust's core tenets is memory safety, which means that memory accesses are restricted by a set of rules. 
// For example, a Rust program is proven to never dereference a null pointer because of the borrowing rules. 

// The Rust compiler only produces memory-safe programs. If your code compiles, then the compiler was able to prove 
// that your program is memory safe. There are, however, memory-safe programs that the Rust compiler can't prove are memory safe. 
// For example, Rust programs that call foreign functions. Rust has interoperability with C, but C is not memory safe, so 
// since C functions can access out of bounds memory, which would make Rust non-memory safe too!
// To solve this dilemma Rust introduces the unsafe keyword. Rust features that are potentially memory-unsafe are 
// restricted to unsafe code blocks. 

// It is the job of the programmer to ensure that any code within these 'unsafe' blocks is still in fact memory-safe.
// 'unchecked' is probably a better name rather than 'unsafe', but oh well.


// Using the 'unsafe' keyword you may do the following potentially memory-unsafe operations:
// - Dereference raw pointers
// - Call unsafe functions (including foreign C functions, etc.)
// - Implement unsafe traits
// - Access or modify mutable static variables
// - Access fields of C-style unions

// Because unsafe Rust allows you to create, modify and dereference raw pointers, you can shoot yourself in the foot 
// in a *truly staggering* number of ways:
// - *Using unsafe does not free you from the rules of the borrow checker.* 
// - *Using unsafe Rust to create multiple mutable references is undefined behaviour.*
// - *Using unsafe Rust to create simultaneous mutable and immutable references is undefined behaviour.* 
// - *Violating the borrowing rules is always undefined behaviour.* 
// - You must be very careful writing unsafe Rust, avoid it entirely if possible.

// An example of an operation that is inherently unsafe is inserting a node in a doubly linked list. 
// There is always a step in which at least one of the two pointers is dangling. Rust references can never dangle, so 
// at some point you must drop down to working with raw pointers during the insertion operation, and working with raw pointers requires unsafe.

// Functions or traits may be marked as unsafe which requires they be called from inside an unsafe block:

fn main() {
    unsafe fn test(n: i32) {
        /* Unsafe if called with n > 20 */
        /* ... */
    }

    fn test2() {
        unsafe{ test(10) } // Ok!
        // test(10); // Err! Unsafe function called outside unsafe block!
    }

    fn test3() {
        test2() // Ok!
    }

    // Note that test2() can be called from outside an unsafe block even though it calls an unsafe function. 
    // Providing safe abstractions over unsafe code is a core pattern in Rust, so for example even though 
    // implementing a doubly linked list may require unsafe, it's the programmer's job to verify what the compiler can't. 
    // Once the programmer is confident the code is safe, they can implement an interface that is not marked with unsafe, 
    // allowing it to be called from 'safe' Rust.
}