#![allow(unused_variables, unused_assignments, dead_code, clippy::needless_return, clippy::useless_vec, clippy::no_effect, unused_labels)]

fn main() {
    // struct_example();
    // simple_enum();
    // tagged_enum();
}

fn struct_example() {
    // Rust supports collating related data into a custom data type using the struct keyword:
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point {x: 10, y: 20};

    println!("{}, {}", p.x, p.y); // Access x and y fields
    
    // Reminder: If your type implements 'Debug' you can print it directly using the ':?' syntax:
    #[derive(Debug)] // This line automatically implements Debug for us. We'll talk about what this actually means later.
    struct Point2 {
        x: i32,
        y: i32,
    }
    let p2 = Point2 {x: 10, y: 30};
    println!("{:?}", p2);
}

fn simple_enum() {
    // Rust supports simple C-style enums, but also a more powerful extension thereof. Simple enums are created like so:

    enum PaymentType {
        Cash,
        CreditCard,
        DebitCard,
        Cheque,
    }

    let payment = PaymentType::Cash;

    // The easiest way to work with enums is typically using a match statement (discussed in detail next):

    let p = PaymentType::Cash;
    match p {
        PaymentType::Cash 					                => give_change(),
        PaymentType::CreditCard | PaymentType::DebitCard    => check_card_number(),
        PaymentType::Cheque 				                => verify_authenticity(),
    }

    // This is nice, but wouldn't it be cool if we could connect data to enums, like a card number or signature string?
}


fn tagged_enum() {
    // Rust supports what are known as 'tagged unions', 'data-carrying enums', or 'disjoint unions'. 
    // Whatever you call it, you can have enums with fields!

    enum PaymentType {
        Cash(u32),
        CreditCard(u32, String, u8), // number, name, security code 
        DebitCard(u32, String, u16), // number, name, PIN
        Cheque(String),
    }

    let p = PaymentType::Cash(55);

    // We can use the 'use' keyword to being all the enum variants into scope, so we don't have to prefix with 'PaymentType::'
    use PaymentType::*;

    match p {
        Cash(amt)				                        => give_change2(amt),
        CreditCard(num, _, _) | DebitCard(num, _, _)  	=> check_card_number2(num),
        Cheque(str)			                            => verify_authenticity2(str),
    }

    // Whoah! Cool!

}




fn give_change() { println!("Giving change...") }
fn check_card_number() { println!("Checking card number...") }
fn verify_authenticity() { println!("Verifying signature...") }

fn give_change2(change: u32) { println!("Giving change for {change}") }
fn check_card_number2(num: u32) { println!("Checking card number: {num}") }
fn verify_authenticity2(sig: String) { println!("Verifying signature: {sig}") }