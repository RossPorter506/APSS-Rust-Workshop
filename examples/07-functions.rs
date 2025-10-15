#![allow(unused_variables, unused_assignments, dead_code, clippy::needless_return)]

// As I'm sure you've guessed by this point, functions are declared using the 'fn' keyword followed by the name, 
// parameter list, return type (if any) then the implementation inside curly brackets:
fn example(int: i32) -> bool {
    return true;
}

// As in C, program execution begins from the start of the main function.

// The return keyword is optional. To return a value omit a semicolon. For example:
fn example2(a: i32, b: i32) -> bool {
	a*b > 10
}



// Due to the ownership and borrowing rules, functions can safely return local variables:
// Returned values have their ownership transferred to the caller:
fn some_fn() -> String {
	let y = "Hello!".to_string(); // <-- y owns the string here
	return y; // <-- y transfers ownership to the caller
}

fn outer() {
	let x = some_fn(); // <-- x takes ownership of the string
	// ...  // <-- x now owns the string so it can still be accessed here.
} // <-- x goes out of scope here, and the string is dropped


// You cannot return a reference to a local variable however, because the local variable will go out of scope at the end of the function:
fn test() -> &[i32; 10] {
    //       ^^ Compile error! The compiler knows you can't possibly return a borrowed value with nothing to borrow from!
	let x = [0; 10]; 
	return &x; // Try to return a borrow of the above array, but x would be dropped at the end of the fn!
}


// Borrowing Parameters
// Functions in Rust take ownership of all their parameters. 
// Remember that some types in Rust (those that are just as cheap to copy as they are to move) will simply be copied rather than dealing with ownership rules. 
// These cheap types are said to implement `Copy`. If a type is not `Copy`, then the following ownership and borrowing rules apply:

fn uh_oh() {
	let x = "Hello!".to_string(); // <-- x takes ownership of the string
    some_fn2(x); 	  // <-- x transfers ownership of the value to the function
    // ... // <-- the function has gone out of scope, so the array is dropped
    some_fn2(x); // Compile error! Use of moved value
} 
fn some_fn2(val: String) {
	//... // <-- val takes ownership of the array
} // <-- val goes out of scope, so the array is dropped 



// Of course we could make some_fn() return the value and then assign it back to x, but this is a bit... eugh...
fn awkward() {
	let mut x = "Hello!".to_string(); // ← x takes ownership of the array
    x = some_fn3(x); //← x transfers ownership of the value to the function
    x = some_fn3(x); // ok!
} 
fn some_fn3(val: String) -> String {
	// ... // <-- val takes ownership of the value of the array
	return val; // <-- val returns ownership of the array to the caller 
} 



// Luckily we can just let some_fn() borrow the data, rather than taking ownership of it. As a reminder, we do this using the & operator. 
// Borrowing is a compiler-enforced contract between the caller and callee:
// The caller allows access to some data, and the callee promises that the data will still be available (unmodified!) at the end of the function:
 
fn borrow() {
	let x = "Hello!".to_string(); // <-- x takes ownership of the array
    some_fn4(&x); 	// <-- some_fn() borrows the contents of x
    some_fn4(&x); 	// ok!
} 
fn some_fn4(val: &String ) { // <-- Note it now expects a borrowed value
	// ... // <-- val borrows the array
} // <-- we don't need to explicitly return the borrowed value, it's done for us



// Likewise a function can also mutably borrow a value:

fn mut_borrow() {
	let mut x = "Hello!".to_string(); //<-- x takes ownership of the array
    some_fn5(&mut x); 	// <-- some_fn() mutably borrows the contents of x
                    // <-- x has value [2,1,1,1,1,...]
    some_fn5(&mut x); 	// ok!
				// <-- x has value [4,1,1,1,1,...]
} 
fn some_fn5(val: &mut String) { // <-- Note it now expects a mutable borrow
	val.clear(); // <-- val mutates the value
} // <-- we don't need to explicitly return the borrowed value, it's done for us



fn main() {

}