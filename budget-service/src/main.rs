///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Budget web service
//
// CREATED:         04/10/2022
//
// LAST EDITED:     11/15/2022
////

use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use budget_backend_lib::secret::SecretManager;
use clap::Parser;
use sea_orm::Database;
use std::env;
use std::fmt;
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
// Command Line Interface
////

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    /// Path to a JSON file containing database credentials
    #[clap(short, long, value_parser)]
    secret_file: String,
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

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
