///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Budget web service
//
// CREATED:         04/10/2022
//
// LAST EDITED:     11/16/2022
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

use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use budget_backend_lib::secret::SecretManager;
use clap::Parser;
use sea_orm::Database;
use serde::Deserialize;
use std::env;
use std::fmt;
use std::fs::File;
use tower_http::trace::TraceLayer;
use tracing::{event, Level};

mod endpoints;

///////////////////////////////////////////////////////////////////////////////
// internal_server_error Helper
////

pub(crate) fn internal_server_error<E: fmt::Debug>(e: E) -> StatusCode {
    event!(Level::ERROR, "{:?}", &e);
    StatusCode::INTERNAL_SERVER_ERROR
}

///////////////////////////////////////////////////////////////////////////////
// Configuration
////

#[derive(Deserialize)]
struct Configuration {
    // The root URL which the service resides at, '/' by default.
    pub root: Option<String>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            root: Some("/".to_string()),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Command Line Interface
////

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    /// Path to a JSON file containing database credentials
    #[clap(short, long, value_parser)]
    secret_file: String,

    /// Path to a YAML file containing application configuration
    #[clap(short, long, value_parser)]
    config_file: Option<String>,
}

///////////////////////////////////////////////////////////////////////////////
// Main
////

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("tower_http=debug,budget_service=trace")
        .init();

    let args = Args::parse();

    let url_template =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let secret_manager = SecretManager::new(args.secret_file);
    let url = secret_manager.with_url(url_template.parse()?)?;

    let configuration: Configuration = match args.config_file {
        Some(file) => serde_yaml::from_reader(File::open(file)?)?,
        None => Configuration::default(),
    };

    // Open a connection to the database for SeaOrm.
    let connection = Database::connect(&url).await?;

    let app = Router::new()
        .route(
            "/api/accounts",
            get({
                let db = connection.clone();
                move || endpoints::accounts::list(db)
            }),
        )
        .route(
            "/api/transactions",
            post({
                let db = connection.clone();
                move |transaction| {
                    endpoints::transactions::create(transaction, db)
                }
            }),
        )
        .layer(TraceLayer::new_for_http());

    let root = configuration.root.as_ref().unwrap();
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(Router::new().nest(&root, app).into_make_service())
        .await?;
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
