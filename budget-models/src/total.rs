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
use crate::money::Money;

pub trait IncrementalApplication {
    fn apply_transaction(&mut self, transaction: &Transaction);
}

pub trait GetTotal {
    fn get_total(&self) -> Money;
}

///////////////////////////////////////////////////////////////////////////////
// BurnUpTotal
//  The BurnUpTotal represents an incremental tracking measure of "Burn-up" of
//  a thing. As transactions are "applied" to the thing, the total increases
//  monotonically by the amount of each transaction.
////

#[derive(Clone, Copy, Debug, Default)]
pub struct BurnUpTotal(Money);

impl GetTotal for BurnUpTotal {
    fn get_total(&self) -> Money {
        self.0
    }
}

impl IncrementalApplication for BurnUpTotal {
    fn apply_transaction(&mut self, transaction: &Transaction) {
        self.0.add(transaction.amount);
    }
}

///////////////////////////////////////////////////////////////////////////////
