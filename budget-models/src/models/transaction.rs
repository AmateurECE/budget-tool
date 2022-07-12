///////////////////////////////////////////////////////////////////////////////
// NAME:            transaction.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     A transaction.
//
// CREATED:         07/05/2022
//
// LAST EDITED:     07/12/2022
////

use chrono::{DateTime, offset::Utc};
use serde::{Serialize, Deserialize};
use super::TransactionType;

#[derive(Clone, Default, Deserialize, PartialEq, Serialize)]
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

    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub send_date: DateTime<Utc>,

    #[serde(with = "chrono::serde::ts_milliseconds_option")]
    pub receive_date: Option<DateTime<Utc>>,

    pub corrects: Option<Vec<i32>>,
    pub periodic_budget: i32,
}

///////////////////////////////////////////////////////////////////////////////
