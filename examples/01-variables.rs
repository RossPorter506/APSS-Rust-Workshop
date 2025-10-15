#![allow(unused_variables, unused_assignments, dead_code)]

fn main() {
    // Define a new variable by using the 'let' keyword. Variable names are 'snake_case' by convention.
    // If a type is not specified the compiler will try to infer it. The default integer type is a signed 32-bit integer.
    let example_variable = 10;
    
    // We can explicitly specify the type if we want, for example an unsigned 16-bit integer.
    let x: u16 = 10;
    
    // Now, let's get onto something actually interesting.
    let x = 10;
    x = 20;
    // What?! A compile error?!
    // Variables are immutable by default in Rust, so they can't be modified after being defined...
    // Unless we specify that it should be mutable when we declare it by adding 'mut' after the 'let' keyword. Try it now.

    // Making variables immutable by default makes it easier for readers to see at a glance whether a variable can change value during execution. 
    // It also helps the compiler make optimisations. 


    // If you know the value of a variable at compile time, you may instead want to create a 'constant'. This is the C equivalent of a #define, but with type safety.
    // During compilation, the value of a constant is evaluated and inlined into the code. Constants must have their type specified explicitly.
    // Constant names are in 'SCREAMING_SNAKE_CASE' by convention.
    const CONSTANT_1: i32 = 20;
}

// Global variables are defined using the 'static' keyword (because they will live at the same memory address for the life of the whole program).
// Like constants, they are in 'SCREAMING_SNAKE_CASE' and must have their type defined explicitly.
static GLOBAL_VAR: u32 = 5;

fn example() {
    GLOBAL_VAR += 1;
    // Don't forget, variables are immutable by default! Let's try adding 'mut' to the definition of GLOBAL_VAR again...

    // Why didn't that work?! Global variables can't safely be made mutable because any sort of concurrency (e.g. multithreading, concurrency, interrupts) can cause race conditions!
    // Rust offers some tools to safely deal with global mutable variables (we will discuss later), but in general they should be *avoided at all costs*. 
    // Mutable variables are fine, global variables are fine, but not global mutable variables!
}
