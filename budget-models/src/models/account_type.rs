///////////////////////////////////////////////////////////////////////////////
// NAME:            account_type.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Type of an account
//
// CREATED:         07/05/2022
//
// LAST EDITED:     07/05/2022
////

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum AccountType {
    Checking,
    Saving,
    Credit,
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
