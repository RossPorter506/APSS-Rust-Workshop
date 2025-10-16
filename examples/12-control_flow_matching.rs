#![allow(dead_code, clippy::while_let_on_iterator)]

fn main() {
    // if_let();
    // while_let();
    // let_else();
}

enum PaymentType {
    Cash(u32),
    CreditCard(u32, String, u8), // number, name, security code 
    DebitCard(u32, String, u16), // number, name, PIN
    Cheque(String),
}

fn if_let() {
    // You may want to do something if some data matches only one form, where 
    // an entire match statement would be excessive. 
    //In this case we can combine an if statement and a variable declaration:

    let p = PaymentType::Cash(50);
    if let PaymentType::Cash(amt) = p {
        println!("Paid by cash: {amt}");
    }  

    struct Point {
        x: i32,
        y: i32,
    }

    let pt = Point{x: 2, y: 5};
    if let Point{x, y: 5} = pt {
        println!("y is 5, x is {x}");
    }
}

fn while_let() {
    // While loops can also interact with iterable types using the ‘while let’ syntax. 
    // Any iterable type has a .next() method that provides the next element until there are none left. 
    // Provided it has some method you can call to get the next item, this continues to loop until 
    // there are no more elements left (i.e. when the iterator returns None, we will discuss Options later):

    let arr = [1,2,3];
    let mut iterable = arr.iter(); 
    while let Some(value) = iterable.next() {
        println!("{value}");
    }
}

fn let_else() {
    // Match or 'if let' statements create a new scope inside which a new variable is valid. 
    // This makes repeated calls increase in indentation. 
    // If we want to exit early in the failure case the 'let else' statement is more ergonomic.

    let PaymentType::Cash(cash_amt) = get_payment() else {
        panic!("Sorry! Cash only");
    };
    get_change(cash_amt);

    // Because the variable cash_amt doesn’t have a value in the ‘else’ case, the
    // ‘else’ case must ‘diverge’, meaning the control flow must somehow be redirected 
    // away from the rest of the function - it could return from the function, panic!(), or 
    // if the statement is inside a loop, it could continue or break, etc..
}

fn get_payment() -> PaymentType { PaymentType::Cash(50) }
fn get_change(_amt: u32) {}