#![allow(dead_code, unused_variables, unused_assignments, clippy::needless_return)]

// Rust contains a number of 'container' or 'smart pointer' objects, meant to augment its contents with some extra 
// features, such as thread-safety, exclusive access, or shared ownership. Containers can, and often are, combined to
// fulfil all the requirements necessitated for common use cases.

use std::cell::{Cell, RefCell};

fn main() {
    boxx();
    ref_count();
    cell();
    ref_cell_mutex();
}

fn boxx() {
    // Box
    // When you want to allocate something on the heap instead of the stack, wrap it in a Box.
    // Box is equivalent to C++'s unique_ptr

    let int = 10_i32; // This is on the stack
    let boxed_int = Box::new(10_i32); // This is on the heap

    // A Box may be necessary for interacting with 'Dynamically Sized Types' (e.g. str, slices, trait objects). 
    // Most of the time DSTs are passed around by reference or mutable reference, but if you need to 
    // actually own a DST then you can wrap it in a Box.
}

fn ref_count() {
    // While avoiding garbage collection was one of the reasons Rust's borrowing rules were invented, runtime
    // reference counting is sometimes the easiest way to solve a problem. Rc is a reference counter - 
    // if you need multiple variables to 'own' some data then Rc can be used to provide an owned handle to your data.
    // Of course, because there are multiple 'owners' the data is immutable. When the last Rc goes out of scope 
    // the data is cleaned up.

    // Arc is an atomic reference counter, usable for multi-threaded applications. 
    // This is useful if you have multiple threads that all need access to some (read-only) data. 
    // The atomicity adds a little extra runtime overhead, so Rc is preferred if you don't need thread safety.

    // Arc is equivalent to C++'s shared_ptr
}

fn cell() {
    // Cell offers a partial escape hatch to the borrowing rules. It does not check borrowing rules at all, 
    // but is only usable with types that implement Copy. It works by ensuring that you can't take references to 
    // their internals at all (only copying out the value, or replacing the whole value), hence the compiler don't
    // have to worry about multiple mutable references existing at once, or a mutable and immutable reference 
    // to the underlying data existing at the same time, etc..

    // Cell has what is known as *interior mutability*, allowing modification even through an immutable reference.

    let cell = Cell::new(0);
    
    // Two immutable references, this is standard so far...
    let a = &cell;
    let b = &cell;

    // These immutable references let you get *a copy* of the underlying data.
    let cell_copy_1 = a.get();
    let cell_copy_2 = b.get();

    // But each of these immutable references can actually change the underlying data!
    a.replace(10);
    b.replace(20);

    // Because Cell doesn't let you take references to the inner data, it can freely be modified from multiple places without issue.
}

fn ref_cell_mutex() {
    // Like Cell, RefCell also acts as a partial escape hatch for the borrowing rules. 
    // RefCell does not require the internal data to be Copy like Cell does, but in exchange the borrowing rules must be checked at runtime, 
    // panicking if broken. This may make it easier to write particularly complex borrowing situations, especially if the user has some extra
    // information the compiler may not.

    let ref_cell = RefCell::new(String::new());
    
    // Two mutable references at once! This compiles, but will panic at runtime.
    let mut_borrow_1 = ref_cell.borrow_mut();
    let mut_borrow_2 = ref_cell.borrow_mut();

    // Mutex is similar to RefCell but is designed for a multithreaded environment - where RefCell would panic on a mutable reference already 
    // existing, a Mutex offers methods to poll or block until unique mutable access to the underlying data is obtained.
    
    // RefCell and Mutex have what is known as *interior mutability* - their internals can be modified even through an immutable reference.
}

// More container types:
// https://github.com/usagi/rust-memory-container-cs
