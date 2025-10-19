#![allow(unreachable_code)]

fn main() {
    // One of Rust's founding principles is safety, so given the choice between halting program execution on an unrecoverable error or 
    // silently attempting to continue, Rust will choose the former every time.
    // This is called a panic. Panics usually occur from doing things like indexing an array out of bounds, etc. You can invoke your own panic through the panic macro:
    panic!("Sorry, not feeling it today"); 

    // Panics aren't like exceptions from C++, they aren't designed to be caught or handled (though this functionality does exist...). We will discuss recoverable errors later.
    // They are designed for unrecoverable conditions that the program can't work around, and will cause the program to immediately terminate (after cleaning up it's own memory).

    // All panics do the same thing, but there are also a few wrappers to help broadcast a particular intent to the (unfortunate) recipient:
    // Temporarily help your code pass compile-time checks, designed to be replaced later.
    todo!();

    // This particular feature isn't implemented and is unlikely to be implemented later.
    unimplemented!();

    // The author *believes* this particular code path isn't reachable. 
    // This is purely for the benefit of the reader for documentation purposes. The compiler won't optimise away paths that lead to unreachable!().
    unreachable!();
    // There *are* ways to force the compiler to remove impossible control flow paths, but their usage is highly discouraged in normal code. 
    // Panicking on a failed assumption is far safer and the overhead is negligible in all but the hottest code paths. 
}

