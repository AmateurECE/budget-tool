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
// LAST EDITED:     07/14/2022
////

use std::collections::HashMap;

use crate::calculation::Calculation;
use crate::models;
use crate::total::BurnUpTotal;
use crate::transaction_breakdown::TransactionBreakdown::{self, *};

///////////////////////////////////////////////////////////////////////////////
// TrackedBudgetItem
////

#[derive(Debug)]
pub struct TrackedBudgetItem {
    pub item: models::BudgetItem,
    pub spent: BurnUpTotal,
}

impl From<models::BudgetItem> for TrackedBudgetItem {
    fn from(item: models::BudgetItem) -> Self {
        Self {
            item,
            spent: BurnUpTotal::default(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// BudgetTracker
////

pub struct BudgetTracker(HashMap<i32, TrackedBudgetItem>);

impl Calculation for BudgetTracker {
    type Input = TransactionBreakdown;
    type Result = HashMap<i32, TrackedBudgetItem>;

    // Apply a transaction to the budget tracker. Ensure that a compound
    // transaction only applies to a budget item one time.
    fn apply(&mut self, input: &Self::Input) {
        let transaction = match &input {
            Single(one) => one,
            Double(one, _) => one,
        };

        if let Some(item) = self.0.get_mut(&transaction.owning_id) {
            item.spent.apply(&transaction.amount);
        }
    }

    fn calculate(&self) -> &Self::Result {
        &self.0
    }
}

///////////////////////////////////////////////////////////////////////////////
