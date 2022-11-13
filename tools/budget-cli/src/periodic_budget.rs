///////////////////////////////////////////////////////////////////////////////
// NAME:            periodic_budget.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Operations done on the set of periodic budgets.
//
// CREATED:         11/10/2022
//
// LAST EDITED:     11/12/2022
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
