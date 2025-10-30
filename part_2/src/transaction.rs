use std::convert::Infallible;

use bigdecimal::BigDecimal;

trait Transaction {
    type Err;
    fn apply(self, balance: &mut BigDecimal) -> Result<(), Self::Err>;
}

/// Deposit into an account
struct Deposit {
    amount: BigDecimal,
}

/// Pay a flat fee to the bank
struct PayFlatFee {
    amount: BigDecimal,
}

/// Pay percentage interest into the account
struct AccrueInterest {
    percentage: u8,
}

// Possible ways a transaction can fail
enum TransactionError {

}