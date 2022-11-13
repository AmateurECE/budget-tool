///////////////////////////////////////////////////////////////////////////////
// NAME:            line_item_instance.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Operations on line_item_instances in the database.
//
// CREATED:         11/11/2022
//
// LAST EDITED:     11/12/2022
////

use budget_backend_lib::prelude::*;
use budget_models::Money;
use clap::Subcommand;
use csv;
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, Set};
use serde::Deserialize;
use std::fs::File;
use table_iter::prelude::*;

use crate::table;

#[derive(Clone, Debug, Deserialize, Fields, FieldNames)]
struct LineItemInstanceRecord {
    #[fields(rename = "Summary")]
    #[serde(rename = "Summary")]
    summary: String,
    #[fields(rename = "From Account", with = "table_iter::display::option")]
    #[serde(rename = "From Account")]
    from_account: Option<String>,
    #[fields(rename = "To Account", with = "table_iter::display::option")]
    #[serde(rename = "To Account")]
    to_account: Option<String>,
    #[fields(rename = "Amount")]
    #[serde(rename = "Amount")]
    amount: f64,
}

async fn import(
    filename: &str,
    budget: i32,
    db: &DatabaseConnection,
) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_reader(File::open(&filename)?);
    let mut imported: Vec<LineItemInstanceRecord> = Vec::new();
    for result in reader.deserialize() {
        let record: LineItemInstanceRecord = result?;
        imported.push(record.clone());
        let model = line_item_instances::ActiveModel {
            summary: Set(record.summary),
            from_account: Set(record.from_account),
            to_account: Set(record.to_account),
            amount: Set(Money::from(record.amount).into()),
            periodic_budget: Set(budget),
            ..Default::default()
        };
        LineItemInstances::insert(model).exec(db).await?;
    }

    println!(
        "Imported {} line items for budged {}",
        imported.len(),
        budget
    );
    table::print(&imported);
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
// Public Interface
////

#[derive(Subcommand)]
pub(crate) enum Verb {
    /// Import line item instances from a CSV file
    Import {
        /// The file to import from
        #[clap(value_parser)]
        filename: String,

        /// The budget ID to apply the line item instances to
        #[clap(value_parser)]
        budget: i32,
    },
}

pub(crate) async fn op(
    verb: &Verb,
    db: &DatabaseConnection,
) -> anyhow::Result<()> {
    match verb {
        Verb::Import { filename, budget } => {
            import(filename, *budget, db).await
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
