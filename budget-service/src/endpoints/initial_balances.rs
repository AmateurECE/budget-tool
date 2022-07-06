///////////////////////////////////////////////////////////////////////////////
// NAME:            initial_balances.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic encapsulating InitialBalance model.
//
// CREATED:         07/04/2022
//
// LAST EDITED:     07/05/2022
////

use axum::{http::StatusCode, Json};
use budget_models::models;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entities::prelude::*;
use crate::internal_server_error;

pub async fn list(db: DatabaseConnection) ->
    Result<Json<Vec<models::InitialBalance>>, StatusCode>
{
    let initial_balances = InitialBalances::find()
        .all(&db)
        .await
        .map_err(internal_server_error)?;
    Ok(Json(
        initial_balances.into_iter()
            .map(|balance| balance.into())
            .collect()
    ))
}

///////////////////////////////////////////////////////////////////////////////
