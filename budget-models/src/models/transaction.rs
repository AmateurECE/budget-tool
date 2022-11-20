///////////////////////////////////////////////////////////////////////////////
// NAME:            transaction.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     A transaction.
//
// CREATED:         07/05/2022
//
// LAST EDITED:     11/20/2022
//
// Copyright 2022, Ethan D. Twardy
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
////

use crate::money::Money;
use chrono::{offset::FixedOffset, offset::Utc, DateTime};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use table_iter::prelude::*;

///////////////////////////////////////////////////////////////////////////////
// TransactionData
// Raw data about a transaction occurring on a financial account. This struct
// has all of the interesting data for business logic, but nothing in the way
// of association to other models.
////

#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
pub struct TransactionData {
    pub id: i32,
    pub summary: String,
    pub account: String,
    pub amount: i64,
    pub completed_by: Option<i32>,

    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NewTransactionData {
    pub summary: String,
    pub account: String,
    pub amount: i64,

    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub date: DateTime<Utc>,
}

///////////////////////////////////////////////////////////////////////////////
// TransactionMetadata
// Associates a transaction to a budget and line item. "Real" transactions
// aren't necessarily associated with a budget, though they may be. Planned
// transactions must be associated with a budget.
////

#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
pub struct TransactionMetadata {
    pub periodic_budget: i32,
    pub line_item: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum TransactionType {
    Real,
    Planned,
}

impl Default for TransactionType {
    fn default() -> Self {
        Self::Real
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum TransactionTypedMetadata {
    Real(Option<TransactionMetadata>),
    Planned(TransactionMetadata),
}

///////////////////////////////////////////////////////////////////////////////
// TransactionSeries
// Since a transaction may either occur on one account, or on two (in the case
// of a transfer), this enum allows to treat transactions in an "atomic"
// fashion.
////

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize)]
pub enum TransactionSeries {
    Single(TransactionData),
    Transfer {
        starting: TransactionData,
        completing: TransactionData,
    },
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize)]
pub enum NewTransactionSeries {
    Single(NewTransactionData),
    Transfer {
        starting: NewTransactionData,
        completing: NewTransactionData,
    },
}

///////////////////////////////////////////////////////////////////////////////
// Transaction
// A whole transaction: Includes any associations to budgets or otherwise, and
// contains all relevant transaction data as a transaction series.
////

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Transaction {
    pub id: i32,
    pub metadata: TransactionTypedMetadata,
    pub series: TransactionSeries,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NewTransaction {
    pub metadata: TransactionTypedMetadata,
    pub series: NewTransactionSeries,
}

///////////////////////////////////////////////////////////////////////////////
// InvalidTransactionError
////

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InvalidTransactionError(String);
impl InvalidTransactionError {
    pub fn new<S: AsRef<str>>(message: S) -> Self {
        Self(message.as_ref().to_string())
    }
}

impl Error for InvalidTransactionError {}
impl fmt::Display for InvalidTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

///////////////////////////////////////////////////////////////////////////////
// CondensedTransaction
// This model is not safe for performing business logic on, but layout and
// trait implementations make this model very convenient in user interactions
// (as display or input). Conversion to this type is safe, but conversion from
// this model to any other model may fail.
////

#[derive(
    Clone,
    Debug,
    Default,
    Fields,
    FieldNames,
    PartialEq,
    Deserialize,
    Serialize,
)]
pub struct CondensedTransaction {
    #[fields(rename = "Summary")]
    pub summary: String,
    #[fields(rename = "Line Item")]
    pub line_item: String,
    #[fields(rename = "Date", with = "crate::display::date")]
    pub date: DateTime<FixedOffset>,
    #[fields(rename = "From Account", with = "table_iter::display::option")]
    pub from_account: Option<String>,
    #[fields(rename = "To Account", with = "table_iter::display::option")]
    pub to_account: Option<String>,
    #[fields(rename = "Amount")]
    pub amount: Money,
    #[fields(skip)]
    pub periodic_budget: Option<i32>,
    #[fields(skip)]
    pub starting_id: Option<i32>,
    #[fields(skip)]
    pub completing_id: Option<i32>,
    #[fields(skip)]
    pub transaction_type: TransactionType,
}

impl TryInto<NewTransaction> for CondensedTransaction {
    type Error = InvalidTransactionError;
    fn try_into(self) -> Result<NewTransaction, Self::Error> {
        let metadata = match self.transaction_type {
            TransactionType::Real => TransactionTypedMetadata::Real(
                self.periodic_budget.map(|budget| TransactionMetadata {
                    periodic_budget: budget,
                    line_item: self.line_item,
                }),
            ),
            TransactionType::Planned => {
                TransactionTypedMetadata::Planned(TransactionMetadata {
                    line_item: self.line_item,
                    periodic_budget: self.periodic_budget.ok_or(
                        InvalidTransactionError::new(
                            "No periodic budget for planned transaction",
                        ),
                    )?,
                })
            }
        };

        let amount: i64 = self.amount.into();
        if self.from_account.is_some() && self.to_account.is_some() {
            Ok(NewTransaction {
                metadata,
                series: NewTransactionSeries::Transfer {
                    starting: NewTransactionData {
                        summary: self.summary.clone(),
                        amount: -1 * amount,
                        account: self.from_account.ok_or(
                            InvalidTransactionError::new(
                                "from_account not specified",
                            ),
                        )?,
                        date: self.date.into(),
                    },

                    completing: NewTransactionData {
                        summary: self.summary,
                        amount,
                        account: self.to_account.ok_or(
                            InvalidTransactionError::new(
                                "to_account not specified",
                            ),
                        )?,
                        date: self.date.into(),
                    },
                },
            })
        } else if self.from_account.is_some() {
            Ok(NewTransaction {
                metadata,
                series: NewTransactionSeries::Single(NewTransactionData {
                    summary: self.summary,
                    amount,
                    account: self.from_account.ok_or(
                        InvalidTransactionError::new(
                            "from_account not specified",
                        ),
                    )?,
                    date: self.date.into(),
                }),
            })
        } else if self.to_account.is_some() {
            Ok(NewTransaction {
                metadata,
                series: NewTransactionSeries::Single(NewTransactionData {
                    summary: self.summary,
                    amount,
                    account: self.to_account.ok_or(
                        InvalidTransactionError::new(
                            "to_account not specified",
                        ),
                    )?,
                    date: self.date.into(),
                }),
            })
        } else {
            Err(InvalidTransactionError::new(
                "Neither from_account nor to_account specified",
            ))
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
