///////////////////////////////////////////////////////////////////////////////
// NAME:            budget_tracker.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Calculations and algorithms for tracking expenses/incomes
//                  against a budget.
//
// CREATED:         07/14/2022
//
// LAST EDITED:     07/15/2022
////

use std::collections::HashMap;

use crate::calculation::Calculation;
use crate::models;
use crate::total::BurnUpTotal;
use crate::transaction_breakdown::{Breakdown, TransactionBreakdown::*};

///////////////////////////////////////////////////////////////////////////////
// BudgetTracker
////

pub struct BudgetTracker(HashMap<i32, BurnUpTotal>);

impl Calculation for BudgetTracker {
    type Input = models::Transaction;
    type Result = HashMap<i32, BurnUpTotal>;

    // Apply a transaction to the budget tracker. Ensure that a compound
    // transaction only applies to a budget item one time.
    fn apply(&mut self, input: Self::Input) {
        let breakdown = input.break_down();
        let transaction = match &breakdown {
            Single(one) => one,
            Double(one, _) => one,
        };

        if let Some(item) = self.0.get_mut(&transaction.owning_id) {
            item.apply(transaction.amount);
        }
    }

    fn calculate(&self) -> &Self::Result {
        &self.0
    }
}

///////////////////////////////////////////////////////////////////////////////
