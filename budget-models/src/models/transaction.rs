///////////////////////////////////////////////////////////////////////////////
// NAME:            transaction.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     A transaction.
//
// CREATED:         07/05/2022
//
// LAST EDITED:     11/09/2022
////

use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, PartialEq, Serialize)]
pub struct Transaction {
    pub id: i32,
    pub summary: String,
    pub from_account: Option<String>,
    pub to_account: Option<String>,
    pub amount: i64,

    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewTransaction {
    pub summary: String,
    pub from_account: Option<String>,
    pub to_account: Option<String>,
    pub amount: i64,

    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub date: DateTime<Utc>,
}

///////////////////////////////////////////////////////////////////////////////
