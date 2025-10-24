fn main() {
    // Printing in Rust actually uses one of the more advanced Rust features, called macros.
    // We won't worry about the details for now, but macros are invoked almost exactly like a function, but 
    // with a ! after the name.
    println!("Hello!");

    // Rust supports format strings by adding curly brackets (called the 'placeholder'), then specifying 
    // the value to go within after the string:
    println!("Value: {}", 50 / 2 + 10);

    // If the expression is a variable name they can be directly placed inside the string:
    let var = 50 / 2 + 10;
    println!("Value: {var}");

    // Most primitive types can be printed automatically (because they implement `Display`, more on this 
    // in the 'common traits' example). Most non-primitive types usually *can't* be printed in the standard manner. 
    // This lets you define this yourself, should you need or want to. However, almost any type can be printed using
    // the special 'Debug' display mode by placing ':?' inside the format string placeholder:
    let array = [1,2,3,4];
    println!("arr: {:?}", array);
}

