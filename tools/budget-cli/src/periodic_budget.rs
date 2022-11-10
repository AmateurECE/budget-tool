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
use sea_orm::prelude::*;
use sea_orm::DatabaseConnection;

use crate::table;
use crate::Verb;

async fn list(db: &DatabaseConnection) -> anyhow::Result<()> {
    let budgets = PeriodicBudgets::find()
        .all(db)
        .await?
        .iter()
        .map(|budget| {
            vec![
                budget.id.to_string(),
                budget.start_date.to_string(),
                budget.end_date.to_string(),
            ]
        })
        .collect::<Vec<Vec<String>>>();

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
