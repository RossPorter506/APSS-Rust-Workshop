#![allow(dead_code, unused_variables, clippy::question_mark)]

// If you've ever written a C function, you may have used a ‘sentinel value’ to mark something as failed. 
// For example, if you call a function find_index(string) and it returns -1, you assume that the value was not found. 
// This has serious issues, as the -1 can accidentally be treated as data if the user doesn't check the result properly.

// More modern languages have tended to prefer exceptions, like C++, but this can often lead to unintended crashes at 
// runtime when someone doesn't realise a function may throw an exception. 
// Even in languages which require a function to be marked as potentially throwing an exception, like Java, the breaking
// of standard control flow can make it hard to reason about the state of a program. For example, if a program encounters 
// an error five functions deep and throws an exception that is caught three functions up suddenly the program might be in
// an invalid state, with some functions only half-applied.

// Instead in Rust errors are encoded *into the type system* using wrapper types. 
// The most common of these are Option and Result

fn main() {
    //reinventing_option();
    option();
    result();
}

// fn reinventing_option() {
//     // Some operations may fail. This is a sad fact of life. For example, say we wanted to write a 
//     // division function that checks if the divisor is zero before doing the division:
//     fn safe_divide1(a: f32, b: f32) -> f32 {
//         if b == 0.0 {
//             panic!("Zero divisor!");
//         }
//         a / b
//     }

//     // We can use the panic!() macro to halt execution, but this is not usable in practice - 
//     // halting execution for a single invalid operation is not very robust. Instead, let's just 
//     // return nothing if the divisor is zero:
//     fn safe_divide2(a: f32, b: f32) -> f32 {
//         if b == 0.0 {
//             return; // Compile error! Expected f32, found nothing!
//         }
//         a / b
//     }

//     // Okay that clearly doesn't work. We always have to return the type we said we would. So... let's 
//     // change the type we said we're returning! What if we had some kind of wrapper type:
//     fn safe_divide3(a: f32, b: f32) -> Maybef32 {
//         if b == 0.0 {
//             return Maybef32::None;
//         }
//         a / b 	// Compile error! Expected Maybef32, found f32! 
//     }

//     // Hmm. We clearly need Maybef32 to be able to hold an f32 if necessary. 
//     // Well since Rust enums can hold data we can use one of those:
//     #[derive(PartialEq)] // For use later
//     enum Maybef32 {
//         None,
//         Some(f32),
//     }

//     fn safe_divide4(a: f32, b: f32) -> Maybef32 {
//         if b == 0.0 {
//             return Maybef32::None;
//         }
//         Maybef32::Some(a / b) 
//     }

//     // This works! The type signature also tells the user that this is a fallible operation, 
//     // and they should check the output before using it. We could write some helper functions:
//     impl Maybef32 {
//         fn is_some(&self) -> bool {
//             *self != Maybef32::None
//         }
//         fn unwrap(self) -> f32 {
//             match self {
//                 Maybef32::Some(f) => f,
//                 Maybef32::None => panic!("Unwrapped a None!"),
//             }
//         }
//     }

//     // This is good, now the compiler is aware that whenever we call this function we must check if the 
//     // result is valid before we can use the data:
//     let d = safe_divide4(10.0, 0.0);
//     let a = d + 10.0; // Compile error! Mismatched types!

//     if d == Maybef32::None {
//         println!("Failed to divide, falling back to failsafe mode...")
//     }
//     else {
//         let a = d.unwrap() + 10.0;
//         /* ... */
//     }
// }

fn option() {
    // Rust's Option type is a generic version of what we just made:
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    // The Option type has many helper functions available to it. For example the map() function can be used to do 
    // an operation if the Some type is present, leaving None as None, and_then allows for fallible operations too:

    // i = log10(sqrt(VAL-2))
    let opt: Option<i32> = Option::Some(10);
    opt.map(|i| i - 2)
        .and_then(|i| i.checked_isqrt())
        .and_then(|i| i.checked_ilog(10));
    
    if let Some(i) = opt {
        println!("The value is {i}");
    }
    else {
        println!("Error during operation");
    }
}

fn result() {
    // Sometimes there can be multiple ways in which something can fail. For instance opening a file - 
    // Does the file exist? Do we have read permissions? Has someone else already opened it?

    // Similar to Option, Result is defined as:
    // enum Result<T,E> {
    //     Ok(T),
    //     Err(E),
    // }

    // Now we can include some extra type as part of our error, for example a String:

    fn some_complex_operation() -> Result<i32, String> {
        todo!()
    }

    let output = some_complex_operation();

    match output {
        Err(e)  => println!("Got an error: {e}"),
        Ok(val) => do_something_else(val),
    };

    // Result also has a bunch of helper functions associated with it such as the map() 
    // function mentioned in the Option section, as well as conversion functions for translating 
    // between Option and Result, such as ok() and err() (Option also has conversion functions).
    enum LocalFilesystemErr {
        FileNotFound,
        InvalidPermissions,
    }
    enum RemoteFilesystemErr {
        CantConnect,
        InvalidPassword,
        FileNotFound,
        InvalidPermissions,
    }

    // Since Result doesn't specify the error type, you may find yourself dealing with different error types:
    fn access_local_file(s: String)  -> Result<String, LocalFilesystemErr> { todo!() }
    fn access_remote_file(s: String) -> Result<String, RemoteFilesystemErr> { todo!() }

    fn convert_between_error_types(e: LocalFilesystemErr) -> RemoteFilesystemErr { 
        match e {
            LocalFilesystemErr::FileNotFound        => RemoteFilesystemErr::FileNotFound,
            LocalFilesystemErr::InvalidPermissions  => RemoteFilesystemErr::InvalidPermissions,
        }
    }

    let filename = "D:/remote_file_location.txt".to_string();
    let i = access_local_file(filename)
        .map_err(convert_between_error_types)
        .map(access_remote_file);
    // i is Result<i16, RemoteFilesystemErr> after calling second fn

    // The anyhow crate is an excellent addition for dealing with many error types.
}

fn result2() {
    // In the previous example we called the conversion function manually, but actually if 
    // we implement From between the two error types, Rust can do this for us automatically:
    enum LocalFilesystemErr {
        FileNotFound,
        InvalidPermissions,
    }
    enum RemoteFilesystemErr {
        CantConnect,
        InvalidPassword,
        FileNotFound,
        InvalidPermissions,
    }
    impl From<LocalFilesystemErr> for RemoteFilesystemErr {
        fn from(e: LocalFilesystemErr) -> Self {
            match e {
                LocalFilesystemErr::FileNotFound        => RemoteFilesystemErr::FileNotFound,
                LocalFilesystemErr::InvalidPermissions  => RemoteFilesystemErr::InvalidPermissions,
            }
        }
    }

    fn access_local_file(s: String)  -> Result<String, LocalFilesystemErr> { todo!() }
    fn access_remote_file(s: String) -> Result<String, RemoteFilesystemErr> { todo!() }

    // Even though these functions return different error types, Rust will automatically convert the 
    // error as needed using our above method
    let filename = "D:/remote_file_location.txt".to_string();
    let i = access_local_file(filename).map(access_remote_file);
}

fn do_something_else(i: i32) {}

fn question_mark_operator() -> Result<i32, String> {
    // A pattern that frequently occurs in Rust code is the following:
    fn try_something() -> Result<i32,String> {
        // Get the output of some function returning a Result
        let result = fallible_function();
        let val = match result {
            Ok(a) => a,
            Err(b) => return Err(b) // return error to the caller
        };
        // continue operations
        todo!()
    }

    // In this code we have some fallible function, and we want to take the inner Ok() value, or return early 
    // and pass the error up the chain to the caller, who will deal with the error.

    // We can use the let-else statement to make this a little less verbose:
    fn try_something2() -> Result<i32,String> {
        // Get the output of some function returning a Result
        let result = fallible_function();
        let Ok(a) = result else { return Err(result.unwrap_err()) };
        // continue operations
        todo!()
    }

    // But we can go further using the ? operator:
    fn try_something3() -> Result<i32,String> {
        let a = fallible_function()?;
        // continue operations
        todo!()
    }

    // The ? operator tries to get the inner Ok() or Some() value, but if it's an Err(e) or None variant 
    // then it returns Err(e) or None from the current function. As pseudocode:
    let val = fallible_function()?; // is equivalent to:

    let val = match fallible_function() {
        Ok(a) => a,
        Err(b) => return Err(b), // return error to the caller
    };

    // Because ? returns an Option or Result in the failure case (depending on whether the function 
    // it’s used on returns an Option or Result), 
    // it can only be used in functions that also return an Option or Result, respectively. 
    
    // If the function you call returns an Option but your function returns a Result (or vice versa)
    // you will have to convert the Option to a Result before using ?, such as with a function like .ok_or().

    // This function needs to return a Result
    Ok(10)
}

fn fallible_function() -> Result<i32, String> {Ok(10)}