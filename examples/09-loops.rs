#![allow(unused_variables, unused_assignments, dead_code, clippy::needless_return, clippy::useless_vec, clippy::no_effect, unused_labels)]


fn main() {
    //forever();
    //for_loop();
    //ownership_and_iterators();
    //while_loop();
    //breaking();
    //nested_break();
}


fn forever() {
    // The simplest type of loop: A forever loop loops… well… forever:
    loop {
        println!("Crazy? I was crazy once.");
        println!("They locked me in a room. A rubber room.");
        println!("A rubber room with rats. Rats make me crazy.");
    }
}

fn for_loop() {
    // Rust supports enhanced for loops:
    let arr = [1,2,3,4];
    for item in arr {
        println!("{item}");
    }

    // Ranges can be used to emulate standard for loops
    for n in 0..10 { // Prints 0 to 9
        println!("{n}");
    }
    for n in 0..=10 { // Prints 0 to 10
        println!("{n}");
    }
}

// Iterating over collections also interacts with the ownership and borrowing system
fn ownership_and_iterators() {
    let vec = vec![1,2,3,4];

    // When iterating over some collection, the items in it have the same 'ownership level' as the collection:
    // Iterating over Vec<T> produces elements of T. Iterating over &mut Vec<T> produces &mut T elements. Iterating over &Vec<T> produces &T elements.
    // Note that in the case of iterating over owned values (e.g. Vec<T>) ownership is transferred from the collection into the iterated elements.
    for item in vec { // 'item' is an i32, it has been moved out of vec.
        println!("{item}");
    }

    //vec[0]; // Compile error! Use of moved value. All the values have been moved out of the vec.

    // You can 'downgrade' the ownership level of the iterator by calling either .iter() or .iter_mut():
    // Of course you can't use these methods to 'upgrade' your ownership level, e.g. from &T to &mut T for instance. 

    let vec2 = vec![1,2,3];
    for item in vec2.iter() { // 'item' is now a &i32.
        println!("{item}");
    }
    vec2[0]; // ok!

    let mut vec3 = vec![1,2,3];
    for item in vec3.iter_mut() { // 'item' is now a &mut i32.
        println!("{item}");
    }
    vec3[0]; // ok!
}

fn while_loop() {
    let mut n = 0;
    while n < 10 {
        println!("Hello!");
        n += 1;
    }
}

fn breaking() {
    // A loop can be broken out of with the break keyword, similar to other languages. 
    for i in 0..10 {
        if i == 9 {
            break;
        }
    }
}

fn nested_break() {
    
    // Rust also supports naming loops to break out of nested loops:
    let mut x = 0;
    'outer: loop {
            'inner: for i in 0..10 {
                if x == 5 && i == 9 {
                    break 'outer;
                }
            }
        x += 1;
    }
}
