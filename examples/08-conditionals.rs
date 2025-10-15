#![allow(unused_variables, unused_assignments, dead_code, clippy::needless_return)]

fn main() {
    //if_statements();
    //inline_ifs();
}


fn if_statements() -> i32 {
    // Rust supports typical if statements. Brackets are not required around the condition:
    let x = true;
    let y = 40;
    if x {
        return 10;
    }
    else if y > 10 {
        return 20;
    } 
    else {
        return 0;
    }
}


fn inline_ifs() -> i32 {
    // In place of the rather cryptic ternary statement from C, Rust supports full inline if statements:
    let y = 40;
    let x = if y > 20 { 0 } else { 10 };
    example_function(30, if x > 2 {55} else {0});
    return if y > 50 {0} else {10}
}


fn example_function(a: i32, b: i32) {}