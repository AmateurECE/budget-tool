///////////////////////////////////////////////////////////////////////////////
// NAME:            models.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Models
//
// CREATED:         04/10/2022
//
// LAST EDITED:     05/09/2022
////

use std::convert::TryFrom;

use chrono::naive::{
    NaiveDateTime, serde::{ts_milliseconds, ts_milliseconds_option},
};
use serde::{Serialize, Deserialize};

#[cfg(not(target_family = "wasm"))]
use diesel_derive_enum::DbEnum;

///////////////////////////////////////////////////////////////////////////////
// AccountType
////

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(not(target_family = "wasm"), derive(Queryable))]
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

#[cfg(not(target_family = "wasm"))]
table! {
    use diesel::sql_types::{Int4, BigInt, Nullable, Text};
    use super::TransactionTypeMapping;

    budget_items (id) {
        id -> Int4,
        description -> Text,
        category -> Text,
        budgeted -> BigInt,
        transaction_type -> TransactionTypeMapping,
        from_account -> Nullable<Text>,
        to_account -> Nullable<Text>,
        periodic_budget -> Int4,
        one_time_budget -> Nullable<Int4>,
    }
}

///////////////////////////////////////////////////////////////////////////////
// TransactionType
////

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Debug)]
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

impl ToString for TransactionType {
    fn to_string(&self) -> String {
        use TransactionType::*;
        match &self {
            Expense => "Expense".to_string(),
            Income => "Income".to_string(),
            Transfer => "Transfer".to_string(),
            Payment => "Payment".to_string(),
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
    pub sending_account: Option<String>,
    pub receiving_account: Option<String>,
    pub transfer_fees: Option<i64>,
    pub receiving_entity: Option<String>,
    pub amount: i64,
    pub tags: Option<Vec<String>>,

    #[serde(with = "ts_milliseconds")]
    pub send_date: NaiveDateTime,

    #[serde(with = "ts_milliseconds_option")]
    pub receive_date: Option<NaiveDateTime>,

    pub corrects: Option<Vec<i32>>,
    pub periodic_budget: i32,
}

#[cfg(not(target_family = "wasm"))]
table! {
    use diesel::sql_types::{Array, BigInt, Int4, Nullable, Timestamp, Text};
    use super::TransactionTypeMapping;

    transactions (id) {
        id -> Int4,
        description -> Text,
        line_item -> Int4,
        transaction_type -> TransactionTypeMapping,
        sending_account -> Nullable<Text>,
        receiving_account -> Nullable<Text>,
        transfer_fees -> Nullable<BigInt>,
        receiving_entity -> Nullable<Text>,
        amount -> BigInt,
        tags -> Nullable<Array<Text>>,
        send_date -> Timestamp,
        receive_date -> Nullable<Timestamp>,
        corrects -> Nullable<Array<Int4>>,
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
