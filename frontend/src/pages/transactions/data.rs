///////////////////////////////////////////////////////////////////////////////
// NAME:            data.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Data view for the Transactions view.
//
// CREATED:         07/17/2022
//
// LAST EDITED:     07/18/2022
////

use std::collections::HashMap;
use budget_models::{
    balance_tracker::BalanceTracker,
    calculation::Calculation,
    models::PeriodicBudgetSummary,
    money::Money
};
use wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::PERIODIC_BUDGETS_PATH;
use crate::network::fetch;
use crate::render::{Render, RenderTable};

use super::transactions::TransactionView;

///////////////////////////////////////////////////////////////////////////////
// DataView
////

pub struct DataView(Vec<TransactionView>);

impl DataView {
    pub async fn get(budget_id: &str, link: Callback<DataView>) ->
        Result<(), JsValue>
    {
        let url = PERIODIC_BUDGETS_PATH.to_string() + "/" + &budget_id;
        let request = web_sys::Request::new_with_str(&url)?;
        let budget: PeriodicBudgetSummary = fetch(request).await
            .map_err::<JsValue, _>(
                |_| "Failed to fetch periodic budget summary".into()
            )?;
        link.emit(budget.into());
        Ok(())
    }
}

impl From<PeriodicBudgetSummary> for DataView {
    fn from(mut value: PeriodicBudgetSummary) -> Self {
        value.transactions.sort_by(|a, b| {
            a.send_date.partial_cmp(&b.send_date).unwrap()
        });

        let items = value.items.into_iter()
            .map(|item| (item.id, item.description))
            .collect::<HashMap<i32, String>>();
        let mut balance_tracker = BalanceTracker::from_initial_balances(
            value.initial_balances.iter());
        Self(value.transactions.into_iter().map(|transaction| {
            let line_item = items.get(&transaction.line_item).unwrap();
            balance_tracker.apply(transaction.clone());

            let new_sending = transaction.sending_account.as_ref()
                .map(|account| {
                    *balance_tracker.calculate().get(account).unwrap()
                }).unwrap_or(Money::default());
            let new_receiving = transaction.receiving_account.as_ref()
                .map(|account| {
                    *balance_tracker.calculate().get(account).unwrap()
                }).unwrap_or(Money::default());

            TransactionView::new(
                transaction, new_sending, new_receiving, line_item.clone()
            )
        }).collect::<Vec<TransactionView>>())
    }
}

impl Render for DataView {
    fn render(&self) -> Html {
        html! {
            <main>
                <h1>{ "Transactions" }</h1>
                <table>{ TransactionView::header() }{
                    self.0.iter().map(|item| item.render()).collect::<Html>()
                }</table>
            </main>
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
