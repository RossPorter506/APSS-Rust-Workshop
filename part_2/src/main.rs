fn main() {
    println!("Hello, world!");
    println!("Once you've looked at the examples in this section, try to fill out the following project.");

    // typestated_bank_account();
}


mod account;
mod transaction;

// use transaction::{Deposit, PayFlatFee, AccrueInterest};

// fn typestated_bank_account() {
//     use account::BankAccount;

//     let mut account1 = BankAccount::default();

//     println!("Initial balance: {}", account1.balance());

//     // Place cash into account1
//     account1.apply(Deposit::cash(100)).unwrap();

//     assert!(account1.balance() == 100.into());

//     // Pay $10 fee
//     account1.apply(PayFlatFee::amount(10)).unwrap();

//     assert!(account1.balance() == 90.into());

//     // Increase account value by 10%
//     account1.apply(AccrueInterest::percentage(10)).unwrap();

//     assert!(account1.balance() == 99.into());

//     // Freeze the account. This should prevent all transactions
//     let account1 = account1.freeze();
//     assert!(account1.balance() == 99.into());
//     // account1.apply(Deposit::cash(100)); // This should fail
//     let mut account1 = account1.unfreeze();
//     assert!(account1.balance() == 99.into());

//     account1.apply(Deposit::cash(100)).unwrap();

//     assert!(account1.balance() == 199.into());
// }