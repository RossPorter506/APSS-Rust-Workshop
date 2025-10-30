#![allow(unused_variables, unused_assignments, dead_code, clippy::needless_return)]

fn main() {
    //if_statements();
    //inline_ifs();
}


fn if_statements() {
    // Rust supports typical if statements. Brackets are not required around the condition:
    let x = true;
    let y = 40;
    if x {
        println!("x is true");
    }
    else if y > 10 {
        println!("y is > 10");
    } 
    else {
        println!("something else");
    }
}


fn inline_ifs() -> i32 {
    // In place of the rather cryptic ternary statement from C, Rust supports full inline if statements:
    let y = 40;
    let x = if y > 20 { 0 } else { 10 };
    println!("x is {x}");

    example_function(if x > 2 {55} else {0});
    
    return if y > 50 {0} else {10}
}


fn example_function(a: i32) {
    println!("{a}");
}