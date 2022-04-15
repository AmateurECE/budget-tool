///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Budget web service
//
// CREATED:         04/10/2022
//
// LAST EDITED:     04/14/2022
////

use std::env;
use std::sync::{Arc, Mutex};

use axum::{routing::get, Router, Json};
use budget_models::models::{PeriodicBudget, periodic_budgets};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

async fn list_accounts(db: Arc<Mutex<PgConnection>>) ->
    Json<Vec<PeriodicBudget>>
{
    let db = db.lock().unwrap();
    Json(periodic_budgets::dsl::periodic_budgets.load::<PeriodicBudget>(&*db)
         .expect("Error loading periodic_budgets from database!"))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = Arc::new(Mutex::new(
        PgConnection::establish(&url)
            .expect(&format!("Error connecting to {}", url))
    ));

    let app = Router::new()
        .route("/api/periodic_budgets",
               get({ let db = connection.clone(); move || list_accounts(db)})
        );

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

///////////////////////////////////////////////////////////////////////////////
