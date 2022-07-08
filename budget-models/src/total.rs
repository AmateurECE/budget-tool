///////////////////////////////////////////////////////////////////////////////
// NAME:            total.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic for calculating particular kinds of totals.
//
// CREATED:         07/07/2022
//
// LAST EDITED:     07/08/2022
////

use crate::models::Transaction;
use crate::models::TransactionType::*;

pub trait IncrementalApplication {
    fn apply_transaction(&mut self, transaction: &Transaction);
}

///////////////////////////////////////////////////////////////////////////////
// BurnUpTotal
//  The BurnUpTotal represents an incremental tracking measure of "Burn-up" of
//  a thing. As transactions are "applied" to the thing, the total increases
//  monotonically by the amount of each transaction.
////

pub struct BurnUpTotal(i64);

impl BurnUpTotal {
    pub fn new() -> Self {
        Self(0 as i64)
    }

    pub fn get_total(&self) -> i64 {
        self.0
    }
}

impl IncrementalApplication for BurnUpTotal {
    fn apply_transaction(&mut self, transaction: &Transaction) {
        if Income == transaction.transaction_type {
            self.0 += transaction.amount;
        } else {
            self.0 += -1 * transaction.amount;
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
