///////////////////////////////////////////////////////////////////////////////
// NAME:            account.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     An account with a financial institution.
//
// CREATED:         07/05/2022
//
// LAST EDITED:     07/09/2022
////

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use super::AccountType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub account_type: AccountType,

    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub date_opened: DateTime<Utc>,

    #[serde(with = "chrono::serde::ts_milliseconds_option")]
    pub date_closed: Option<DateTime<Utc>>,
}

///////////////////////////////////////////////////////////////////////////////
