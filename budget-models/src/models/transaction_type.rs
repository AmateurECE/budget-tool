///////////////////////////////////////////////////////////////////////////////
// NAME:            transaction_type.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     The type of a transaction
//
// CREATED:         07/05/2022
//
// LAST EDITED:     07/07/2022
////

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, Deserialize, EnumIter, PartialEq, Serialize)]
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
