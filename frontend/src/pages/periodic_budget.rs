///////////////////////////////////////////////////////////////////////////////
// NAME:            periodic_budget_view.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Page for view of the Periodic Budget
//
// CREATED:         04/20/2022
//
// LAST EDITED:     04/30/2022
////

use std::collections::HashMap;

use budget_models::{
    entities::PeriodicBudgetEndpoint,
    models::{BudgetItem, PeriodicBudget},
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::budgetizer::{
    TrackedBudgetItem,
    TrackedAccount,
    Budgetizer,
};
use crate::network::fetch;
use crate::PERIODIC_BUDGETS;
use crate::render::Render;

///////////////////////////////////////////////////////////////////////////////
// BudgetItemView
////

pub struct BudgetItemView(TrackedBudgetItem);
impl From<TrackedBudgetItem> for BudgetItemView {
    fn from(item: TrackedBudgetItem) -> BudgetItemView {
        BudgetItemView(item)
    }
}

impl Render for BudgetItemView {
    fn render(&self) -> Html {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
// AccountView
////

pub struct AccountView(TrackedAccount);
impl From<TrackedAccount> for AccountView {
    fn from(account: TrackedAccount) -> Self {
        AccountView(account)
    }
}

impl Render for AccountView {
    fn render(&self) -> Html {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
// ResolvedBudgetView
////

pub struct ResolvedBudgetView {
    budget: PeriodicBudget,
    items: HashMap<String, Vec<BudgetItemView>>,
    accounts: HashMap<String, Vec<AccountView>>,
}

impl Render for ResolvedBudgetView {
    fn render(&self) -> Html {
        html! {<div>{
            self.items.iter().map(|(k, v)| {
                html! {
                    <div>
                        <h2>{ k }</h2>
                        <div>{
                            v.iter().map(|item| item.render())
                                .collect::<Html>()
                        }</div>
                    </div>
                }
            }).collect::<Html>()
        }</div>}
    }
}

///////////////////////////////////////////////////////////////////////////////
// PeriodicBudgetView
////

#[derive(Properties, PartialEq)]
pub struct PeriodicBudgetViewProperties {
    pub id: i32,
}

pub enum PeriodicBudgetViewMessage {
    Received(PeriodicBudgetEndpoint),
}

#[derive(Default)]
pub struct PeriodicBudgetView {
    budget: Option<ResolvedBudgetView>,
}

impl Component for PeriodicBudgetView {
    type Message = PeriodicBudgetViewMessage;
    type Properties = PeriodicBudgetViewProperties;

    fn create(context: &Context<Self>) -> Self {
        let mut view = PeriodicBudgetView::default();
        view.request_budget(context);
        view
    }

    // Here, we abuse the changed handler to request the new budget
    fn changed(&mut self, context: &Context<Self>) -> bool {
        self.request_budget(context);
        true
    }

    fn update(&mut self, _context: &Context<Self>, message: Self::Message) ->
        bool
    {
        use PeriodicBudgetViewMessage::*;
        match message {
            Received(budget) => {
                self.budget = Some(self.budgetize(budget));
                true
            }
        }
    }

    fn view(&self, _context: &Context<Self>) -> Html {
        match &self.budget {
            Some(budget) => budget.render(),
            None => html! { <p>{ "Loading..." }</p> },
        }
    }
}

impl PeriodicBudgetView {
    // Submit HTTP request to get the budget and related data from the server
    fn request_budget(&mut self, context: &Context<Self>) {
        self.budget = None;

        use PeriodicBudgetViewMessage::*;
        let link = context.link().callback(
            |budget: PeriodicBudgetEndpoint| Received(budget)
        );
        let url = PERIODIC_BUDGETS.to_string() + "/"
            + &context.props().id.to_string();
        spawn_local(async move {
            let request = web_sys::Request::new_with_str(&url)
                .unwrap();
            let response: PeriodicBudgetEndpoint = fetch(request).await
                .unwrap();
            link.emit(response);
        });
    }

    // Convert PeriodicBudgetEndpoint to ResolvedBudgetView using a Budgetizer
    fn budgetize(&self, budget: PeriodicBudgetEndpoint) -> ResolvedBudgetView {
        let items = budget.items.into_iter().map(|(_, v)| v)
            .collect::<Vec<Vec<BudgetItem>>>()
            .concat()
            .into_iter()
            .map(|item| {
                (item.id, item)
            }).collect::<HashMap<i32, BudgetItem>>();

        let budgetizer = Budgetizer::new(budget.budget);
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
