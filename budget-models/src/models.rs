///////////////////////////////////////////////////////////////////////////////
// NAME:            models.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Models
//
// CREATED:         04/10/2022
//
// LAST EDITED:     04/11/2022
////
use std::convert::TryFrom;

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
}

table! {
    use diesel::sql_types::{Int4, Varchar};
    use super::AccountTypeMapping;

    accounts (id) {
        id -> Int4,
        name -> Varchar,
        account_type -> AccountTypeMapping,
    }
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="accounts"]
pub struct NewAccount<'a> {
    pub name: &'a str,
    pub account_type: AccountType,
}

///////////////////////////////////////////////////////////////////////////////
