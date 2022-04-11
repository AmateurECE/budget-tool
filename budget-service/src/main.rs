///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Budget web service
//
// CREATED:         04/10/2022
//
// LAST EDITED:     04/10/2022
////

use std::env;

use axum::{routing::get, Router, Json};
use budget_models::{Database, account};
use budget_models::models::{NewAccount, Account};
use dotenv::dotenv;

pub fn default_db() -> Database {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    Database::connect(database_url)
}

async fn list_accounts() -> Json<Vec<Account>> {
    let db = default_db();
    Json(account::list(&db))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/accounts",
               get(list_accounts)
        );

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

///////////////////////////////////////////////////////////////////////////////
