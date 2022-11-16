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
use budget_backend_lib::prelude::*;
use budget_models::models;
use sea_orm::{DatabaseConnection, EntityTrait};

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
