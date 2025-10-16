#![allow(clippy::approx_constant, dead_code)]

fn main() {
    // simple_match();
    // structured_match();
}

fn simple_match() {
    // Rust also supports pattern matching through the match keyword. This allows for powerful yet succinct expressions. 
    // Matches are checked from top to bottom, executing the first matching case. 
    // A match statement must be exhaustive, matching every possible case:

    let x = 50;
    match x {
        0     		        => println!("Zero"),
        1 | 2 		        => println!("1 or 2: {x}"),
        3..=9  		        => println!("3 to 9: {x}"),
        n if n % 10 == 0 	=> println!("Multiple of 10: {n}"),
        _ 			        => println!("Other"), // default case
    };

    // Which is equivalent to the following if statement:
    if x == 0 {
        println!("Zero");
    }
    else if x == 1 || x == 2 {
        println!("1 or 2: {x}");
    }
    else if [3,4,5,6,7,8].contains(&x) {
        println!("3 to 9: {x}");
    }
    else if x % 10 == 0 {
        println!("Multiple of 10");
    }
    else {
        println!("Other");
    }
}

fn structured_match() {
    // Match expressions are particularly useful on structured data such as arrays, tuples, structs, enums, etc.:

    fn return_many_things() -> (i32, f64, bool) { (10, 25.0, true) }

    match return_many_things() {
        (_,     _,      true ) => println!("bool is true!"),
        (0..10, _,      _    ) => println!("int is between 0 and 10!"),
        (_,     3.14,   false) => println!("float is pi and bool is false!"),
        _ 			           => println!("everything else..."),
    };

    // Match statements can also be used inline, as part of assignments for example:
    let b = true;
    let _x = match b {
        true => 10,
        false => 0,
    };

    // If you have a case where you don't want to do anything, Rust requires placing the empty tuple after the big arrow:
    let i = 10;
    match i {
        0..=9   => println!("single digit"),
        10..=99 => println!("two digits"),
        _       => (), // Do nothing
    }
}
