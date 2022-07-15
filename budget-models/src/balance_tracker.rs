///////////////////////////////////////////////////////////////////////////////
// NAME:            balance_tracker.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Tracks account balances against transactions.
//
// CREATED:         07/14/2022
//
// LAST EDITED:     07/14/2022
////

use std::collections::HashMap;
use crate::calculation::Calculation;
use crate::money::Money;
use crate::transaction_breakdown::{
    AtomicTransaction, TransactionBreakdown::{self, *},
    AtomicTransactionDirection::*,
};

///////////////////////////////////////////////////////////////////////////////
// TrackedAccount
////

#[derive(Clone, Debug, Default)]
pub struct TrackedAccount {
    pub initial: Money,
    pub current: Money,
}

impl TrackedAccount {
    pub fn new(initial_balance: Money) -> Self {
        Self {
            initial: initial_balance,
            current: initial_balance,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// BalanceTracker
////

pub struct BalanceTracker(HashMap<String, TrackedAccount>);

impl BalanceTracker {
    fn apply_transaction(&mut self, input: &AtomicTransaction) {
        let account = self.0.get_mut(&input.account).unwrap();
        match input.direction {
            Entering => {
                account.current.add(input.amount);
            },

            Leaving => {
                account.current.subtract(input.amount);
            },
        }
    }
}

impl Calculation for BalanceTracker {
    type Input = TransactionBreakdown;
    type Result = HashMap<String, TrackedAccount>;

    fn apply(&mut self, input: &Self::Input) {
        match &input {
            Single(one) => {
                self.apply_transaction(&one);
            },

            Double(from, into) => {
                self.apply_transaction(&from);
                self.apply_transaction(&into);
            },
        }
    }

    fn calculate(&self) -> &Self::Result {
        &self.0
    }
}

///////////////////////////////////////////////////////////////////////////////
