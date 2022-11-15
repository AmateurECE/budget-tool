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
// LAST EDITED:     11/15/2022
////

use budget_backend_lib::error::{MissingBudgetError, MissingInstanceError};
use budget_backend_lib::prelude::*;
use budget_models::Money;
use chrono::{
    naive::{NaiveDate, NaiveTime},
    offset::Local,
};
use clap::{Subcommand, ValueEnum};
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, Set};
use serde::Deserialize;
use std::fs::File;
use table_iter::prelude::*;

use crate::table;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum TransactionType {
    Real,
    Planned,
}

#[derive(Clone, Debug)]
struct AssociatedLineItem {
    name: String,
    instance_id: i32,
}

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
    #[fields(rename = "Line Item", with = "display_line_item")]
    line_item: AssociatedLineItem,
    #[fields(rename = "Date", with = "crate::display::date")]
    date: DateTimeWithTimeZone,
    #[fields(rename = "From Account", with = "table_iter::display::option")]
    from_account: Option<String>,
    #[fields(rename = "To Account", with = "table_iter::display::option")]
    to_account: Option<String>,
    #[fields(rename = "Amount")]
    amount: f64,
}

// Display an AssociatedLineItem (for use with table_iter)
fn display_line_item(value: &AssociatedLineItem) -> String {
    value.name.to_owned()
}

// Obtain a list of the periodic budget instances which are referenced (by
// date) by at least one of the transactions in the slice.
async fn get_referenced_budgets(
    transactions: &[TransactionRecord],
    db: &DatabaseConnection,
) -> Result<Vec<periodic_budgets::Model>, sea_orm::DbErr> {
    let dates = transactions
        .iter()
        .map(|transaction| {
            NaiveDate::parse_from_str(&transaction.date, "%m/%d/%y")
                .unwrap()
                .and_time(NaiveTime::default())
                .and_local_timezone(Local)
                .single()
                .unwrap()
                .into()
        })
        .collect::<Vec<DateTimeWithTimeZone>>();
    let min_date = dates.iter().min().unwrap();
    let max_date = dates.iter().max().unwrap();
    PeriodicBudgets::find()
        .filter(periodic_budgets::Column::StartDate.lte(*min_date))
        .filter(periodic_budgets::Column::EndDate.gte(*max_date))
        .all(db)
        .await
}

// Get a list of the line items which are referenced by at least one of the
// transactions in the list.
async fn get_referenced_line_items(
    transactions: &[TransactionRecord],
    budgets: &[periodic_budgets::Model],
    db: &DatabaseConnection,
) -> Result<Vec<line_item_instances::Model>, sea_orm::DbErr> {
    let budget_ids =
        budgets.iter().map(|budget| budget.id).collect::<Vec<i32>>();
    let summaries = transactions
        .iter()
        .map(|t| t.line_item.clone())
        .collect::<Vec<String>>();
    LineItemInstances::find()
        .filter(line_item_instances::Column::PeriodicBudget.is_in(budget_ids))
        .filter(line_item_instances::Column::Summary.is_in(summaries))
        .all(db)
        .await
}

// Convert a list of TransactionRecord to a list of AssociatedTransaction.
async fn associate(
    transactions: &[TransactionRecord],
    db: &DatabaseConnection,
) -> anyhow::Result<Vec<AssociatedTransaction>> {
    let budgets = get_referenced_budgets(transactions, db).await?;
    let line_items =
        get_referenced_line_items(transactions, &budgets, db).await?;

    let mut associated: Vec<AssociatedTransaction> = Vec::new();
    for transaction in transactions {
        let date: DateTimeWithTimeZone =
            NaiveDate::parse_from_str(&transaction.date, "%m/%d/%y")?
                .and_time(NaiveTime::default())
                .and_local_timezone(Local)
                .single()
                .unwrap()
                .into();

        let budget = budgets
            .iter()
            .find(|&b| b.start_date <= date && b.end_date >= date)
            .ok_or_else(|| {
                MissingBudgetError::new(date.format("%d %b %Y").to_string())
            })?;

        let line_item = line_items
            .iter()
            .find(|item| {
                item.summary == transaction.line_item
                    && item.periodic_budget == budget.id
            })
            .ok_or_else(|| {
                MissingInstanceError::new(
                    budget.id,
                    transaction.line_item.clone(),
                )
            })?;

        associated.push(AssociatedTransaction {
            summary: transaction.summary.clone(),
            line_item: AssociatedLineItem {
                name: transaction.line_item.clone(),
                instance_id: line_item.id,
            },
            date,
            from_account: transaction.from_account.clone(),
            to_account: transaction.to_account.clone(),
            amount: transaction.amount,
        });
    }

    Ok(associated)
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

        let id = Transactions::insert(transaction_model)
            .exec(db)
            .await?
            .last_insert_id;

        // Create the Real or Planned Transaction instances
        match transaction_type {
            TransactionType::Real => {
                let real_model = real_transactions::ActiveModel {
                    transaction: Set(id),
                    line_item_instance: Set(Some(
                        record.line_item.instance_id,
                    )),
                    ..Default::default()
                };
                RealTransactions::insert(real_model).exec(db).await?;
            }

            TransactionType::Planned => {
                let planned_model = planned_transactions::ActiveModel {
                    transaction: Set(id),
                    line_item_instance: Set(record.line_item.instance_id),
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
            import(filename, transaction_type, db).await
        }
        Verb::DeleteAll { budget } => match transaction_type {
            TransactionType::Planned => delete_all_planned(*budget, db).await,
            TransactionType::Real => todo!(),
        },
    }
}

///////////////////////////////////////////////////////////////////////////////
