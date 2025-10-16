#![allow(dead_code, unused_variables)]



fn main() {
    // Regular generics are generic over types. A const generic has a fixed type, but varies over values. 
    // For example, suppose we want to write a function that returns a particular size of array:

    // fn return_arr(val: i32, length: usize) -> [i32; length] {
    // 	   [val; length] // array size must be a constant ^^^ 
    // }//        ^^^^^ array size must be a constant

    // This doesn't work! Arrays are stored on the stack, as their size is known at compile time, but 
    // here the array size isn't constant. We could write several copies of the function:
    fn return_arr1(val: i32) -> [i32; 1] {
        [val; 1]
    }
    fn return_arr2(val: i32, length: usize) -> [i32; 2] {
        [val; 2]
    }
    // This works, but is obviously not very ergonomic. 

    // Instead, we can use a const generic:
    fn return_arr<const N: usize>(val: i32) -> [i32; N] {
        [val; N]
    }
    // When we call the function we have to specify the const generic value
    let arr = return_arr::<10>(0);
    // ...unless the compiler can infer it from context.
    let arr: [i32; 10] = return_arr(0);
}
