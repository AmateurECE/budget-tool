///////////////////////////////////////////////////////////////////////////////
// NAME:            budgetizer.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Performs calculations over a given budget.
//
// CREATED:         04/30/2022
//
// LAST EDITED:     05/03/2022
////

use std::collections::HashMap;

use budget_models::{
    models::{
        Account,
        PeriodicBudget,
        BudgetItem,
        InitialBalance,
        Transaction,
        TransactionType,
    },
};

///////////////////////////////////////////////////////////////////////////////
// ProgressizeBudgetItem
////

#[derive(Debug)]
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

#[derive(Debug)]
pub struct TrackedAccount {
    pub account: Account,
    pub initial_balance: i64,
    pub expected_end_balance: i64,
    pub current_balance: i64,
}

impl From<Account> for TrackedAccount {
    fn from(account: Account) -> Self {
        Self {
            account,
            initial_balance: 0,
            expected_end_balance: 0,
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
            expected_end_balance: initial_balance.balance,
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

    // First algorithm: Predict an account's end balance, based on budget.
    pub fn predict_balance(
        &self,
        accounts: &mut HashMap<String, TrackedAccount>,
        item: &TrackedBudgetItem,
    ) {
        use TransactionType::*;
        let transaction_type = item.item.transaction_type;
        let budgeted = item.item.budgeted;
        let from_account = &item.item.from_account;
        let to_account = &item.item.to_account;

        if Income != transaction_type {
            let selected_account: &mut TrackedAccount = accounts
                .get_mut(from_account.as_ref().unwrap())
                .unwrap();
            selected_account.expected_end_balance += -1 * budgeted;
        }

        if Expense != transaction_type {
            let selected_account: &mut TrackedAccount = accounts
                .get_mut(to_account.as_ref().unwrap())
                .unwrap();
            selected_account.expected_end_balance += budgeted;
        }
    }

    // Second algorithm: Apply a transaction to series of accounts and budgets.
    pub fn apply_transaction(
        &self,
        items: &mut HashMap<i32, TrackedBudgetItem>,
        accounts: &mut HashMap<String, TrackedAccount>,
        transaction: &Transaction
    ) {
        use TransactionType::*;

        let item: &mut TrackedBudgetItem = items
            .get_mut(&transaction.line_item)
            .unwrap();
        let from_account = &transaction.sending_account;
        let to_account = &transaction.receiving_account;
        let transaction_type = transaction.transaction_type;

        if Income != transaction_type {
            let account: &mut TrackedAccount = accounts
                .get_mut(from_account.as_ref().unwrap())
                .unwrap();
            account.current_balance += -1 * transaction.amount;
            item.spent += -1 * transaction.amount;
        }

        if Expense != transaction_type {
            let account: &mut TrackedAccount = accounts
                .get_mut(to_account.as_ref().unwrap())
                .unwrap();
            account.current_balance += transaction.amount;
            item.spent += transaction.amount;
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
