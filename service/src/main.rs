///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Budget web service
//
// CREATED:         04/10/2022
//
// LAST EDITED:     04/25/2022
////

use std::env;
use std::sync::{Arc, Mutex};

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

use axum::{
    extract::Path,
    http::StatusCode,
    routing::get,
    Router, Json,
};

use budget_models::{
    models::{PeriodicBudget, periodic_budgets},
    entities::PeriodicBudgetEndpoint,
};

async fn list_accounts(db: Arc<Mutex<PgConnection>>) ->
    Json<Vec<PeriodicBudget>>
{
    let db = db.lock().unwrap();
    Json(periodic_budgets::dsl::periodic_budgets.load::<PeriodicBudget>(&*db)
         .expect("Error loading periodic_budgets from database!"))
}

async fn detailed_budget(Path(id): Path<i32>, db: Arc<Mutex<PgConnection>>) ->
    Result<Json<PeriodicBudgetEndpoint>, StatusCode>
{
    // let db = db.lock().unwrap();
    Err(StatusCode::NOT_FOUND)
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
        )
        .route("/api/periodic_budgets/:id",
               get({
                   let db = connection.clone();
                   move |id| detailed_budget(id, db)
               })
        );

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

///////////////////////////////////////////////////////////////////////////////
