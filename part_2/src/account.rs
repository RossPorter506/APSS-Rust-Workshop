use std::marker::PhantomData;

use bigdecimal::BigDecimal;

use crate::transaction::Transaction;

pub struct Frozen;
pub struct Active;

pub struct BankAccount<S> {
    balance: BigDecimal,
    _state: PhantomData<S>,
}

// Methods for active bank accounts only
impl BankAccount<Active> {
    pub fn apply<E>(&mut self, transaction: impl Transaction) -> Result<(), E> {
        todo!()
    }
}

// Methods implemented for all possible states
impl<S> BankAccount<S> {
    
}

// Methods for frozen bank accounts only
impl BankAccount<Frozen> {

}
