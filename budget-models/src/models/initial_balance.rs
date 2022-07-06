///////////////////////////////////////////////////////////////////////////////
// NAME:            initial_balance.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Balance snapshot of an account.
//
// CREATED:         07/05/2022
//
// LAST EDITED:     07/05/2022
////

use chrono::{DateTime, offset::Utc};
use serde::{Serialize, Deserialize};

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
