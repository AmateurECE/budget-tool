///////////////////////////////////////////////////////////////////////////////
// NAME:            initial_balance.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Balance snapshot of an account.
//
// CREATED:         07/05/2022
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

use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InitialBalance {
    pub id: i32,
    pub account: String,
    pub budget: i32,
    pub balance: i64,

    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub last_updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct NewInitialBalance {
    pub account: String,
    pub budget: i32,
    pub balance: i64,
}

///////////////////////////////////////////////////////////////////////////////
