#![allow(dead_code, unused_variables, unused_assignments, clippy::needless_return)]


fn main() {
    lifetimes();
    the_static_lifetime()
}

fn lifetimes() {
    // The lifetime of a variable is the amount of time that it is in scope for. 
    // The lifetime of a variable can influence whether a particular borrow is valid, for example:

    let borrow: &i32; // Let’s store a reference to something
    {
        let y = 10;
        borrow = &y; 
    }  // <-- 'y' dropped here while still borrowed
    println!("{borrow}"); // Compile error! 'y' does not live long enough. Comment out this line and watch the error disappear.

    // In the above example we can see that we try to store a reference to y in ref. 
    // But because y goes out of scope before ref does, there is a period of time where ref would refer to deallocated memory. 
    // This would break the memory safety guarantee of Rust, so it becomes a compile error. 
    // We say that the lifetime of 'borrow' outlives the lifetime of 'y'. 

    // Lifetimes are only ever an issue when taking references to other pieces of data, so most of the time the compiler 
    // is able to automatically infer and hide ('elide') lifetimes. 
    // To explicitly name a lifetime we use the following syntax:
    fn example1<'a>(in1: i32, in2: &'a i32) -> &'a i32 {
        todo!()
    }

    // Let's consider a case where the compiler can automatically infer the lifetimes for us: 

    fn example2(in1: i32, in2: &i32) -> &i32 {
        todo!()
    }

    // In this case the compiler sees that the output is a reference to some piece of data, so 
    // it must ensure this new reference does not outlive the data it's referring to (to prevent the above case). 
    // The only way to return a reference is to take a reference against one of the function's inputs, so the output 
    // lifetime must be the same as one of the input's lifetimes. Since example() takes ownership of in1 it will be 
    // dropped at the end of the function call, so trying to return a reference to in1 is nonsensical. This leaves in2. 
    // Because there is only one input that is a reference the compiler can automatically infer that the output reference lives as long as the input.

    // Lifetime errors only occur with functions when returning a reference. 
    // If the function takes only one reference as an input the lifetimes can be automatically elided for you.

    // Let's consider a case where the compiler can't automatically infer the lifetimes.
    
    fn example3(in2: &i32, in3: &i32) -> &i32 { 
        todo!()
    } // Compile error! Is the output valid for in2's lifetime, or in3's?

    // The compiler can't automatically figure out what the lifetime of the output will be. For example:
    let a: i32 = 0;
    let mut c: &i32;
    {
        let b = 1;
        c = example3(&a, &b);
    }
    println!("{c}"); // Is c still referring to allocated memory here?

    // Suppose that example2()'s implementation actually has the output dependent on the value of the first 
    // borrowed value, not the second, so that the above example should compile. In this case we can tell the compiler 
    // that by defining two lifetimes and setting the output as the lifetime of the first value:
    fn example4<'a, 'b>(in1: i32, in2: &'a i32, in3: &'b i32) -> &'a i32 {
        in2 // example implementation
    }

    // Another possible solution might be to force the two inputs to have the same lifetime:
    fn example5<'a>(in1: i32, in2: &'a i32, in3: &'a i32) -> &'a i32 {
        in2 // example implementation
    }
    // This is easier to reason about, but limits what can be passed into the function. The previous 
    // code example would produce a compile error, as the lifetime of a and b are different.

    // But what if the function is more complex, and returns a reference to either in2 or in3 based on the execution of the code?

    // fn example6<'a,'b>(first: bool, in2: &'a i32, in3: &'b i32) -> &'?' i32 {
    //     if first {                       // what lifetime goes here? ^
    //         in2
    //     } else {
    //         in3
    //     }
    // }

    // This is more difficult, as the lifetime of the output isn’t always 'a or 'b. 
    // To solve this, first note that for any two lifetimes a and b, either a and b are equal, or one completely contains the other:
    fn example7() {
        let a = 10; // <-- a begins here 
        {
            let b = 20; // <-- b begins here
        } // <-- b ends here
    } // <-- a ends here

    // Suppose, like the code example above, that 'a outlives 'b (or in other words, 'a lives at least as long as 'b). 
    // In all cases the lifetime of the output lives at least as long as 'b. To tell the compiler that 'a outlives 'b 
    // (aka 'a lives at least as long as 'b) we write 'a:'b. The colon can be read as ‘outlives’ (or more accurately ‘lives at least as long as’).

    //      vvvvv 'a lives at least as long as 'b
    fn test<'a:'b,'b>(first: bool, in2: &'a i32, in3: &'b i32) -> &'b i32 {
        if first {
            in2
        } else {
            in3
        }
    }

    let a: i32 = 0;
    let mut c: &i32;
    {
        let b = 0;
        c = test(true, &a, &b);
        println!("{c}"); // Ok! b still exists so c can be used
    }
    println!("{c}"); // Compile error! b does not live long enough

    // Because the compiler now knows that 'a lives at least as long as 'b, it can allow the output to exist for 
    // at least as long as 'b, the shortest of the input lifetimes.
}

fn the_static_lifetime() {
    // I lied a little bit when I said that if a function takes only one reference as input then the only possible lifetime 
    // of the output reference is that same lifetime:
    fn example<'a, T>(in1: T, in2: &'a T) -> &T {
        in2 // Surely the output lifetime here is always 'a right??
    }
    // Well global (static) variables exist, so a function may always return a static reference, even if it takes no inputs:

    static GLOBAL_T: i32 = 0;

    fn static_ref() -> &'static i32 {
        &GLOBAL_T
    }

    // Static references are exceedingly useful, as static references are always guaranteed to exist at any point in the program, 
    // effectively removing most lifetime issues. Of course, static lifetimes are difficult to construct. Static variables are typically 
    // immutable and must be able to be constructed at compile time. In this case that means T::new() must be a const function.

    // You might be tempted to say that any variable defined in main() has a static lifetime after it’s declared, right? Not so fast! 
    // If a program panics, then the stack will be unwound, which can cause variables in main to be deallocated. Similarly, multithreaded 
    // programs can run into similar issues, where one thread may have a reference to some other thread’s data, but if the thread with the 
    // data panics or ends then the data goes too.
}