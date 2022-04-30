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
    models::{Account, BudgetItem, PeriodicBudget},
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::budgetizer::{
    TrackedBudgetItem,
    TrackedAccount,
    Budgetizer,
};
use crate::network::fetch;
use crate::{ACCOUNTS_PATH, PERIODIC_BUDGETS_PATH};
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

// Data needed for rendering this view
pub struct PeriodicBudgetViewContext {
    pub budget: PeriodicBudgetEndpoint,
    pub accounts: Vec<Account>,
}

// Properties for this component
#[derive(Properties, PartialEq)]
pub struct PeriodicBudgetViewProperties {
    pub id: i32,
}

// Message for this component
pub enum PeriodicBudgetViewMessage {
    Received(PeriodicBudgetViewContext),
}

// Component for this view
#[derive(Default)]
pub struct PeriodicBudgetView {
    budget: Option<ResolvedBudgetView>,
}

impl Component for PeriodicBudgetView {
    type Message = PeriodicBudgetViewMessage;
    type Properties = PeriodicBudgetViewProperties;

    fn create(context: &Context<Self>) -> Self {
        let mut view = PeriodicBudgetView::default();
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
    fn request_context(&mut self, context: &Context<Self>) {
        self.budget = None;

        use PeriodicBudgetViewMessage::*;
        let link = context.link().callback(
            |budget: PeriodicBudgetViewContext| Received(budget)
        );
        let id = context.props().id.to_string();
        spawn_local(async move {
            // Get the detailed budget view
            let url = PERIODIC_BUDGETS_PATH.to_string() + "/" + &id;
            let request = web_sys::Request::new_with_str(&url)
                .unwrap();
            let budget: PeriodicBudgetEndpoint = fetch(request).await
                .unwrap();

            // Get the account view
            let request = web_sys::Request::new_with_str(ACCOUNTS_PATH)
                .unwrap();
            let accounts: Vec<Account> = fetch(request).await
                .unwrap();
            link.emit(PeriodicBudgetViewContext {
                budget, accounts,
            });
        });
    }

    // Convert PeriodicBudgetEndpoint to ResolvedBudgetView using a Budgetizer
    fn budgetize(&self, data: PeriodicBudgetViewContext) -> ResolvedBudgetView
    {
        let mut items = data.budget.items.into_iter().map(|(_, v)| v)
            .collect::<Vec<Vec<BudgetItem>>>()
            .concat()
            .into_iter()
            .map(|item| {
                (item.id, item.into())
            }).collect::<HashMap<i32, TrackedBudgetItem>>();

        let mut accounts = data.accounts.into_iter()
            .map(|account| {
                let initial_balance = data.budget.initial_balances.iter()
                    .find(|item| item.account == account.name);
                let name = account.name.to_owned();
                match initial_balance {
                    Some(initial_balance) => (
                        name, TrackedAccount::with_balance(
                            account, initial_balance)
                    ),
                    None => (name, account.into()),
                }
            })
            .collect::<HashMap<String, TrackedAccount>>();

        let budgetizer = Budgetizer::new(data.budget.budget);
        for transaction in data.budget.transactions {
            budgetizer.apply_transaction(
                &mut items,
                &mut accounts,
                &transaction
            );
        }

        // ResolvedBudgetView {
        //     budget,
        //     items,
        //     accounts,
        // }
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
