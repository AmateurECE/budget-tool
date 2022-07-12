///////////////////////////////////////////////////////////////////////////////
// NAME:            transaction_breakdown.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Transactions can break down into other transactions. An
//                  AtomicTransaction is an atomic (indivisible) summary of one
//                  effect of a greater transaction.
//
// CREATED:         07/12/2022
//
// LAST EDITED:     07/12/2022
////

use crate::models::{Account, Transaction, TransactionType};
use crate::money::Money;

#[derive(Clone, Copy, Debug)]
pub enum AtomicTransactionDirection {
    Entering,
    Leaving,
}

#[derive(Clone, Debug)]
pub struct AtomicTransaction {
    amount: Money,
    account: String,
    direction: AtomicTransactionDirection,
}

#[derive(Clone, Debug)]
pub enum TransactionBreakdown {
    Single(AtomicTransaction),
    Double(AtomicTransaction, AtomicTransaction),
}

pub trait Breakdown {
    fn break_down(self) -> TransactionBreakdown;
}

impl Breakdown for Transaction {
    fn break_down(self) -> TransactionBreakdown {
        match self.transaction_type {
            TransactionType::Income => {
                todo!()
            },

            TransactionType::Expense => {
                todo!()
            },

            TransactionType::Transfer => {
                todo!()
            },

            TransactionType::Payment => {
                todo!()
            },
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
