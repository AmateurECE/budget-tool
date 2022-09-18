///////////////////////////////////////////////////////////////////////////////
// NAME:            balance_tracker.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Tracks account balances against transactions.
//
// CREATED:         07/14/2022
//
// LAST EDITED:     07/16/2022
////

use crate::calculation::Calculation;
use crate::models;
use crate::money::Money;
use crate::transaction_breakdown::{
    AtomicTransaction, AtomicTransactionDirection::*, Breakdown,
    TransactionBreakdown::*,
};
use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////
// BalanceTracker
////

// Track account balances as transactions are applied to them.
pub struct BalanceTracker(HashMap<String, Money>);

impl BalanceTracker {
    pub fn from_initial_balances<'a>(
        balances: impl Iterator<Item = &'a models::InitialBalance>,
    ) -> Self {
        Self(
            balances
                .map(|item| (item.account.clone(), item.balance.into()))
                .collect::<HashMap<String, Money>>(),
        )
    }

    fn apply_transaction(&mut self, input: &AtomicTransaction) {
        let account = self.0.get_mut(&input.account).unwrap();
        match input.direction {
            Entering => account.add(input.amount),
            Leaving => account.subtract(input.amount),
        }
    }
}

impl Calculation for BalanceTracker {
    type Input = models::Transaction;
    type Result = HashMap<String, Money>;

    fn apply(&mut self, input: Self::Input) {
        let breakdown = input.break_down();
        match &breakdown {
            Single(one) => {
                self.apply_transaction(&one);
            }

            Double(from, into) => {
                self.apply_transaction(&from);
                self.apply_transaction(&into);
            }
        }
    }

    fn calculate(&self) -> &Self::Result {
        &self.0
    }
}

///////////////////////////////////////////////////////////////////////////////
