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

impl From<Account> for TrackedAccount {
    fn from(account: Account) -> Self {
        Self {
            account,
            initial_balance: 0,
            current_balance: 0,
        }
    }
}

impl TrackedAccount {
    pub fn with_balance(account: Account, initial_balance: &InitialBalance) ->
        Self
    {
        Self {
            account,
            initial_balance: initial_balance.balance,
            current_balance: initial_balance.balance,
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
        _items: &mut HashMap<i32, TrackedBudgetItem>,
        _accounts: &mut HashMap<String, TrackedAccount>,
        _transaction: &Transaction
    ) {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
