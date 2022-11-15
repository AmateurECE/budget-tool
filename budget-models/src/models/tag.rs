///////////////////////////////////////////////////////////////////////////////
// NAME:            tag.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     A tag for a transaction (provides an additional method of
//                  searching/indexing).
//
// CREATED:         09/17/2022
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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub id: i32,
    pub tag: String,
}

///////////////////////////////////////////////////////////////////////////////
