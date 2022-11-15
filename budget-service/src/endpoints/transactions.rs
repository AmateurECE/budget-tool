///////////////////////////////////////////////////////////////////////////////
// NAME:            transactions.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic for CRUD operations on the transactions table.
//
// CREATED:         07/06/2022
//
// LAST EDITED:     11/15/2022
////

use axum::{http::StatusCode, Json};
use budget_models::models;
use sea_orm::{DatabaseConnection, EntityTrait};
use budget_backend_lib::prelude::*;

use crate::internal_server_error;

pub async fn create(
    Json(new_transaction): Json<models::NewTransaction>,
    db: DatabaseConnection,
) -> Result<Json<models::Transaction>, StatusCode> {
    let active_model: transactions::ActiveModel =
        new_transaction.try_into().map_err(internal_server_error)?;
    Ok(Json(
        Transactions::insert(active_model)
            .exec_with_returning(&db)
            .await
            .map_err(internal_server_error)?
            .try_into()
            .map_err(internal_server_error)?,
    ))
}

///////////////////////////////////////////////////////////////////////////////
