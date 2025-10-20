#![allow(dead_code, unused_variables)]

// Multi-file projects are organised like so:

// Tell the compiler that these two other files are part of this project
// This is only required once inside the root of the project (i.e. the file with the main fn)
mod module1; 
mod module2;

// Use things in the other files. This must be done in each file that wants to use things from other files.
use module1::Module1Struct; 
use module2::Module2Struct;

fn main() {
    // We can construct Module2Struct because the fields aren't private
	let two = Module2Struct{a:  10, b: String::from("Hello")};

    // However the following wouldn't work because Module1Struct has private fields. 
    // let b = Module1Struct{a: 10, b:"Hello".to_string(), c: true}; // Compile error! Private fields!

    // Instead we can call the public constructor defined in module1.
    let one = Module1Struct::new(10, "".to_string(), true);

    // Of course, if something returns a Module1Struct we can use it, even if the struct is private.
	let also_one: Module1Struct = crate::module1::convert(two);
    // Because we didn't add a 'use' statement for the convert(), we have to fully specify the path.
}
