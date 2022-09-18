///////////////////////////////////////////////////////////////////////////////
// NAME:            budget_item.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     An item in a budget.
//
// CREATED:         07/05/2022
//
// LAST EDITED:     07/05/2022
////

use super::TransactionType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BudgetItem {
    pub id: i32,
    pub description: String,
    pub category: String,
    pub budgeted: i64,
    pub transaction_type: TransactionType,
    pub from_account: Option<String>,
    pub to_account: Option<String>,
    pub periodic_budget: i32,
    pub one_time_budget: Option<i32>,
}

///////////////////////////////////////////////////////////////////////////////
