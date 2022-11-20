///////////////////////////////////////////////////////////////////////////////
// NAME:            transaction.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Operations available on the set of transactions in the
//                  database.
//
// CREATED:         11/11/2022
//
// LAST EDITED:     11/19/2022
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

use budget_backend_lib::error::{MissingBudgetError, MissingLineItemError};
use budget_backend_lib::prelude::*;
use budget_backend_lib::transaction;
use budget_models::models::{self, CondensedTransaction};
use chrono::{
    naive::{NaiveDate, NaiveTime},
    offset::Local,
};
use clap::{Subcommand, ValueEnum};
use sea_orm::prelude::*;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use std::fs::File;

use crate::table;

///////////////////////////////////////////////////////////////////////////////
// TransactionType
// Unfortunately, this is necessary (otherwise models::TransactionType must
// implement ValueEnum).
////

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum TransactionType {
    Real,
    Planned,
}

impl Into<models::TransactionType> for TransactionType {
    fn into(self) -> models::TransactionType {
        match self {
            TransactionType::Real => models::TransactionType::Real,
            TransactionType::Planned => models::TransactionType::Planned,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// TransactionRecord
////

#[derive(Clone, Debug, Deserialize)]
struct TransactionRecord {
    #[serde(rename = "Summary")]
    summary: String,
    #[serde(rename = "Line Item")]
    line_item: String,
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "From Account")]
    from_account: Option<String>,
    #[serde(rename = "To Account")]
    to_account: Option<String>,
    #[serde(rename = "Amount")]
    amount: f64,
}

impl TryInto<CondensedTransaction> for TransactionRecord {
    type Error = chrono::format::ParseError;
    fn try_into(self) -> Result<CondensedTransaction, Self::Error> {
        Ok(CondensedTransaction {
            summary: self.summary,
            line_item: self.line_item,
            from_account: self.from_account,
            to_account: self.to_account,
            amount: self.amount.into(),
            date: NaiveDate::parse_from_str(&self.date, "%m/%d/%y")?
                .and_time(NaiveTime::default())
                .and_local_timezone(Local)
                .single()
                .unwrap()
                .into(),
            ..Default::default()
        })
    }
}

async fn read_transactions(
    filename: &str,
    db: &DatabaseConnection,
    transaction_type: TransactionType,
) -> anyhow::Result<Vec<CondensedTransaction>> {
    // Read the input records into a vector
    let mut records = csv::Reader::from_reader(File::open(filename)?)
        .deserialize()
        .collect::<Result<Vec<TransactionRecord>, _>>()?
        .into_iter()
        .map(|t| t.try_into())
        .collect::<Result<Vec<CondensedTransaction>, _>>()?;
    let dates = records
        .iter()
        .map(|r| r.date)
        .collect::<Vec<DateTimeWithTimeZone>>();
    let min_date = dates.iter().min().unwrap();
    let max_date = dates.iter().max().unwrap();

    let budgets = PeriodicBudgets::find()
        .filter(periodic_budgets::Column::StartDate.lte(*min_date))
        .filter(periodic_budgets::Column::EndDate.gte(*max_date))
        .all(db)
        .await?;

    let line_items = LineItems::find().all(db).await?;
    for mut record in &mut records {
        let CondensedTransaction {
            line_item, date, ..
        } = record;
        if line_items
            .iter()
            .find(|l| l.summary == *line_item)
            .is_none()
        {
            return Err(MissingLineItemError::new(line_item).into());
        }

        record.transaction_type = transaction_type.into();
        record.periodic_budget = Some(
            budgets
                .iter()
                .find(|&b| b.start_date <= *date && b.end_date >= *date)
                .ok_or_else(|| {
                    MissingBudgetError::new(
                        date.format("%d %b %Y").to_string(),
                    )
                })?
                .id,
        );
    }
    Ok(records)
}

///////////////////////////////////////////////////////////////////////////////
// Operation Functions
////

// Import a list of transactions.
async fn import(
    filename: &str,
    transaction_type: TransactionType,
    db: &DatabaseConnection,
) -> anyhow::Result<()> {
    let records = read_transactions(filename, db, transaction_type).await?;
    for record in &records {
        transaction::create(db, record.clone().try_into()?).await?;
    }

    table::print(&records);
    Ok(())
}

async fn delete_all_planned(
    budget: i32,
    db: &DatabaseConnection,
) -> anyhow::Result<()> {
    let transactions = PlannedTransactions::find()
        .filter(planned_transactions::Column::PeriodicBudget.eq(budget))
        .all(db)
        .await?
        .iter()
        .map(|transaction| transaction.id)
        .collect::<Vec<i32>>();
    transactions::Entity::delete_many()
        .filter(transactions::Column::Id.is_in(transactions))
        .exec(db)
        .await?;
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
// Public Interface
////

#[derive(Subcommand)]
pub(crate) enum Verb {
    /// Import a set of transactions from a CSV file.
    Import {
        /// The file to import transactions from.
        #[clap(value_parser)]
        filename: String,
    },

    /// Delete all transactions which reference a budget.
    DeleteAll {
        /// The budget ID
        #[clap(value_parser)]
        budget: i32,
    },
}

pub(crate) async fn op(
    verb: &Verb,
    transaction_type: TransactionType,
    db: &DatabaseConnection,
) -> anyhow::Result<()> {
    match verb {
        Verb::Import { filename } => {
            import(filename, transaction_type, db).await
        }
        Verb::DeleteAll { budget } => match transaction_type {
            TransactionType::Planned => delete_all_planned(*budget, db).await,
            TransactionType::Real => todo!(),
        },
    }
}

///////////////////////////////////////////////////////////////////////////////
