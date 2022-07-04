///////////////////////////////////////////////////////////////////////////////
// NAME:            initial_balances.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic encapsulating InitialBalance model.
//
// CREATED:         07/04/2022
//
// LAST EDITED:     07/04/2022
////

use axum::{http::StatusCode, Json};
use budget_models::models;
use sea_orm::{DatabaseConnection, EntityTrait};
use tracing::{Level, event};

use crate::entities::prelude::*;

pub async fn list(db: DatabaseConnection) ->
    Result<Json<Vec<models::InitialBalance>>, StatusCode>
{
    let initial_balances = InitialBalances::find()
        .all(&db)
        .await
        .map_err(|e| {
            event!(Level::ERROR, "{:?}", &e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(
        initial_balances.into_iter()
            .map(|balance| balance.into())
            .collect()
    ))
}

///////////////////////////////////////////////////////////////////////////////
