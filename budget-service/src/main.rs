///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Budget web service
//
// CREATED:         04/10/2022
//
// LAST EDITED:     07/06/2022
////

use std::env;
use std::fmt;
use axum::{http::StatusCode, routing::{get, post}, Router};
use sea_orm::Database;
use tower_http::trace::TraceLayer;
use tracing::{Level, event};

mod conversions;
mod endpoints;
mod entities;

///////////////////////////////////////////////////////////////////////////////
// internal_server_error Helper
////

pub(crate) fn internal_server_error<E: fmt::Debug>(e: E) -> StatusCode {
    event!(Level::ERROR, "{:?}", &e);
    StatusCode::INTERNAL_SERVER_ERROR
}

///////////////////////////////////////////////////////////////////////////////
// Main
////

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("tower_http=debug,budget_service=trace")
        .init();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = Database::connect(&url).await?;

    let app = Router::new()
        .route("/api/periodic_budgets",
               get({
                   let db = connection.clone();
                   move || endpoints::periodic_budgets::list(db)
               })
        )

        .route("/api/periodic_budgets/:id",
               get({
                   let db = connection.clone();
                   move |id| endpoints::periodic_budgets::detailed(id, db)
               })
        )

        .route("/api/initial_balances",
               get({
                   let db = connection.clone();
                   move || endpoints::initial_balances::list(db)
               })
        )

        .route("/api/accounts",
               get({
                   let db = connection.clone();
                   move || endpoints::accounts::list(db)
               })
        )

        .route("/api/transactions",
               post({
                   let db = connection.clone();
                   move |transaction| endpoints::transactions::create(
                       transaction, db)
               })
        )
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
