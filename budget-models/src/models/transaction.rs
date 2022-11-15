///////////////////////////////////////////////////////////////////////////////
// NAME:            transaction.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     A transaction.
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

#[derive(Clone, Default, Deserialize, PartialEq, Serialize)]
pub struct Transaction {
    pub id: i32,
    pub summary: String,
    pub from_account: Option<String>,
    pub to_account: Option<String>,
    pub amount: i64,

    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewTransaction {
    pub summary: String,
    pub from_account: Option<String>,
    pub to_account: Option<String>,
    pub amount: i64,

    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub date: DateTime<Utc>,
}

///////////////////////////////////////////////////////////////////////////////
