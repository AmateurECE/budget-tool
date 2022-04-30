///////////////////////////////////////////////////////////////////////////////
// NAME:            budgetizer.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Performs calculations over a given budget.
//
// CREATED:         04/30/2022
//
// LAST EDITED:     04/30/2022
////

use std::collections::HashMap;

use budget_models::{
    models::{
        Account,
        PeriodicBudget,
        BudgetItem,
        InitialBalance,
        Transaction,
    },
};

///////////////////////////////////////////////////////////////////////////////
// ProgressizeBudgetItem
////

pub struct TrackedBudgetItem {
    pub item: BudgetItem,
    pub spent: i64,
}

impl From<BudgetItem> for TrackedBudgetItem {
    fn from(item: BudgetItem) -> Self {
        Self {
            item,
            spent: 0
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// TrackedAccount
////

pub struct TrackedAccount {
    pub account: Account,
    pub initial_balance: i64,
    pub current_balance: i64,
}

impl TrackedAccount {
    pub fn new(account: Account, initial_balance: i64) -> Self {
        Self {
            account,
            initial_balance,
            current_balance: initial_balance,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Budgetizer
////

pub struct Budgetizer {
    periodic_budget: PeriodicBudget,
}

impl Budgetizer {
    pub fn new(periodic_budget: PeriodicBudget) -> Self {
        Budgetizer {
            periodic_budget,
        }
    }

    // The algorithm: Apply a transaction to a series of accounts and budgets.
    pub fn apply_transaction(
        &self,
        items: &HashMap<i32, TrackedBudgetItem>,
        accounts: &HashMap<String, TrackedAccount>,
        transaction: Transaction
    ) {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
