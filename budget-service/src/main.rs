///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Budget web service
//
// CREATED:         04/10/2022
//
// LAST EDITED:     07/05/2022
////

use std::env;
use std::fmt;
use axum::{http::StatusCode, routing::{get, post}, Router, Json};
use budget_models::models;
use sea_orm::{Database, DatabaseConnection};
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
// Initial Balance Endpoints
////

async fn post_initial_balance(
    Json(_initial_balance): Json<models::NewInitialBalance>,
    _db: DatabaseConnection,
) ->
    Result<Json<models::InitialBalance>, StatusCode>
{
    // let db = db.lock().unwrap();
    // let initial_balance = diesel::insert_into(initial_balances::table)
    //     .values(&initial_balance)
    //     .get_result(&*db);
    // if let Err::<InitialBalance, _>(_) = initial_balance {
    //     return Err(StatusCode::INTERNAL_SERVER_ERROR);
    // }

    // Ok(Json(initial_balance.unwrap()))
    todo!()
}

///////////////////////////////////////////////////////////////////////////////
// Transactions
////

async fn create_transaction (
    Json(_new_transaction): Json<models::NewTransaction>,
    _db: DatabaseConnection,
) ->
    Result<Json<models::Transaction>, StatusCode>
{
    todo!()
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
               .post({
                   let db = connection.clone();
                   move |balance| post_initial_balance(balance, db)
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
                   move |transaction| create_transaction(transaction, db)
               })
        )
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
