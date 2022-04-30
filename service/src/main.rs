///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Budget web service
//
// CREATED:         04/10/2022
//
// LAST EDITED:     04/29/2022
////

use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

use diesel::prelude::*;
use diesel::{pg::PgConnection, result::Error::NotFound};
use dotenv::dotenv;
use tower_http::trace::TraceLayer;
use tracing_subscriber;
use tracing::{event, Level};

use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Router, Json,
};

use budget_models::{
    models::{
        Account, accounts,
        BudgetItem, budget_items,
        InitialBalance, NewInitialBalance, initial_balances,
        PeriodicBudget, periodic_budgets,
    },
    entities::PeriodicBudgetEndpoint,
};

async fn list_budgets(db: Arc<Mutex<PgConnection>>) ->
    Result<Json<Vec<PeriodicBudget>>, StatusCode>
{
    let db = db.lock().unwrap();
    let budgets = periodic_budgets::dsl::periodic_budgets
        .load::<PeriodicBudget>(&*db);
    if let Err::<Vec<PeriodicBudget>, _>(_) = budgets {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(budgets.unwrap()))
}

async fn list_accounts(db: Arc<Mutex<PgConnection>>) ->
    Result<Json<Vec<Account>>, StatusCode>
{
    let db = db.lock().unwrap();
    let accounts = accounts::dsl::accounts.load::<Account>(&*db);
    if let Err::<Vec<Account>, _>(_) = accounts {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(accounts.unwrap()))
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
    } else if let Err::<PeriodicBudget, _>(e) = budget {
        event!(Level::ERROR, "{}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let budget = budget.unwrap();

    // Locate all the budget items for this budget
    let items: Result<Vec<BudgetItem>, _> = budget_items::dsl::budget_items
        .filter(budget_items::periodic_budget.eq(budget.id))
        .load::<BudgetItem>(&*db);
    if let Err::<Vec<BudgetItem>, _>(e) = items {
        event!(Level::ERROR, "{}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let mut items = items.unwrap();

    // Collect all the categories listed in all budget items, then list the
    // budget items by category.
    let mut categories = items.iter().map(|item| item.category.to_owned())
        .collect::<Vec<String>>();
    categories.sort();
    categories.dedup();
    let items = categories.iter().map(|cat| {
        // Filter out the budget items that map to this category
        let filtered = items.iter().filter_map(|item| {
            if item.category == *cat {
                Some(item.clone())
            } else {
                None
            }
        }).collect::<Vec<BudgetItem>>();
        items.retain(|item| item.category != *cat);

        (cat.to_owned(), filtered)
    }).collect::<HashMap<String, Vec<BudgetItem>>>();

    // Collect all initial balances for the time period corresponding to this
    // budget.
    let initial_balances: Result<Vec<InitialBalance>, _> =
        initial_balances::dsl::initial_balances
        .filter(initial_balances::budget.eq(budget.id))
        .load::<InitialBalance>(&*db);
    if let Err::<Vec<InitialBalance>, _>(e) = initial_balances {
        event!(Level::ERROR, "{}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let initial_balances = initial_balances.unwrap();

    Ok(Json(PeriodicBudgetEndpoint {
        budget, items, initial_balances,
    }))
}

async fn post_initial_balance(
    Json(initial_balance): Json<NewInitialBalance>,
    db: Arc<Mutex<PgConnection>>
) ->
    Result<Json<InitialBalance>, StatusCode>
{
    let db = db.lock().unwrap();
    let initial_balance = diesel::insert_into(initial_balances::table)
        .values(&initial_balance)
        .get_result(&*db);
    if let Err::<InitialBalance, _>(_) = initial_balance {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(initial_balance.unwrap()))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = Arc::new(Mutex::new(
        PgConnection::establish(&url)
            .expect(&format!("Error connecting to {}", url))
    ));

    let app = Router::new()
        .route("/api/periodic_budgets",
               get({ let db = connection.clone(); move || list_budgets(db) })
        )
        .route("/api/periodic_budgets/:id",
               get({
                   let db = connection.clone();
                   move |id| detailed_budget(id, db)
               })
        )
        .route("/api/initial_balances",
               post({
                   let db = connection.clone();
                   move |balance| post_initial_balance(balance, db)
               })
        )
        .route("/api/accounts",
               get({ let db = connection.clone(); move || list_accounts(db) })
        )
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

///////////////////////////////////////////////////////////////////////////////
