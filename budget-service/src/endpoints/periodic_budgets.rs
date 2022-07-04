///////////////////////////////////////////////////////////////////////////////
// NAME:            periodic_budgets.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic encapsulating exposing PeriodicBudgets over HTTP.
//
// CREATED:         07/04/2022
//
// LAST EDITED:     07/04/2022
////

use axum::{http::StatusCode, Json};
use budget_models::models;
use sea_orm::{DatabaseConnection, EntityTrait};
use tracing::{Level, event};

use crate::entities::*;
use crate::entities::prelude::*;

pub async fn list(db: DatabaseConnection) ->
    Result<Json<Vec<models::PeriodicBudget>>, StatusCode>
{
    let budgets: Vec<periodic_budgets::Model> = PeriodicBudgets::find()
        .all(&db)
        .await
        .map_err(|e| {
            event!(Level::ERROR, "{:?}", &e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(
        budgets.into_iter()
            .map(|budget| budget.into())
            .collect()
    ))
}

///////////////////////////////////////////////////////////////////////////////
