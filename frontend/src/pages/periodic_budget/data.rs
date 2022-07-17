///////////////////////////////////////////////////////////////////////////////
// NAME:            data.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Presenter for the periodic budget view.
//
// CREATED:         04/30/2022
//
// LAST EDITED:     07/16/2022
////

use std::collections::HashMap;
use budget_models::{
    balance_tracker::BalanceTracker,
    calculation::Calculation,
    models::{PeriodicBudget, PeriodicBudgetSummary},
    policy::TransactionReceivedPolicy,
};
use wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::PERIODIC_BUDGETS_PATH;
use crate::network::fetch;
use crate::render::{Render, RenderTable};

use super::account::AccountView;
use super::budget_item::BudgetItemView;

///////////////////////////////////////////////////////////////////////////////
// DataView
////

pub struct DataView {
    budget: PeriodicBudget,
    items: HashMap<String, Vec<BudgetItemView>>,
    accounts: Vec<AccountView>,
}

impl DataView {
    pub async fn get(budget_id: &str, link: Callback<DataView>) ->
        Result<(), JsValue>
    {
        // Get the detailed budget view
        let url = PERIODIC_BUDGETS_PATH.to_string() + "/" + &budget_id;
        let request = web_sys::Request::new_with_str(&url)?;
        let budget: PeriodicBudgetSummary = fetch(request)
            .await
            .map_err::<JsValue, _>(
                |_| "Failed to fetch Periodic Budget Summary".into()
            )?;

        link.emit(budget.into());
        Ok(())
    }

    fn get_account_views(
        summary: &PeriodicBudgetSummary, policy: TransactionReceivedPolicy
    ) -> Vec<AccountView> {
        let mut balance_estimator = BalanceTracker::from_initial_balances(
            summary.initial_balances.iter());
        let mut balance_tracker = BalanceTracker::from_initial_balances(
            summary.initial_balances.iter());

        summary.transactions.iter().for_each(|transaction| {
            balance_estimator.apply(transaction.clone());
            if policy.is_received(&transaction) {
                balance_tracker.apply(transaction.clone());
            }
        });

        summary.initial_balances.iter()
            .map(|balance| {
                let current = balance_tracker.calculate()
                    .get(&balance.account)
                    .unwrap();
                let end = balance_estimator.calculate()
                    .get(&balance.account)
                    .unwrap();
                AccountView::new(&balance, *current, *end)
            }).collect::<Vec<AccountView>>()
    }

    fn get_item_views(_summary: &PeriodicBudgetSummary) ->
        HashMap<String, Vec<BudgetItemView>>
    {
        todo!()
    }
}

impl From<PeriodicBudgetSummary> for DataView {
    fn from(value: PeriodicBudgetSummary) -> Self {
        let policy = TransactionReceivedPolicy::new();
        Self {
            accounts: DataView::get_account_views(&value, policy),
            items: DataView::get_item_views(&value),
            budget: value.budget,
        }
    }
}

impl Render for DataView {
    fn render(&self) -> Html {
        html! {<div><h1>{
            &self.budget.start_date
        }</h1><h2>{ "Budget" }</h2>{
            self.items.iter().map(|(k, v)| {
                html! {
                    <div>
                        <h3>{ k }</h3>
                        <div><table>{ BudgetItemView::header() }{
                            v.iter().map(|item| item.render())
                                .collect::<Html>()
                        }</table></div>
                    </div>
                }
            }).collect::<Html>()
        }<h2>{ "Accounts" }</h2><table>{ AccountView::header() }{
            self.accounts.iter().map(|account| {
                account.render()
            }).collect::<Html>()
        }</table></div>}
    }
}

///////////////////////////////////////////////////////////////////////////////
