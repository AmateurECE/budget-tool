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
use budget_models::models::{PeriodicBudget, PeriodicBudgetSummary};
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

        // link.emit(Self {budget.budget, accounts});
        // Ok(())
        todo!()
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
