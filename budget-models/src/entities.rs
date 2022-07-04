///////////////////////////////////////////////////////////////////////////////
// NAME:            entities.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entities--types that have behavior and wrap DB objects.
//
// CREATED:         04/14/2022
//
// LAST EDITED:     07/04/2022
////

use crate::models::{
    BudgetItem,
    InitialBalance,
    PeriodicBudget,
    Transaction,
};

use serde::{Serialize, Deserialize};

///////////////////////////////////////////////////////////////////////////////
// PeriodicBudgetEndpoint
////

/// This struct enables the Periodic Budget view with a single model.
#[derive(Serialize, Deserialize)]
pub struct PeriodicBudgetEndpoint {
    /// This PeriodicBudget.
    pub budget: PeriodicBudget,

    /// List of BudgetItems for this PeriodicBudget
    pub items: Vec<BudgetItem>,

    /// Initial balances for all of the accounts affected by this budget.
    pub initial_balances: Vec<InitialBalance>,

    /// Transactions mapped to this budget's BudgetItems
    pub transactions: Vec<Transaction>,
}

///////////////////////////////////////////////////////////////////////////////
