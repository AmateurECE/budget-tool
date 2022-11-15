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
