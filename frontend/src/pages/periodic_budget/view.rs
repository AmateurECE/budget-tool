///////////////////////////////////////////////////////////////////////////////
// NAME:            view.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Page for view of the Periodic Budget
//
// CREATED:         04/20/2022
//
// LAST EDITED:     07/15/2022
////

use std::collections::HashMap;
use budget_models::models::PeriodicBudget;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::network::fetch;
use crate::{ACCOUNTS_PATH, PERIODIC_BUDGETS_PATH};
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
// View
////

// Properties for this component
#[derive(Properties, PartialEq)]
pub struct ViewProperties {
    pub id: i32,
}

// Component for this view
#[derive(Default)]
pub struct View(Option<DataView>);

impl Component for View {
    type Message = ();
    type Properties = ViewProperties;

    fn create(context: &Context<Self>) -> Self {
        let mut view = View::default();
        view.request_context(context);
        view
    }

    // Here, we abuse the changed handler to request the new budget
    fn changed(&mut self, context: &Context<Self>) -> bool {
        self.request_context(context);
        true
    }

    fn update(&mut self, _context: &Context<Self>, message: Self::Message) ->
        bool
    { true }

    fn view(&self, _context: &Context<Self>) -> Html {
        match &self.0 {
            Some(context) => context.render(),
            None => html! { <p>{ "Loading..." }</p> },
        }
    }
}

impl View {
    // Submit HTTP request to get the budget and related data from the server
    fn request_context(&mut self, context: &Context<Self>) {
        // self.budget = None;

        // use ViewMessage::*;
        // let link = context.link().callback(
        //     |budget: DataView| Received(budget)
        // );
        // let id = context.props().id.to_string();
        // spawn_local(async move {
        //     // Get the detailed budget view
        //     let url = PERIODIC_BUDGETS_PATH.to_string() + "/" + &id;
        //     let request = web_sys::Request::new_with_str(&url)
        //         .unwrap();
        //     let budget: PeriodicBudgetSummary = fetch(request).await
        //         .unwrap();

        //     // Get the account view
        //     let request = web_sys::Request::new_with_str(ACCOUNTS_PATH)
        //         .unwrap();
        //     let accounts: Vec<Account> = fetch(request).await
        //         .unwrap();

        //     // Get initial balances for all accounts
        //     let balancer = BalanceSynchronizer::new().await.unwrap();
        //     link.emit(DataView {
        //         budget, accounts, balancer,
        //     });
        // });
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
