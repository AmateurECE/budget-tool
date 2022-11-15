///////////////////////////////////////////////////////////////////////////////
// NAME:            secret.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Secret manager for the budget service.
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

use axum::http::Uri;
use serde::Deserialize;
use std::fs::File;

#[derive(Deserialize)]
struct DatabaseUserCredentials {
    pub username: String,
    pub password: String,
}

pub struct SecretManager {
    file_path: String,
}

impl SecretManager {
    pub fn new(file_path: String) -> Self {
        SecretManager { file_path }
    }

    pub fn with_url(&self, uri: Uri) -> anyhow::Result<String> {
        let secret: DatabaseUserCredentials =
            serde_json::from_reader(File::open(&self.file_path)?)?;
        let authority = uri
            .authority()
            .map(|a| {
                a.host().to_owned()
                    + &a.port()
                        .map(|p| ":".to_string() + p.as_str())
                        .unwrap_or("".to_string())
            })
            .expect("uri should contain an authority");
        let secret_authority = format!(
            "{}:{}@{}",
            &secret.username, &secret.password, &authority
        );
        Ok(Uri::builder()
            .scheme(uri.scheme().map(|a| a.as_str()).unwrap_or(""))
            .authority(secret_authority.as_str())
            .path_and_query(
                uri.path_and_query().map(|p| p.as_str()).unwrap_or(""),
            )
            .build()
            .unwrap()
            .to_string())
    }
}

///////////////////////////////////////////////////////////////////////////////
