///////////////////////////////////////////////////////////////////////////////
// NAME:            periodic_budgets.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic encapsulating exposing PeriodicBudgets over HTTP.
//
// CREATED:         07/04/2022
//
// LAST EDITED:     07/05/2022
////

use axum::{extract::Path, http::StatusCode, Json};
use budget_models::models;
use sea_orm::{DatabaseConnection, EntityTrait};
use tracing::{Level, event};

use crate::entities::*;
use crate::entities::prelude::*;

pub async fn list(db: DatabaseConnection) ->
    Result<Json<Vec<models::PeriodicBudget>>, StatusCode>
{
    let budgets: Vec<periodic_budgets::Model> = PeriodicBudgets::find()
        .all(&db)
        .await
        .map_err(|e| {
            event!(Level::ERROR, "{:?}", &e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(
        budgets.into_iter()
            .map(|budget| budget.into())
            .collect()
    ))
}

pub async fn detailed(Path(id): Path<i32>, db: DatabaseConnection) ->
    Result<Json<models::PeriodicBudgetSummary>, StatusCode>
{
    // Get the budget requested by `id'
    let budget = PeriodicBudgets::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| {
            event!(Level::ERROR, "{:?}", &e);
            StatusCode::NOT_FOUND
        })?;

    // Locate all the budget items for this budget
    // let items: Result<Vec<BudgetItem>, _> = budget_items::dsl::budget_items
    //     .filter(budget_items::periodic_budget.eq(budget.id))
    //     .load::<BudgetItem>(&*db);
    // if let Err::<Vec<BudgetItem>, _>(e) = items {
    //     event!(Level::ERROR, "{}", e);
    //     return Err(StatusCode::INTERNAL_SERVER_ERROR);
    // }

    // let items = items.unwrap();

    // Collect all initial balances for the time period corresponding to this
    // budget.
    // let initial_balances: Result<Vec<InitialBalance>, _> =
    //     initial_balances::dsl::initial_balances
    //     .filter(initial_balances::budget.eq(budget.id))
    //     .load::<InitialBalance>(&*db);
    // if let Err::<Vec<InitialBalance>, _>(e) = initial_balances {
    //     event!(Level::ERROR, "{}", e);
    //     return Err(StatusCode::INTERNAL_SERVER_ERROR);
    // }
    // let initial_balances = initial_balances.unwrap();

    // Collect all the transactions for the time period corresponding to this
    // budget.
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
