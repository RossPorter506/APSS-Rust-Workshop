#![allow(unused_variables, unused_assignments, dead_code, clippy::bool_comparison)]


fn main() {
    // integers();
    // floats();
    // bools();
    // casting();
}

fn integers() {
    /* Integers */

    // Rust supports all the usual integer types:
    /*
    | Rust type  | Description                  | C equivalent  | Range                                                                                                      |
    |------------|------------------------------|---------------|------------------------------------------------------------------------------------------------------------|
    | u8         | 8-bit   unsigned integer     | uint8_t       | 0 ~ 255                                                                                                    |
    | u16        | 16-bit  unsigned integer     | uint16_t      | 0 ~ 65,535                                                                                                 |
    | u32        | 32-bit  unsigned integer     | uint32_t      | 0 ~ 4,294,967,295                                                                                          |
    | u64        | 64-bit  unsigned integer     | uint64_t      | 0 ~ 18,446,744,073,709,551,615                                                                             |
    | u128       | 128-bit unsigned integer     | uint128_t     | 0 ~ 340,282,366,920,938,463,463,374,607,431,768,211,455                                                    |
    | usize      | Platform-sized unsigned int  | unsigned int  | Platform-specific                                                                                          |
    | i8         | 8-bit   signed integer       | int8_t        | -128 ~ 127                                                                                                 |
    | i16        | 16-bit  signed integer       | int16_t       | -32,768 ~ 32,767                                                                                           |
    | i32        | 32-bit  signed integer       | int32_t       | -2,147,483,648 ~ 2,147,483,647                                                                             |
    | i64        | 64-bit  signed integer       | int64_t       | -9,223,372,036,854,775,808 ~ 9,223,372,036,854,775,807                                                     |
    | i128       | 128-bit signed integer       | int128_t      | -170,141,183,460,469,231,731,687,303,715,884,105,728 ~ 170,141,183,460,469,231,731,687,303,715,884,105,727 |
    | isize      | Platform-sized signed int    | int           | Platform-specific                                                                                          |
    |------------|------------------------------|---------------|------------------------------------------------------------------------------------------------------------|
    */

    // The default integer type is a signed 32-bit integer.

    // Underscores are ignored, so we can use them as thousands separators, etc.
    let x = 10_000;

    // (Alternatively, integer literals can be typed directly by appending the type name. This is the C equivalent of doing something like '10UL'.)
    let x = 10_u64;

    // Integers are base 10 by default, but numbers can be expressed in different bases such as binary, octal, hexadecimal:
    let bin = 0b101010100;  // 340
    let oct = 0o1234567;	// 342391
    let hex = 0xABCD; 	    // 43981

    /*
    | Operation          | Operator  |
    |--------------------|-----------|
    | Addition           | +         |
    | Subtraction        | -         |
    | Multiplication     | *         |
    | Division           | /         |
    | Modulo / Remainder | %         |
    | Bitwise AND        | &         |
    | Bitwise OR         | |         |
    | Bitwise XOR        | ^         |
    | Bitwise NOT        | !         |
    | Left shift         | <<        |
    | Right shift        | >>        |
    */
    // A notable difference from C is that in Rust the 'bitwise NOT' operation is done using the ! operator.
}

fn floats() {
    /* Floating Point Types */
    // Rust supports both single-precision floats as the f32 type and double-precision as f64. The default float type is f64.

    // Floats may also contain  '_'s as grouping separators. They don't affect the value at all.

    // Float literals must contain a decimal point, but any fractional digits are optional:
    let float = 1.0; // float inferred as an f64
    let float32: f32 = 2.; // Fractional digits not required

    // Float literals can have their type specified by suffixing them with the name of the type. The type must follow a digit:
    let f  = 1234567890.0f32; // x will be inferred as f32
    // let f2 = 1234567890.f32; // compile error!
}

fn bools() {
    /* Booleans */
    // Rust supports all the standard boolean expressions using the bool type. 

    let a = true; // type inferred as bool
    let b: bool = true; 

    let not_b       = !b;
    let a_and_b     = a & b;	// Short circuiting version: a && b
    let a_or_b      = a | b; 	// Short circuiting version: a || b
    let a_xor_b     = a ^ b; 	// Alternatively a != b
    let a_equals_b  = a == b;
    let a_greater_b = a > b;
    // etc.

    // We also have the short-circuiting OR and short-circuiting AND from C, available as || and &&. These are equivalent to 
    // | and & respectively, except the second value is only evaluated as necessary. 
    // That is, if a || expensive_fn() then expensive_fn() is only evaluated if a is false. As a result,
    // these should generally be preferred over their single counterparts.

    // It should be noted that the behaviour of ! depends on the type: For booleans it is logical negation, but for integer 
    // types it is a bitwise negation. Because Rust does not perform implicit type coercion (discussed next) you do not need to
    // worry about this distinction! A boolean has no concept of bitwise negation, and integers do not have a concept of logical negation.
}

fn casting() {
    /* Type casting */

    // In Rust all type casts are explicit. Unlike C, Rust will not attempt to coerce any data types to make sense. 
    // At first I found this annoying until I realised how it made every type cast visible, and as a result, clearly obvious where 
    // data loss could occur - each one became a careful case to consider, rather than something the compiler does behind the scenes. 

    // For example in C the following compiles just fine, and it doesn't overflow on a 32-bit system, but
    // will silently overflow on a 16-bit system, despite using explicitly sized types everywhere:
    // uint16_t a = XXXX;
    // uint16_t b = YYYY;
    // uint32_t c = a*b;
    // This occurs because of C's implicit integer promotion rules, which automatically promote everything up to int, 
    // which is 32-bit on a desktop, but on a 16-bit system a and b remain 16-bit, which then potentially overflow during multiplication, 
    // then the result is uselessly extended to 32 bits and stored into c after the overflow.

    // With that cautionary tale out of the way, in Rust there are two main ways to cast, using the 'as' keyword, and with into() and from().


    // Primitive Casting
    // The as keyword can cast primitive types into other types, where the conversion is well defined:
    let a: usize = 10;
    let b: u32 = 5;
 // let c = a + b; // Compile error! Cannot add usize and u32
    let c = a + b as usize; // Ok! converts b to usize before addition


    // Casting using Into and From
    // More complex types can't use the 'as' keyword, but they can use  into() and from() methods. 
    // Most types in Rust have .into() and .from() methods that can be used to convert between types, if they implement `From` (more on this later). 
    // The into() method is defined on the type you have, whereas the from() method is defined on the type you want. 
    // This makes into() easier to reach for, but also potentially ambiguous, in which case you may want to use from():

    let x: i32 = 10;
    let y: i64 = x.into(); 	// Type conversion inferred by compiler
    let y = i64::from(x);   // Type supplied by user
}

