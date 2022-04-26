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
use diesel::{pg::PgConnection, result::Error::NotFound};
use dotenv::dotenv;

use axum::{
    extract::Path,
    http::StatusCode,
    routing::get,
    Router, Json,
};

use budget_models::{
    models::{
        PeriodicBudget, periodic_budgets,
        BudgetItem, budget_items,
    },
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
    // Lock the database connection Mutex
    let db = db.lock().unwrap();

    // Get the budget requested by `id'
    let budget: Result<PeriodicBudget, _> =
        periodic_budgets::dsl::periodic_budgets
        .find(id)
        .first(&*db);
    if let Err::<PeriodicBudget, _>(NotFound) = budget {
        return Err(StatusCode::NOT_FOUND);
    } else if let Err::<PeriodicBudget, _>(_) = budget {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let budget = budget.unwrap();

    // Locate all the budget items for this budget
    let items: Result<Vec<BudgetItem>, _> = budget_items::dsl::budget_items
        .filter(budget_items::periodic_budget.eq(budget.id))
        .load::<BudgetItem>(&*db);
    if let Err::<Vec<BudgetItem>, _>(_) = items {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Collect all the categories listed in all budget items, then list the
    // budget items by category.
    todo!()
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
