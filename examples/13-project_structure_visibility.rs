#![allow(dead_code)]

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

    // However this wouldn't work, because Module1Struct has private fields. 
    // We would have to rely on calling functions from module1 that return a constructed Module1Struct for us
    // let b = Module1Struct{a: 10, b:"Hello".to_string(), c: true}; // Compile error! Private fields!

	let _one: Module1Struct = module1::convert(two);
}
