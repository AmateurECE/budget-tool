///////////////////////////////////////////////////////////////////////////////
// NAME:            periodic_budget.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Operations done on the set of periodic budgets.
//
// CREATED:         11/10/2022
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
use clap::Subcommand;
use futures::future;
use sea_orm::prelude::*;
use sea_orm::DatabaseConnection;
use table_iter::prelude::*;

use crate::table;

#[derive(Fields, FieldNames)]
struct PeriodicBudgetRecord {
    #[fields(rename = "Id")]
    id: i32,
    #[fields(rename = "Start Date", with = "crate::display::date")]
    start_date: DateTimeWithTimeZone,
    #[fields(rename = "End Date", with = "crate::display::date")]
    end_date: DateTimeWithTimeZone,
    #[fields(rename = "Line Items")]
    line_items: usize,
    #[fields(rename = "Planned Transactions")]
    planned_transactions: usize,
}

async fn list(db: &DatabaseConnection) -> anyhow::Result<()> {
    let budgets = future::join_all(
        PeriodicBudgets::find()
            .all(db)
            .await?
            .iter()
            .map(|budget| async {
                let line_items = budget
                    .find_related(LineItemInstances)
                    .all(db)
                    .await
                    .unwrap();
                let number_of_planned_transactions: usize = future::join_all(
                    line_items.iter().map(|line_item| async {
                        line_item
                            .find_related(PlannedTransactions)
                            .all(db)
                            .await
                            .unwrap()
                            .len()
                    }),
                )
                .await
                .iter()
                .sum();
                PeriodicBudgetRecord {
                    id: budget.id,
                    start_date: budget.start_date,
                    end_date: budget.end_date,
                    line_items: line_items.len(),
                    planned_transactions: number_of_planned_transactions,
                }
            })
            .collect::<Vec<_>>(),
    )
    .await;

    table::print(&budgets);
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
// Public Interface
////

#[derive(Subcommand)]
pub(crate) enum Verb {
    /// List periodic budgets in the database
    List,
}

pub(crate) async fn op(
    verb: &Verb,
    db: &DatabaseConnection,
) -> anyhow::Result<()> {
    match &verb {
        Verb::List => list(db).await,
    }
}

///////////////////////////////////////////////////////////////////////////////
