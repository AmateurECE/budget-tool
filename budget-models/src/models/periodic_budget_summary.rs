///////////////////////////////////////////////////////////////////////////////
// NAME:            periodic_budget_summary.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     A summary of a single budget
//
// CREATED:         07/05/2022
//
// LAST EDITED:     07/05/2022
////

use serde::{Serialize, Deserialize};
use super::{PeriodicBudget, BudgetItem, InitialBalance, Transaction};

/// This struct enables the Periodic Budget view with a single model.
#[derive(Serialize, Deserialize)]
pub struct PeriodicBudgetSummary {
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
