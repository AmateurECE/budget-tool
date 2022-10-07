///////////////////////////////////////////////////////////////////////////////
// NAME:            lib.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Models for the application.
//
// CREATED:         10/06/2022
//
// LAST EDITED:     10/06/2022
////

use chrono::{DateTime, offset::FixedOffset};

// A periodic budget, which applies to all transactions between two dates.
pub struct PeriodicBudget {
    pub id: i32,
    pub start_date: DateTime<FixedOffset>,
    pub end_date: DateTime<FixedOffset>,
}

// A "LineItem" that may be instantiated for a particular budget.
pub struct LineItem {
    pub id: i32,
    pub description: String,
}

// A transaction that is applied to one (or two) accounts, with a fixed amount,
// on a particular date.
pub struct Transaction {
    pub id: i32,
    pub line_item: i32,
    pub from_account: String,
    pub to_account: Option<String>,
    pub amount: f64,
    pub date: DateTime<FixedOffset>,
}

// A constraint on a budget item that allows traceability between transactions
// and budget itesm.
pub struct BudgetConstraint {
    pub id: i32,
    pub line_item: i32,
    pub from_account: String,
    pub to_account: Option<String>,
    pub amount: f64,
}

///////////////////////////////////////////////////////////////////////////////
