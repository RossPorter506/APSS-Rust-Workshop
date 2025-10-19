#![allow(dead_code, unused_variables, unused_assignments, clippy::needless_return)]

// Rust contains a number of 'container' or 'smart pointer' objects, meant to augment its contents with some extra 
// features, such as thread-safety, exclusive access, or shared ownership. Containers can, and often are, combined to
// fulfil all the requirements necessitated for common use cases.

use std::{cell::{Cell, RefCell}, rc::Rc};

fn main() {
    boxx();
    ref_count();
    cell();
    ref_cell_mutex();
}

fn boxx() {
    // Box
    // When you want to allocate something on the heap instead of the stack, wrap it in a Box.
    // This is effectively places the element on the heap and gives you a pointer to it.
    // Regardless of the size of T, the size of Box<T> is always the size of a pointer.
    // Box is equivalent to C++'s unique_ptr

    let int = 10_i32; // This is on the stack
    let boxed_int = Box::new(10_i32); // This is on the heap

    // A Box may be necessary for interacting with 'Dynamically Sized Types' (e.g. str, slices, trait objects). 
    // Most of the time DSTs are passed around by reference or mutable reference, but if you need to 
    // actually own a DST then you can wrap it in a Box.

    // Box is often used with recursive data types, like a linked list:
    // Because local variables are placed on the stack by default, their size must be known at compile time.
    // Recursive data types however appear to have infinite size: The size of List<T> is the size of T plus the size of List<T>!
    // enum List<T> { // Compile error! Infinite size element
    //     Element(T, List<T>),
    //     End,
    // }
    // let linked_list = List::Element(10, List::Element(20, List::End));

    // If we replace List<T> with Box<List<T>> the size of List<T> goes from being seemingly infinite to the size of T plus the size of a pointer.
    // enum List<T> {
    //     Element(T, Box<List<T>>),
    //     End,
    // }
    // let a = List::Element(10, Box::new(List::Element(20, Box::new(List::End))));
}

fn ref_count() {
    // Rc is a reference counter - useful if you need to 'share' ownership of something.
    // Because there are multiple 'owners' the data is immutable. 
    // When the last owner goes out of scope the data is cleaned up.

    // For example, suppose we have a directed cyclic graph with four nodes: 
    //    A
    //  /   \    |
    // B     C   |
    //  \   /    V
    //    D

    // B and C need to somehow share ownership of D:

    // Note: We don't implement Copy or Clone
    struct Node {
        name: String,
        children: Vec<Rc<Node>>
    }
    impl Node {
        fn new(name: &str, children: Vec<Rc<Self>>) -> Node {
            Node {name: name.to_string(), children}
        }
    }

    let d = Node::new("D", vec![]);
    let d_refcount: Rc<Node> = Rc::new(d); // Note that since our Node doesn't implement Copy, passing 'd' to new() consumes it.
    //println!("{}", d.name); // Compile error! Use of moved value!
    
    let c = Node::new("C", vec![d_refcount.clone()]);
    let b = Node::new("B", vec![d_refcount]);
    
    let a = Node::new("A", vec![Rc::new(b), Rc::new(c)]);
    println!("The first node two levels deep is {}", a.children[0].children[0].name);

    // Arc is an atomic reference counter, usable for multi-threaded applications. 
    // This is useful if you have multiple threads that all need access to some (read-only) data. 
    // The atomicity adds a little extra runtime overhead, so Rc is preferred if you don't need thread safety.

    // Arc is equivalent to C++'s shared_ptr
}

fn ref_cell_mutex() {
    // The compiler usually checks the borrowing rules at compile time, but RefCell allows the checks to be done at runtime instead. 
    // This is useful if you *know* that you aren't violating the rules using some information the compiler doesn't have.
    // If the borrowing rules are violated (e.g. two mutable references at once), then RefCell will panic. 
    let ref_cell = RefCell::new(String::new());
    
    // Two mutable references at once! This compiles, but will panic at runtime.
    let mut_borrow_1 = ref_cell.borrow_mut();
    let mut_borrow_2 = ref_cell.borrow_mut();

    // Mutex is similar to RefCell but is designed for a multithreaded environment - where RefCell would panic on a mutable reference already 
    // existing, a Mutex offers methods to poll or block until unique mutable access to the underlying data is obtained.

    // A common use for RefCell is in embedded systems where we might want to have mutable access to some object in the main program and also in an interrupt handler.
    // Provided the object is properly synchronised (disabling interrupts while it's being used in the main program), we know that multiple mutable references to the
    // object will never co-exist, but the compiler can't prove this! This situation is exactly what RefCell is for. 

    // RefCell and Mutex have what is known as *interior mutability* - their internals can be modified even through an immutable reference.
}


fn cell() {
    // Cell is a simpler form of RefCell. It prevents taking references to the inner data, only allowing copying values in or out. The inner type must therefore implement Copy.
    // Because it prevents all borrowing it doesn't need to check any borrow checking rules at runtime.
    // Cell is useful for allowing mutability with only a shared reference.

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

    // Note that Cell doesn't solve synchronisation issues between threads, so isn't safe for multithreaded accessing.
    // Multithreaded safety is defined by the two traits Send and Sync. Because multithreaded programming is a whole other topic, we 
    // won't discuss it here.
}

// More container types:
// https://github.com/usagi/rust-memory-container-cs
