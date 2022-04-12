///////////////////////////////////////////////////////////////////////////////
// NAME:            models.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Models
//
// CREATED:         04/10/2022
//
// LAST EDITED:     04/12/2022
////

use std::convert::TryFrom;

use chrono::naive::{NaiveDateTime, serde::ts_milliseconds};
use diesel_derive_enum::DbEnum;
use serde::{Serialize, Deserialize};

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
    use diesel::sql_types::{Double, Int4, Varchar, Timestamp};
    use super::AccountTypeMapping;

    accounts (id) {
        id -> Int4,
        name -> Varchar,
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
