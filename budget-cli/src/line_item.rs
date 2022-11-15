///////////////////////////////////////////////////////////////////////////////
// NAME:            line_item.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Operations on line items in the database.
//
// CREATED:         11/11/2022
//
// LAST EDITED:     11/15/2022
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

use budget_backend_lib::prelude::*;
use budget_models::Money;
use clap::Subcommand;
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

async fn import_instance(
    filename: &str,
    budget: i32,
    db: &DatabaseConnection,
) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_reader(File::open(filename)?);
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

async fn delete_all(
    budget: i32,
    db: &DatabaseConnection,
) -> anyhow::Result<()> {
    line_item_instances::Entity::delete_many()
        .filter(line_item_instances::Column::PeriodicBudget.eq(budget))
        .exec(db)
        .await?;
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
// Public Interface
////

#[derive(Subcommand)]
pub(crate) enum Verb {
    /// Import line item instances from a CSV file
    ImportInstance {
        /// The file to import from
        #[clap(value_parser)]
        filename: String,

        /// The budget ID to apply the line item instances to
        #[clap(value_parser)]
        budget: i32,
    },

    /// Delete all transactions belonging to a budget
    DeleteAll {
        /// The budget ID
        #[clap(value_parser)]
        budget: i32,
    },
}

pub(crate) async fn op(
    verb: &Verb,
    db: &DatabaseConnection,
) -> anyhow::Result<()> {
    match verb {
        Verb::ImportInstance { filename, budget } => {
            import_instance(filename, *budget, db).await
        }
        Verb::DeleteAll { budget } => delete_all(*budget, db).await,
    }
}

///////////////////////////////////////////////////////////////////////////////
