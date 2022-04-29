///////////////////////////////////////////////////////////////////////////////
// NAME:            models.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Models
//
// CREATED:         04/10/2022
//
// LAST EDITED:     04/29/2022
////

use std::convert::TryFrom;

use chrono::naive::{NaiveDateTime, serde::ts_milliseconds};
use serde::{Serialize, Deserialize};

#[cfg(not(target_family = "wasm"))]
use diesel_derive_enum::DbEnum;

///////////////////////////////////////////////////////////////////////////////
// AccountType
////

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(not(target_family = "wasm"), derive(DbEnum))]
pub enum AccountType {
    Checking,
    Saving,
    Credit,
    Loan,
}

#[derive(Debug)]
pub struct ParseAccountTypeError;

impl TryFrom<String> for AccountType {
    type Error = ParseAccountTypeError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Checking" => Ok(AccountType::Checking),
            "Saving" => Ok(AccountType::Saving),
            "Credit" => Ok(AccountType::Credit),
            "Loan" => Ok(AccountType::Loan),
            _ => Err(ParseAccountTypeError),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Account
////

#[cfg(not(target_family = "wasm"))]
table! {
    use diesel::sql_types::{Double, Text, Timestamp};
    use super::AccountTypeMapping;

    accounts (name) {
        name -> Text,
        account_type -> AccountTypeMapping,
        apr -> Double,
        accruing_start_date -> Timestamp,
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(not(target_family = "wasm"), derive(Queryable, Insertable))]
#[cfg_attr(not(target_family = "wasm"), table_name="accounts")]
pub struct Account {
    pub name: String,
    pub account_type: AccountType,
    pub apr: f64,

    #[serde(with = "ts_milliseconds")]
    pub accruing_start_date: NaiveDateTime,
}

///////////////////////////////////////////////////////////////////////////////
// Periodic Budget
////

#[derive(Serialize, Deserialize)]
#[cfg_attr(not(target_family = "wasm"), derive(Queryable))]
pub struct PeriodicBudget {
    pub id: i32,
    #[serde(with = "ts_milliseconds")]
    pub start_date: NaiveDateTime,
    #[serde(with = "ts_milliseconds")]
    pub end_date: NaiveDateTime,
}

#[cfg(not(target_family = "wasm"))]
table! {
    use diesel::sql_types::{Int4, Timestamp};

    periodic_budgets (id) {
        id -> Int4,
        start_date -> Timestamp,
        end_date -> Timestamp,
    }
}

///////////////////////////////////////////////////////////////////////////////
// One-Time Budgets
////

#[derive(Serialize, Deserialize)]
#[cfg_attr(not(target_family = "wasm"), derive(Queryable))]
pub struct OneTimeBudget {
    pub id: i32,
    pub description: String,
}

#[cfg(not(target_family = "wasm"))]
table! {
    use diesel::sql_types::{Int4, Text};

    one_time_budgets (id) {
        id -> Int4,
        description -> Text,
    }
}

///////////////////////////////////////////////////////////////////////////////
// Categories
////

#[derive(Serialize, Deserialize)]
#[cfg_attr(not(target_family = "wasm"), derive(Queryable))]
pub struct Category {
    pub name: String,
}

#[cfg(not(target_family = "wasm"))]
table! {
    use diesel::sql_types::Text;

    categories (name) {
        name -> Text,
    }
}

///////////////////////////////////////////////////////////////////////////////
// Budget Items
////

#[derive(Serialize, Deserialize)]
#[cfg_attr(not(target_family = "wasm"), derive(Queryable))]
pub struct BudgetItem {
    pub id: i32,
    pub description: String,
    pub category: String,
    pub budgeted: i64,
    pub transaction_type: TransactionType,
    pub from_account: String,
    pub to_account: String,
    pub periodic_budget: i32,
    pub one_time_budget: i32,
}

#[cfg(not(target_family = "wasm"))]
table! {
    use diesel::sql_types::{Int4, BigInt, Text};
    use super::TransactionTypeMapping;

    budget_items (id) {
        id -> Int4,
        description -> Text,
        category -> Text,
        budgeted -> BigInt,
        transaction_type -> TransactionTypeMapping,
        from_account -> Text,
        to_account -> Text,
        periodic_budget -> Int4,
        one_time_budget -> Int4,
    }
}

///////////////////////////////////////////////////////////////////////////////
// TransactionType
////

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(not(target_family = "wasm"), derive(DbEnum))]
pub enum TransactionType {
    Expense,
    Income,
    Transfer,
    Payment,
}

#[derive(Debug)]
pub struct ParseTransactionTypeError;

impl TryFrom<String> for TransactionType {
    type Error = ParseTransactionTypeError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Expense" => Ok(TransactionType::Expense),
            "Income" => Ok(TransactionType::Income),
            "Transfer" => Ok(TransactionType::Transfer),
            "Payment" => Ok(TransactionType::Payment),
            _ => Err(ParseTransactionTypeError),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Transaction
////

#[derive(Serialize, Deserialize)]
#[cfg_attr(not(target_family = "wasm"), derive(Queryable))]
pub struct Transaction {
    pub id: i32,
    pub description: String,
    pub line_item: i32,
    pub transaction_type: TransactionType,
    pub sending_account: String,
    pub receiving_account: String,
    pub transfer_fees: i64,
    pub receiving_entity: String,
    pub amount: i64,
    pub tags: Vec<String>,

    #[serde(with = "ts_milliseconds")]
    pub send_date: NaiveDateTime,

    #[serde(with = "ts_milliseconds")]
    pub receive_date: NaiveDateTime,

    pub corrects: Vec<i32>,
    pub periodic_budget: i32,
}

#[cfg(not(target_family = "wasm"))]
table! {
    use diesel::sql_types::{Array, BigInt, Int4, Timestamp, Text};
    use super::TransactionTypeMapping;

    transactions (id) {
        id -> Int4,
        description -> Text,
        line_item -> Int4,
        transaction_type -> TransactionTypeMapping,
        sending_account -> Text,
        receiving_account -> Text,
        transfer_fees -> BigInt,
        receiving_entity -> Text,
        amount -> BigInt,
        tags -> Array<Text>,
        send_date -> Timestamp,
        receive_date -> Timestamp,
        corrects -> Array<Int4>,
        periodic_budget -> Int4,
    }
}

///////////////////////////////////////////////////////////////////////////////
// Initial Balances
////

#[derive(Serialize, Deserialize)]
#[cfg_attr(not(target_family = "wasm"), derive(Queryable))]
pub struct InitialBalance {
    pub id: i32,
    pub account: String,
    pub budget: i32,
    pub balance: i64,

    #[serde(with = "ts_milliseconds")]
    pub last_updated: NaiveDateTime,
}

#[cfg(not(target_family = "wasm"))]
table! {
    use diesel::sql_types::{Int4, BigInt, Text, Timestamp};

    initial_balances (id) {
        id -> Int4,
        account -> Text,
        budget -> Int4,
        balance -> BigInt,
        last_updated -> Timestamp,
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(not(target_family = "wasm"), derive(Insertable))]
#[cfg_attr(not(target_family = "wasm"), table_name="initial_balances")]
pub struct NewInitialBalance {
    pub account: String,
    pub budget: i32,
    pub balance: i64,
}

///////////////////////////////////////////////////////////////////////////////
