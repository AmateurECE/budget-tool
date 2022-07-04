///////////////////////////////////////////////////////////////////////////////
// NAME:            models.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Models
//
// CREATED:         04/10/2022
//
// LAST EDITED:     07/04/2022
////

use std::convert::TryFrom;
use chrono::{DateTime, offset::Utc};
use serde::{Serialize, Deserialize};

#[cfg(feature = "sea-orm")]
use sea_orm::{sea_query, entity::prelude::*};

///////////////////////////////////////////////////////////////////////////////
// AccountType
////

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "sea-orm", derive(DeriveActiveEnum, EnumIter, Iden))]
#[cfg_attr(feature = "sea-orm", sea_orm(rs_type = "String", db_type = "Enum"))]
pub enum AccountType {
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "checking"))]
    Checking,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "saving"))]
    Saving,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "credit"))]
    Credit,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "loan"))]
    Loan,
}

impl ToString for AccountType {
    fn to_string(&self) -> String {
        format!("{:?}", &self)
    }
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub account_type: AccountType,
}

///////////////////////////////////////////////////////////////////////////////
// Periodic Budget
////

#[derive(Clone, Serialize, Deserialize)]
pub struct PeriodicBudget {
    pub id: i32,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub start_date: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub end_date: DateTime<Utc>,
}

///////////////////////////////////////////////////////////////////////////////
// One-Time Budgets
////

#[derive(Serialize, Deserialize)]
pub struct OneTimeBudget {
    pub id: i32,
    pub description: String,
}

///////////////////////////////////////////////////////////////////////////////
// Categories
////

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub name: String,
}

///////////////////////////////////////////////////////////////////////////////
// Budget Items
////

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
// TransactionType
////

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Debug)]
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

impl Default for TransactionType {
    fn default() -> Self {
        Self::Expense
    }
}

///////////////////////////////////////////////////////////////////////////////
// Transaction
////

#[derive(Serialize, Deserialize, Clone, PartialEq)]
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

    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub send_date: DateTime<Utc>,

    #[serde(with = "chrono::serde::ts_milliseconds_option")]
    pub receive_date: Option<DateTime<Utc>>,

    pub corrects: Option<Vec<i32>>,
    pub periodic_budget: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewTransaction {
    pub description: String,
    pub line_item: i32,
    pub transaction_type: TransactionType,
    pub sending_account: Option<String>,
    pub receiving_account: Option<String>,
    pub transfer_fees: Option<i64>,
    pub receiving_entity: Option<String>,
    pub amount: i64,
    pub tags: Option<Vec<String>>,

    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub send_date: DateTime<Utc>,

    #[serde(with = "chrono::serde::ts_milliseconds_option")]
    pub receive_date: Option<DateTime<Utc>>,

    pub corrects: Option<Vec<i32>>,
    pub periodic_budget: i32,
}

///////////////////////////////////////////////////////////////////////////////
// Initial Balances
////

#[derive(Serialize, Deserialize)]
pub struct InitialBalance {
    pub id: i32,
    pub account: String,
    pub budget: i32,
    pub balance: i64,

    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub last_updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct NewInitialBalance {
    pub account: String,
    pub budget: i32,
    pub balance: i64,
}

///////////////////////////////////////////////////////////////////////////////
