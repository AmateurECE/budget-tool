///////////////////////////////////////////////////////////////////////////////
// NAME:            models.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Models
//
// CREATED:         04/10/2022
//
// LAST EDITED:     04/14/2022
////

use std::convert::TryFrom;

use chrono::naive::{NaiveDateTime, serde::ts_milliseconds};
use diesel_derive_enum::DbEnum;
use serde::{Serialize, Deserialize};

///////////////////////////////////////////////////////////////////////////////
// AccountType
////

#[derive(Serialize, Deserialize, DbEnum, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Queryable)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub account_type: AccountType,
    pub apr: f64,

    #[serde(with = "ts_milliseconds")]
    pub accruing_start_date: NaiveDateTime,
}

table! {
    use diesel::sql_types::{Double, Int4, Text, Timestamp};
    use super::AccountTypeMapping;

    accounts (id) {
        id -> Int4,
        name -> Text,
        account_type -> AccountTypeMapping,
        apr -> Double,
        accruing_start_date -> Timestamp,
    }
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="accounts"]
pub struct NewAccount<'a> {
    pub name: &'a str,
    pub account_type: AccountType,
    pub apr: f64,

    #[serde(with = "ts_milliseconds")]
    pub accruing_start_date: NaiveDateTime,
}

///////////////////////////////////////////////////////////////////////////////
// Periodic Budget
////

#[derive(Serialize, Deserialize, Queryable)]
pub struct PeriodicBudget {
    pub id: i32,
    #[serde(with = "ts_milliseconds")]
    pub start_date: NaiveDateTime,
    #[serde(with = "ts_milliseconds")]
    pub end_date: NaiveDateTime,
}

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

#[derive(Serialize, Deserialize, Queryable)]
pub struct OneTimeBudget {
    pub id: i32,
    pub description: String,
}

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

#[derive(Serialize, Deserialize, Queryable)]
pub struct Category {
    pub name: String,
}

table! {
    use diesel::sql_types::Text;

    categories (name) {
        name -> Text,
    }
}

///////////////////////////////////////////////////////////////////////////////
// Budget Items
////

#[derive(Serialize, Deserialize, Queryable)]
pub struct BudgetItem {
    pub id: i32,
    pub description: String,
    pub category: String,
    pub budgeted: f32,
    pub account: i32,
    pub periodic_budget: i32,
    pub one_time_budget: i32,
}

table! {
    use diesel::sql_types::{Int4, Money, Text};

    budget_items (id) {
        id -> Int4,
        description -> Text,
        category -> Text,
        budgeted -> Money,
        account -> Int4,
        periodic_budget -> Int4,
        one_time_budget -> Int4,
    }
}

///////////////////////////////////////////////////////////////////////////////
// TransactionType
////

#[derive(Serialize, Deserialize, DbEnum, Debug, PartialEq)]
pub enum TransactionType {
    Expense,
    Income,
    Transfer,
    Correction,
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
            "Correction" => Ok(TransactionType::Correction),
            _ => Err(ParseTransactionTypeError),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Transaction
////

#[derive(Serialize, Deserialize, Queryable)]
pub struct Transaction {
    pub id: i32,
    pub category: i32,
    pub line_item: i32,
    pub transaction_type: TransactionType,
    pub sending_account: i32,
    pub receiving_account: i32,
    pub transfer_fees: f32,
    pub receiving_entity: String,
    pub amount: f32,
    pub tags: Vec<String>,

    #[serde(with = "ts_milliseconds")]
    pub send_date: NaiveDateTime,

    #[serde(with = "ts_milliseconds")]
    pub receive_date: NaiveDateTime,

    pub corrects: i32,
    pub periodic_budget: i32,
}

table! {
    use diesel::sql_types::{Array, Int4, Money, Timestamp, Text};
    use super::TransactionTypeMapping;

    transactions (id) {
        id -> Int4,
        category -> Int4,
        line_item -> Int4,
        transaction_type -> TransactionTypeMapping,
        sending_account -> Int4,
        receiving_account -> Int4,
        transfer_fees -> Money,
        receiving_entity -> Text,
        amount -> Money,
        tags -> Array<Text>,
        send_date -> Timestamp,
        receive_date -> Timestamp,
        corrects -> Int4,
        periodic_budget -> Int4,
    }
}

///////////////////////////////////////////////////////////////////////////////
