///////////////////////////////////////////////////////////////////////////////
// NAME:            periodic_budgets.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic encapsulating exposing PeriodicBudgets over HTTP.
//
// CREATED:         07/04/2022
//
// LAST EDITED:     07/06/2022
////

use axum::{extract::Path, http::StatusCode, Json};
use budget_models::models;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};

use crate::entities::prelude::*;
use crate::entities::*;
use crate::internal_server_error;

pub async fn list(
    db: DatabaseConnection,
) -> Result<Json<Vec<models::PeriodicBudget>>, StatusCode> {
    let budgets: Vec<periodic_budgets::Model> = PeriodicBudgets::find()
        .all(&db)
        .await
        .map_err(internal_server_error)?;
    Ok(Json(
        budgets.into_iter().map(|budget| budget.into()).collect(),
    ))
}

pub async fn detailed(
    Path(id): Path<i32>,
    db: DatabaseConnection,
) -> Result<Json<models::PeriodicBudgetSummary>, StatusCode> {
    // Get the budget requested by `id'
    let budget: periodic_budgets::Model = PeriodicBudgets::find_by_id(id)
        .one(&db)
        .await
        .map_err(internal_server_error)?
        .ok_or(StatusCode::NOT_FOUND)?
        .into();

    // Locate all the budget items for this budget
    let items = budget
        .find_related(BudgetItems)
        .all(&db)
        .await
        .map_err(internal_server_error)?
        .into_iter()
        .map(|item| item.into())
        .collect::<Vec<models::BudgetItem>>();

    // Collect all initial balances for the time period corresponding to this
    let initial_balances = budget
        .find_related(InitialBalances)
        .all(&db)
        .await
        .map_err(internal_server_error)?
        .into_iter()
        .map(|balance| balance.into())
        .collect::<Vec<models::InitialBalance>>();

    // Collect all the transactions for the time period corresponding to this
    let transactions: Vec<models::Transaction> = budget
        .find_related(Transactions)
        .all(&db)
        .await
        .map_err(internal_server_error)?
        .into_iter()
        .map(|transaction| transaction.try_into())
        .collect::<Result<Vec<models::Transaction>, _>>()
        .map_err(internal_server_error)?;

    Ok(Json(models::PeriodicBudgetSummary {
        budget: budget.into(),
        items,
        initial_balances,
        transactions,
    }))
}

///////////////////////////////////////////////////////////////////////////////
