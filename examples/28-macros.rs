#![allow(dead_code, unused_variables, unused_assignments, clippy::needless_return)]

fn main() {
    simple_macro();
    multi_body_macro();
    variadic_macro();
}

// Rust macros are similar to the C preprocessor (with the added bonus of type checking), 
// it can be used to generate code snippets at compile time. 
// The syntax for macros is pretty atrocious. Here is an example macro that takes no arguments:
macro_rules! say_hello {
//  vv - Parameters defined here. We take no parameters.
	() => {
    	println!("Hello!");
	};
}

fn simple_macro() {
    // To use it:
    say_hello!(); // Prints "Hello!"
}

// Of course, the real power of macros comes from being able to include arguments. 
// For example, let's say we have some trait we want to implement on several types:
trait BackwardsAdd {
	type T;
	fn backwards_add(a: Self::T, b: Self::T) -> Self::T;
}
// And an example of how we might implement this manually...
impl BackwardsAdd for u64 {
	type T = u64;
	fn backwards_add(a: Self::T, b: Self::T) -> Self::T {
	    b+a
    }
}
// Note: In this particular case we could have used a default trait implementation, but we used a macro here just as an example.


// In this case the implementation code is going to be almost identical whether it's 
// being implemented on u8, u16, i32, etc., and it's quite a few lines for implementation, 
// so let's write a macro to help us. We take one parameter, which is the type we want to implement the trait on:
macro_rules! impl_backwards_add_for {
	($type: ty) => {
        impl BackwardsAdd for $type {
            type T = $type;
            fn backwards_add(a: Self::T, b: Self::T) -> Self::T {
                b+a
            }
        }
	};
}
impl_backwards_add_for!(u8);
impl_backwards_add_for!(u16);
impl_backwards_add_for!(i32);
// etc.


// In this case we took a type ('ty'), but macros can also take expressions like 'a+b' using 'expr', 
// statements like 'if' statements or 'let' statements using 'stmt', identifiers like the name of a variable using 
// 'ident', literals like 10 or "Hello" using 'literal', etc.

// Macros can have different bodies depending on the number and type of arguments passed:
macro_rules! say_hello {
	// If we are called with no parameters then replace with this
	() => { 
    	println!("Hello!");
	};
	// If we are passed the name of a variable print that instead
    ($variable: ident) => { 
    	println!("Variable: {}", $variable);
	};
}

fn multi_body_macro() {
    say_hello!(); // Prints "Hello!"
    let i = 10;
    say_hello!(i); // Prints "Variable: 10"
}

// Macros are variadic, which means they can potentially take an arbitrary number of arguments.
macro_rules! print_all {
    ($($arg:expr),* $(,)?) => {
        $(
            println!("{:?}", $arg);
        )*
    };
}
fn variadic_macro() {
    print_all!("Hi");
    println!();

    print_all!("Hi", 2);
    println!();

    print_all!("Hi", 2, [1,2,3]);
}

// Macros are a very advanced topic and you can easily get lost in them, so we will leave them here 
// for now. For more info see the Rust book chapter on macros