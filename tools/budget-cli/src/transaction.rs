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
// LAST EDITED:     11/12/2022
////

use budget_backend_lib::prelude::*;
use budget_models::Money;
use chrono::{
    naive::{NaiveDate, NaiveTime},
    offset::Local,
};
use clap::{Subcommand, ValueEnum};
use csv;
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, Set};
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs::File;

use crate::table;

///////////////////////////////////////////////////////////////////////////////
// TransactionType
////

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum TransactionType {
    Real,
    Planned,
}

///////////////////////////////////////////////////////////////////////////////
// MissingInstanceError
////

#[derive(Debug)]
struct MissingInstanceError {
    budget: i32,
    line_item: String,
}

impl Error for MissingInstanceError {}

impl fmt::Display for MissingInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "periodic budget {} has no instance of line item {}",
            self.budget, &self.line_item
        )
    }
}

///////////////////////////////////////////////////////////////////////////////
// MissingBudgetError
////

#[derive(Debug)]
struct MissingBudgetError(String);

impl Error for MissingBudgetError {}

impl fmt::Display for MissingBudgetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "no periodic budget exists which includes date {}",
            &self.0
        )
    }
}

///////////////////////////////////////////////////////////////////////////////
// PoorlyFormedTimeError
////

#[derive(Debug)]
struct PoorlyFormedTimeError(String);
impl Error for PoorlyFormedTimeError {}
impl fmt::Display for PoorlyFormedTimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Date '{}' is poorly formed!", &self.0)
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

#[derive(Clone, Debug)]
struct AssociatedTransaction {
    summary: String,
    line_item: String,
    date: DateTimeWithTimeZone,
    from_account: Option<String>,
    to_account: Option<String>,
    amount: f64,
    budget: i32,
}

async fn associate(
    transactions: &[TransactionRecord],
    db: &DatabaseConnection,
) -> anyhow::Result<Vec<AssociatedTransaction>> {
    let mut budgets: Vec<periodic_budgets::Model> = Vec::new();
    let mut associated: Vec<AssociatedTransaction> = Vec::new();
    for transaction in transactions {
        let date: DateTimeWithTimeZone =
            NaiveDate::parse_from_str(&transaction.date, "%m/%d/%y")?
                .and_time(NaiveTime::default())
                .and_local_timezone(Local)
                .single()
                .ok_or(PoorlyFormedTimeError(transaction.date.clone()))?
                .into();

        let budget = budgets
            .iter()
            .find(|&b| b.start_date <= date && b.end_date >= date);
        let budget = match budget {
            Some(budget) => budget,
            None => {
                let budget = PeriodicBudgets::find()
                    .filter(periodic_budgets::Column::StartDate.lte(date))
                    .filter(periodic_budgets::Column::EndDate.gte(date))
                    .one(db)
                    .await?
                    .ok_or(MissingBudgetError(date.to_string()))?;
                budgets.push(budget);
                budgets.last().unwrap()
            }
        };

        let new_transaction = AssociatedTransaction {
            budget: budget.id,
            summary: transaction.summary.clone(),
            line_item: transaction.line_item.clone(),
            date,
            from_account: transaction.from_account.clone(),
            to_account: transaction.to_account.clone(),
            amount: transaction.amount,
        };
        associated.push(new_transaction);
    }

    Ok(associated)
}

///////////////////////////////////////////////////////////////////////////////
// Operation Functions
////

async fn import(
    filename: &str,
    transaction_type: TransactionType,
    db: &DatabaseConnection,
) -> anyhow::Result<()> {
    // Read the input records into a vector
    let records = csv::Reader::from_reader(File::open(filename)?)
        .deserialize()
        .collect::<Result<Vec<TransactionRecord>, _>>()?;
    let records = associate(&records, db).await?;

    // TODO: Add periodic_budget id field to TransactionRecord, or allow
    // conversion of TransactionRecord into another struct that has one.

    for record in records.iter() {
        // Create the Transaction instances
        let transaction_model = transactions::ActiveModel {
            summary: Set(record.summary.clone()),
            date: Set(record.date),
            from_account: Set(record.from_account.clone()),
            to_account: Set(record.to_account.clone()),
            amount: Set(Money::from(record.amount).into()),
            ..Default::default()
        };

        // TODO: Should do this operation before attempting to insert ANY
        // transactions.
        let line_item_instance = LineItemInstances::find()
            .filter(
                line_item_instances::Column::PeriodicBudget.eq(record.budget),
            )
            .filter(
                line_item_instances::Column::Summary
                    .eq(record.line_item.clone()),
            )
            .one(db)
            .await?
            .ok_or(MissingInstanceError {
                budget: record.budget,
                line_item: record.line_item.clone(),
            })?;

        let id = Transactions::insert(transaction_model)
            .exec(db)
            .await?
            .last_insert_id;

        // Create the Real or Planned Transaction instances
        match transaction_type {
            TransactionType::Real => {
                let real_model = real_transactions::ActiveModel {
                    transaction: Set(id),
                    line_item_instance: Set(Some(line_item_instance.id)),
                    ..Default::default()
                };
                RealTransactions::insert(real_model).exec(db).await?;
            }

            TransactionType::Planned => {
                let planned_model = planned_transactions::ActiveModel {
                    transaction: Set(id),
                    line_item_instance: Set(line_item_instance.id),
                    ..Default::default()
                };
                PlannedTransactions::insert(planned_model).exec(db).await?;
            }
        }
    }

    let formatted_records = records
        .iter()
        .map(|record| {
            vec![
                record.summary.to_string(),
                record.line_item.to_string(),
                record.date.format("%d %b %Y").to_string(),
                record
                    .from_account
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("")
                    .to_owned(),
                record
                    .to_account
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("")
                    .to_owned(),
                Money::from(record.amount).to_string(),
            ]
        })
        .collect::<Vec<Vec<String>>>();
    let headers = vec![];
    table::print(
        &formatted_records
            .iter()
            .map(|record| record.as_slice())
            .collect::<Vec<&[String]>>(),
        &headers,
    );
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
}

pub(crate) async fn op(
    verb: &Verb,
    transaction_type: TransactionType,
    db: &DatabaseConnection,
) -> anyhow::Result<()> {
    match verb {
        Verb::Import { filename } => {
            import(&filename, transaction_type, db).await
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
