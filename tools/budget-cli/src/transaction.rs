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
// LAST EDITED:     11/13/2022
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
use table_iter::prelude::*;

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

#[derive(Clone, Debug, Fields, FieldNames)]
struct AssociatedTransaction {
    #[fields(rename = "Summary")]
    summary: String,
    #[fields(rename = "Line Item")]
    line_item: String,
    #[fields(rename = "Date", with = "crate::display::date")]
    date: DateTimeWithTimeZone,
    #[fields(rename = "From Account", with = "table_iter::display::option")]
    from_account: Option<String>,
    #[fields(rename = "To Account", with = "table_iter::display::option")]
    to_account: Option<String>,
    #[fields(rename = "Amount")]
    amount: f64,
    #[fields(rename = "Budget")]
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

    table::print(&records);
    Ok(())
}

async fn delete_all_planned(
    budget: i32,
    db: &DatabaseConnection,
) -> anyhow::Result<()> {
    let items = LineItemInstances::find()
        .filter(line_item_instances::Column::PeriodicBudget.eq(budget))
        .all(db)
        .await?
        .iter()
        .map(|instance| instance.id)
        .collect::<Vec<i32>>();

    planned_transactions::Entity::delete_many()
        .filter(planned_transactions::Column::LineItemInstance.is_in(items))
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
            import(&filename, transaction_type, db).await
        }
        Verb::DeleteAll { budget } => match transaction_type {
            TransactionType::Planned => delete_all_planned(*budget, db).await,
            TransactionType::Real => todo!(),
        },
    }
}

///////////////////////////////////////////////////////////////////////////////
