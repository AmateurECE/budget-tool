///////////////////////////////////////////////////////////////////////////////
// NAME:            periodic_budget.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Operations done on the set of periodic budgets.
//
// CREATED:         11/10/2022
//
// LAST EDITED:     11/10/2022
////

use budget_backend_lib::prelude::*;
use futures::future;
use sea_orm::prelude::*;
use sea_orm::DatabaseConnection;

use crate::table;
use crate::Verb;

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
                vec![
                    budget.id.to_string(),
                    budget.start_date.format("%d %b %Y").to_string(),
                    budget.end_date.format("%d %b %Y").to_string(),
                    line_items.len().to_string(),
                    number_of_planned_transactions.to_string(),
                ]
            })
            .collect::<Vec<_>>(),
    )
    .await;

    table::print(
        budgets
            .iter()
            .map(|row| row.as_slice())
            .collect::<Vec<&[String]>>()
            .as_slice(),
        vec![
            "Id".to_string(),
            "Start Date".to_string(),
            "End Date".to_string(),
            "Line Items".to_string(),
            "Planned Transactions".to_string(),
        ]
        .as_slice(),
    );
    Ok(())
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
