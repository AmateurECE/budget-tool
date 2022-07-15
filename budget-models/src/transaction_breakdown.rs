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
// LAST EDITED:     07/14/2022
////

use crate::models::{Transaction, TransactionType};
use crate::money::Money;

#[derive(Clone, Copy, Debug)]
pub enum AtomicTransactionDirection {
    Entering,
    Leaving,
}

#[derive(Clone, Debug)]
pub struct AtomicTransaction {
    pub owning_id: i32,
    pub line_item: i32,
    pub amount: Money,
    pub account: String,
    pub direction: AtomicTransactionDirection,
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
                TransactionBreakdown::Single(AtomicTransaction {
                    owning_id: self.id,
                    line_item: self.line_item,
                    amount: self.amount.into(),
                    account: self.receiving_account.unwrap(),
                    direction: AtomicTransactionDirection::Entering,
                })
            },

            TransactionType::Expense => {
                TransactionBreakdown::Single(AtomicTransaction {
                    owning_id: self.id,
                    line_item: self.line_item,
                    amount: self.amount.into(),
                    account: self.sending_account.unwrap(),
                    direction: AtomicTransactionDirection::Leaving,
                })
            },

            TransactionType::Transfer => {
                TransactionBreakdown::Double(
                    AtomicTransaction {
                        owning_id: self.id,
                        line_item: self.line_item,
                        amount: self.amount.into(),
                        account: self.sending_account.unwrap(),
                        direction: AtomicTransactionDirection::Leaving,
                    },

                    AtomicTransaction {
                        owning_id: self.id,
                        line_item: self.line_item,
                        amount: self.amount.into(),
                        account: self.receiving_account.unwrap(),
                        direction: AtomicTransactionDirection::Entering,
                    }
                )
            },

            TransactionType::Payment => {
                TransactionBreakdown::Double(
                    AtomicTransaction {
                        owning_id: self.id,
                        line_item: self.line_item,
                        amount: self.amount.into(),
                        account: self.sending_account.unwrap(),
                        direction: AtomicTransactionDirection::Leaving,
                    },

                    AtomicTransaction {
                        owning_id: self.id,
                        line_item: self.line_item,
                        amount: self.amount.into(),
                        account: self.receiving_account.unwrap(),
                        direction: AtomicTransactionDirection::Entering,
                    }
                )
            },
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
