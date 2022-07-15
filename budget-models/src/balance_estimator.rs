///////////////////////////////////////////////////////////////////////////////
// NAME:            balance_estimator.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Estimates period-end balances for accounts based on budget.
//
// CREATED:         07/14/2022
//
// LAST EDITED:     07/15/2022
////

use std::collections::HashMap;

use crate::calculation::Calculation;
use crate::models;
use crate::money::Money;
use crate::transaction_breakdown::{
    AtomicTransaction, Breakdown, TransactionBreakdown::*,
};

///////////////////////////////////////////////////////////////////////////////
// BalanceEstimator
////

pub struct BalanceEstimator(HashMap<String, Money>);

impl BalanceEstimator {
    fn apply_transaction(&mut self, input: &AtomicTransaction) {
        let balance = self.0.get_mut(&input.account).unwrap();
        balance.add(input.amount);
    }
}

impl Calculation for BalanceEstimator {
    type Input = models::BudgetItem;
    type Result = HashMap<String, Money>;

    fn apply(&mut self, input: Self::Input) {
        let breakdown = input.break_down();
        match breakdown {
            Single(one) => {
                self.apply_transaction(&one);
            },

            Double(first, second) => {
                self.apply_transaction(&first);
                self.apply_transaction(&second);
            },
        }
    }

    fn calculate(&self) -> &Self::Result {
        &self.0
    }
}

///////////////////////////////////////////////////////////////////////////////
