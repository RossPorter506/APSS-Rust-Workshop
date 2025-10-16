#![allow(dead_code, unused_variables, clippy::needless_return)]

fn main() {
    anonymous_functions();
    closures();
}

fn anonymous_functions() {
    // Rust supports anonymous (or 'lambda') functions, and also a more powerful variant called a closure.
    // Anonymous functions are particularly useful when a function wants to remain generic over its actions, and so 
    // takes another function as a parameter. 
    // For example, you may have some functions that check if some piece of data fulfils some requirement like:

    fn check_if_even(n: i32) -> bool {
        n % 2 == 0
    }

    fn check_if_bigger_than_10(n: i32) -> bool {
        n > 10
    }

    fn check_if_power_of_two(n: i32) -> bool {
        n.count_ones() == 1
    }

    // Instead of writing all of these different functions, we might write a single function that accepts a 
    // number and a function as a parameter. 
    // The outer function then calls the inner function with the number as a parameter, and returns the result
    fn check(n:i32, f: fn(i32)->bool) -> bool {
        f(n)
    }
    fn is_even(n: i32) -> bool {
        n%2 == 0
    }
    check(10, is_even);

    // However you can see we still need to fully define the functions we pass into check, even if they're very simple one-liners.
    // Rust allows you to write anonymous functions (so called because they do not need to be named):
    check(10, |n| n % 2 == 0);

    // The anonymous function begins with two pipe characters || with the names of parameters in between them. 
    // Note that because the compiler knows the function signature already it can usually infer the types of all the parameters. 
    // If the function is one expression it can be written immediately after the pipes. If the function is multiple expressions 
    // then the body should be encased in squiggly brackets:
    check(10, |n| {let log2n = n.ilog2(); return log2n > 3});

}

fn closures() {
    // Closures are anonymous functions that also use (or 'capture') variables from their environment, 
    // not just those passed in as parameters:

    let val: usize = 10;
    let closure = |n: usize| n == val;

    // Closures interact with the ownership and borrowing system. By default a closure will try and take the 
    // lowest level of ownership possible:
    // An immutable borrow, then a mutable borrow if that's insufficient, and only taking ownership if necessary.

    // If you want a closure to take ownership of a value it otherwise wouldn't, you can use the 'move' keyword:
    // A non-Copy type to demonstrate moves. We will discuss Box in detail later.
    let val: Box<i32> = Box::new(10); 

    // Normally this closure only needs an immutable reference to val
    // But the move keyword forces the closure take ownership of val
    let closure = move |n: i32| n == *val;

    if *val == 10 { // Compile error! Use of moved value
        println!("Still here!");
    }

    // Because closures capture information from their environment, their type signatures are actually unique and can't be expressed normally.
    // Luckily, Rust will implement traits on closures so we can still reference them:

    // FnOnce: Implemented on all closures that can be called at least once (i.e. all closures). 
    // This sounds silly, but if a closure moves a captured value (either by consuming it or by returning it) then 
    // trying to call the closure more than once would result in a 'use of moved value' compile error.

    // FnMut: Implemented for closures do not move any captured values, but may mutate them. 
    // Because they don't move any captured values they can be called an unlimited number of times. 
    // Closures that implement FnMut also implement FnOnce.

    // Fn: Implemented for closures that don't move captured values, nor mutate them. 
    // This is the most restrictive trait. Because they don't move any captured values they can be called an unlimited number of times. 
    // Closures that implement Fn also implement FnMut and FnOnce.
}
