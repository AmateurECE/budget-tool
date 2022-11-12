///////////////////////////////////////////////////////////////////////////////
// NAME:            line_item_instance.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Operations on line_item_instances in the database.
//
// CREATED:         11/11/2022
//
// LAST EDITED:     11/11/2022
////

use budget_backend_lib::prelude::*;
use budget_models::Money;
use clap::Subcommand;
use csv;
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, Set};
use serde::Deserialize;
use std::fs::File;

use crate::table;

#[derive(Clone, Debug, Deserialize)]
struct LineItemInstanceRecord {
    #[serde(rename = "Summary")]
    summary: String,
    #[serde(rename = "From Account")]
    from_account: Option<String>,
    #[serde(rename = "To Account")]
    to_account: Option<String>,
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
            ..Default::default()
        };
        LineItemInstances::insert(model).exec(db).await?;
    }

    let table_items = imported
        .iter()
        .map(|model| {
            vec![
                model.summary.to_owned(),
                model
                    .from_account
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("")
                    .to_owned(),
                model
                    .to_account
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("")
                    .to_owned(),
                model.amount.to_string(),
            ]
        })
        .collect::<Vec<Vec<String>>>();
    println!(
        "Imported {} transactions for budged {}",
        imported.len(),
        budget
    );
    let headers = vec![
        "Summary".to_string(),
        "From Account".to_string(),
        "To Account".to_string(),
        "Amount".to_string(),
    ];
    table::print(
        &table_items
            .iter()
            .map(|item| item.as_slice())
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
