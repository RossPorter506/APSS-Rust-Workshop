#![allow(dead_code, unused_variables, clippy::needless_return)]

fn main() {
    // A subset of Rust functionality can be used at compile time. For example, a function marked as a 'const fn' can be called wherever a constant value is expected. For example, array lengths must be known at compile time, so they must be constant values:

    let arr = [10; 10]; // Ok!
    const ARR_LEN: usize = 10;
    let arr = [10; ARR_LEN]; // Ok!

    fn add_one(n: usize) -> usize { n + 1 }
    // let arr = [10; add_one(9)]; // Compile error! Expected constant value

    // If we instead mark the function as a const fn:
    const fn const_add_one(n: usize) -> usize { n + 1 }
    let arr = [10; const_add_one(9)]; // Ok!
        
    // To force an expression to be evaluated at compile-time use a const {} expression to make a constant scope. For example, let's say we want a function that only operates on types that are 4 bytes in size. We could use the assert! macro to panic at runtime if a non-4-byte type is used:

    // This function requires the generic type T to be 4 bytes in size
    // Using this function with a non-4-byte T will panic at runtime
    fn example_fn<T>(n: T) {
        assert!(std::mem::size_of::<T>() == 4);
        todo!()
    }

    // But we don't want runtime panics! Instead, force the assertion to the computed at compile time by placing it inside a const scope:

    // This function requires the generic type T to be 4 bytes in size
    // If T is not 4 bytes this will cause a compile error.
    fn example_fn2<T>(n: T) {
        const { assert!(std::mem::size_of::<T>() == 4) }
        todo!()
    }

    // Of course, not all things can be computed in a const context. Notable examples include:
    // - Non-terminating loops (or loops that can't be proven to terminate by the compiler),
    // - System I/O (printing, reading from stdin, web requests, etc.), 
    // - Dynamic type checks (e.g. dyn Trait), 
    // - Heap allocations, 
    // - Mutating static variables, 
    // etc.
}
