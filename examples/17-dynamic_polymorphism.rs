#![allow(dead_code, unused_variables, clippy::to_string_trait_impl)]

// In cases where the compiler can't determine what type will be passed to a function 
// (or we don't want to bloat the executable from monomorphization), we must fall back on 
// dynamic polymorphism. Dynamic polymorphism is what you probably think about when you think 
// of an object-oriented language: When one object is treated like another by casting to a different type. 

// In Rust this happens by providing a pointer to an object and the object's 'vtable' (list of function pointers). 
// The receiver has no idea about the underlying type besides the functions it can call on it 
// (i.e. those defined in the trait). This pointer to object and vtable is called a 'trait object'. 
// We use the `dyn` keyword to talk about a trait object. Because a trait object is a reference to something else 
// you can never own a trait object (e.g. have a `dyn T`), you can only borrow an existing object (e.g. a `&dyn T`, or `&mut dyn T`)

struct Point {
    x: i32,
    y: i32,
}
impl ToString for Point {
	fn to_string(&self) -> String {
	    format!("({}, {})", self.x, self.y)
    }
}

fn main() {
    fn dynamic_print(thing: &dyn ToString) {
        println!("{}", thing.to_string());
    }
    dynamic_print(&Point{x: 5, y:2});
    dynamic_print(&10_i32);

    // Dynamic polymorphism is slower than static polymorphism because it requires following a pointer 
    // to interact with an object, but on the other hand does not require the compiler to duplicate function 
    // implementations, nor does the compiler need to be able to determine the types being passed into the function at compile-time.
}