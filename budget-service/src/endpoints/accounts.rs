///////////////////////////////////////////////////////////////////////////////
// NAME:            accounts.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic encapsulating Accounts endpoints.
//
// CREATED:         07/04/2022
//
// LAST EDITED:     11/15/2022
////

use axum::{http::StatusCode, Json};
use budget_models::models;
use sea_orm::{DatabaseConnection, EntityTrait};
use budget_backend_lib::prelude::*;

use crate::internal_server_error;

pub async fn list(
    db: DatabaseConnection,
) -> Result<Json<Vec<models::Account>>, StatusCode> {
    let accounts: Vec<accounts::Model> = Accounts::find()
        .all(&db)
        .await
        .map_err(internal_server_error)?;
    Ok(Json(
        accounts.into_iter().map(|account| account.into()).collect(),
    ))
}

///////////////////////////////////////////////////////////////////////////////
