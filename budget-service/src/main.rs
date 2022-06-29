///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Budget web service
//
// CREATED:         04/10/2022
//
// LAST EDITED:     06/29/2022
////

use std::env;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing::{event, Level};

use axum::{
    extract::Path, http::StatusCode, routing::{get, post}, Router, Json,
};

use budget_models::{
    models::{
        Account, InitialBalance, NewInitialBalance, PeriodicBudget,
        NewTransaction, Transaction,
    },
    entities::PeriodicBudgetEndpoint,
};

///////////////////////////////////////////////////////////////////////////////
// Accounts Endpoints
////

async fn list_accounts(db: PgPool) ->
    Result<Json<Vec<Account>>, StatusCode>
{
    let result = sqlx::query_as!(
        Account,
        r#"SELECT name, account_type as "account_type: _" FROM accounts"#
    )
        .fetch_all(&db)
        .await
        .map_err(|e| {
            event!(Level::ERROR, "{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(result))
}

///////////////////////////////////////////////////////////////////////////////
// Budget Endpoints
////

async fn list_budgets(_db: PgPool) ->
    Result<Json<Vec<PeriodicBudget>>, StatusCode>
{
    // let db = db.lock().unwrap();
    // let budgets = periodic_budgets::dsl::periodic_budgets
    //     .load::<PeriodicBudget>(&*db);
    // if let Err::<Vec<PeriodicBudget>, _>(_) = budgets {
    //     return Err(StatusCode::INTERNAL_SERVER_ERROR);
    // }

    // Ok(Json(budgets.unwrap()))
    todo!()
}

async fn detailed_budget(Path(_id): Path<i32>, _db: PgPool) ->
    Result<Json<PeriodicBudgetEndpoint>, StatusCode>
{
    // // Lock the database connection Mutex
    // let db = db.lock().unwrap();

    // // Get the budget requested by `id'
    // let budget: Result<PeriodicBudget, _> =
    //     periodic_budgets::dsl::periodic_budgets
    //     .find(id)
    //     .first(&*db);
    // if let Err::<PeriodicBudget, _>(NotFound) = budget {
    //     return Err(StatusCode::NOT_FOUND);
    // } else if let Err::<PeriodicBudget, _>(e) = budget {
    //     event!(Level::ERROR, "{}", e);
    //     return Err(StatusCode::INTERNAL_SERVER_ERROR);
    // }

    // let budget = budget.unwrap();

    // // Locate all the budget items for this budget
    // let items: Result<Vec<BudgetItem>, _> = budget_items::dsl::budget_items
    //     .filter(budget_items::periodic_budget.eq(budget.id))
    //     .load::<BudgetItem>(&*db);
    // if let Err::<Vec<BudgetItem>, _>(e) = items {
    //     event!(Level::ERROR, "{}", e);
    //     return Err(StatusCode::INTERNAL_SERVER_ERROR);
    // }

    // let items = items.unwrap();

    // // Collect all initial balances for the time period corresponding to this
    // // budget.
    // let initial_balances: Result<Vec<InitialBalance>, _> =
    //     initial_balances::dsl::initial_balances
    //     .filter(initial_balances::budget.eq(budget.id))
    //     .load::<InitialBalance>(&*db);
    // if let Err::<Vec<InitialBalance>, _>(e) = initial_balances {
    //     event!(Level::ERROR, "{}", e);
    //     return Err(StatusCode::INTERNAL_SERVER_ERROR);
    // }
    // let initial_balances = initial_balances.unwrap();

    // // Collect all the transactions for the time period corresponding to this
    // // budget.
    // let transactions: Result<Vec<Transaction>, _> =
    //     transactions::dsl::transactions
    //     .filter(transactions::periodic_budget.eq(budget.id))
    //     .load::<Transaction>(&*db);
    // if let Err::<Vec<Transaction>, _>(e) = transactions {
    //     event!(Level::ERROR, "{}", e);
    //     return Err(StatusCode::INTERNAL_SERVER_ERROR);
    // }
    // let transactions = transactions.unwrap();

    // Ok(Json(PeriodicBudgetEndpoint {
    //     budget, items, initial_balances, transactions,
    // }))
    todo!()
}

///////////////////////////////////////////////////////////////////////////////
// Initial Balance Endpoints
////

async fn list_initial_balances(_db: PgPool) ->
    Result<Json<Vec<InitialBalance>>, StatusCode>
{
    // let db = db.lock().unwrap();
    // let initial_balances = initial_balances::dsl::initial_balances
    //     .load::<InitialBalance>(&*db);
    // if let Err::<Vec<InitialBalance>, _>(e) = initial_balances {
    //     event!(Level::ERROR, "{}", e);
    //     return Err(StatusCode::INTERNAL_SERVER_ERROR);
    // }
    // Ok(Json(initial_balances.unwrap()))
    todo!()
}

async fn post_initial_balance(
    Json(_initial_balance): Json<NewInitialBalance>,
    _db: PgPool,
) ->
    Result<Json<InitialBalance>, StatusCode>
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
    Json(_new_transaction): Json<NewTransaction>,
    _db: PgPool,
) ->
    Result<Json<Transaction>, StatusCode>
{
    // let db = db.lock().unwrap();
    // event!(Level::INFO, "{:?}", &new_transaction);
    // let transaction = diesel::insert_into(transactions::table)
    //     .values(&new_transaction)
    //     .get_result(&*db);
    // if let Err::<Transaction, _>(e) = transaction {
    //     event!(Level::ERROR, "{}", e);
    //     return Err(StatusCode::INTERNAL_SERVER_ERROR);
    // }

    // Ok(Json(transaction.unwrap()))
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
    let connection = PgPool::connect(&url).await?;

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
               get({
                   let db = connection.clone();
                   move || list_initial_balances(db)
               })
               .post({
                   let db = connection.clone();
                   move |balance| post_initial_balance(balance, db)
               })
        )
        .route("/api/accounts",
               get({ let db = connection.clone(); move || list_accounts(db) })
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
