///////////////////////////////////////////////////////////////////////////////
// NAME:            periodic_budget_view.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Page for view of the Periodic Budget
//
// CREATED:         04/20/2022
//
// LAST EDITED:     07/07/2022
////

use std::collections::HashMap;

use budget_models::models::{Account, PeriodicBudget, PeriodicBudgetSummary};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::balance_synchronizer::BalanceSynchronizer;
use crate::budgetizer::{
    TrackedBudgetItem,
    TrackedAccount,
    Budgetizer,
};
use crate::network::fetch;
use crate::{ACCOUNTS_PATH, PERIODIC_BUDGETS_PATH};
use crate::render::{Render, RenderTable};

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
        let budgeted = format!("{:.2}", (self.0.item.budgeted as f64) / 100.0);
        let spent = format!("{:.2}", (self.0.spent as f64) / 100.0);
        html! { <tr><td data-label="Description">{
            &self.0.item.description
        }</td><td data-label="Transaction Type">{
            &self.0.item.transaction_type.to_string()
        }</td><td data-label="From">{
            match &self.0.item.from_account {
                Some(account) => account,
                None => "",
            }
        }</td><td data-label="To">{
            match &self.0.item.to_account {
                Some(account) => account,
                None => "",
            }
        }</td><td data-label="Budgeted">{
            budgeted
        }</td><td data-label="Spent">{
            spent
        }</td></tr>}
    }
}

impl RenderTable for BudgetItemView {
    fn header() -> Html {
        html! {<tr><th>{
            "Description"
        }</th><th>{
            "Transaction Type"
        }</th><th>{
            "From"
        }</th><th>{
            "To"
        }</th><th>{
            "Budgeted"
        }</th><th>{
            "Spent"
        }</th></tr>}
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
        let initial = format!(
            "{:.2}", (self.0.initial_balance as f64) / 100.0);
        let current = format!(
            "{:.2}", (self.0.current_balance as f64) / 100.0);
        let expected_end = format!(
            "{:.2}", (self.0.expected_end_balance as f64) / 100.0);
        html! {<tr><td data-label="Name">{
            &self.0.account.name
        }</td><td data-label="Initial Balance">{
            initial
        }</td><td data-label="Current Balance">{
            current
        }</td><td data-label="Expected End Balance">{
            expected_end
        }</td></tr>}
    }
}

impl RenderTable for AccountView {
    fn header() -> Html {
        html! {<tr><th>{
            "Name"
        }</th><th>{
            "Initial Balance"
        }</th><th>{
            "Current Balance"
        }</th><th>{
            "Expected End Balance"
        }</th></tr>}
    }
}

///////////////////////////////////////////////////////////////////////////////
// ResolvedBudgetView
////

pub struct ResolvedBudgetView {
    budget: PeriodicBudget,
    items: HashMap<String, Vec<BudgetItemView>>,
    accounts: Vec<AccountView>,
}

impl Render for ResolvedBudgetView {
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
// Data Transformations
////

mod data_transformations {
    use std::collections::HashMap;
    use budget_models::models::{Account, BudgetItem, InitialBalance};
    use crate::budgetizer::{TrackedAccount, TrackedBudgetItem};
    use super::{AccountView, BudgetItemView};

    pub fn trackable_accounts(
        accounts: Vec<Account>, initial_balances: Vec<InitialBalance>
    ) -> HashMap<String, TrackedAccount>
    {
        accounts.into_iter().map(|account| {
            let initial_balance = initial_balances.iter()
                .find(|item| item.account == account.name);
            let name = account.name.to_owned();
            match initial_balance {
                Some(initial_balance) => (
                    name, TrackedAccount::with_balance(
                        account, initial_balance)
                ),
                None => (name, account.into()),
            }
        }).collect::<HashMap<String, TrackedAccount>>()
    }

    pub fn viewable_accounts(accounts: HashMap<String, TrackedAccount>) ->
        Vec<AccountView>
    {
        accounts.into_iter().map(|(_, account)| account.into())
            .collect::<Vec<AccountView>>()
    }

    pub fn trackable_budget_items(items: Vec<BudgetItem>) ->
        HashMap<i32, TrackedBudgetItem>
    {
        items.into_iter().map(|item| {
            (item.id, item.into())
        }).collect::<HashMap<i32, TrackedBudgetItem>>()
    }

    // Re-organize the trackable budget items for rendering.
    pub fn viewable_budget_items(
        budget_items: HashMap<i32, TrackedBudgetItem>
    ) -> HashMap<String, Vec<BudgetItemView>>
    {
        let mut items = budget_items.values()
            .map(|i| i.item.category.to_owned())
            .collect::<Vec<String>>();
        items.sort();
        items.dedup();
        let mut items = items.into_iter()
            .map(|item| (item, Vec::new()))
            .collect::<HashMap<String, Vec<BudgetItemView>>>();

        let mut budget_items = budget_items.into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<TrackedBudgetItem>>();
        while !budget_items.is_empty() {
            let item = budget_items.pop().unwrap();
            items.get_mut(&item.item.category).unwrap().push(item.into());
        }

        items
    }
}

///////////////////////////////////////////////////////////////////////////////
// PeriodicBudgetView
////

// Data needed for rendering this view
pub struct PeriodicBudgetViewContext {
    pub budget: PeriodicBudgetSummary,
    pub accounts: Vec<Account>,
    pub balancer: BalanceSynchronizer,
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
    balancer: Option<BalanceSynchronizer>,
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
            let budget: PeriodicBudgetSummary = fetch(request).await
                .unwrap();

            // Get the account view
            let request = web_sys::Request::new_with_str(ACCOUNTS_PATH)
                .unwrap();
            let accounts: Vec<Account> = fetch(request).await
                .unwrap();

            // Get initial balances for all accounts
            let balancer = BalanceSynchronizer::new().await.unwrap();
            link.emit(PeriodicBudgetViewContext {
                budget, accounts, balancer,
            });
        });
    }

    // Convert PeriodicBudgetSummary to ResolvedBudgetView using a Budgetizer
    fn budgetize(&self, data: PeriodicBudgetViewContext) -> ResolvedBudgetView
    {
        use data_transformations::*;

        // Arrange the budget items and the accounts to be trackable
        let mut budget_items = trackable_budget_items(data.budget.items);
        let mut accounts = trackable_accounts(
            data.accounts, data.budget.initial_balances);

        let budgetizer = Budgetizer::new(data.budget.budget.clone());
        // Predict account balances
        for budget_item in budget_items.values() {
            budgetizer.predict_balance(&mut accounts, &budget_item);
        }

        // Apply the transactions to the accounts and budget
        for transaction in data.budget.transactions {
            budgetizer.apply_transaction(
                &mut budget_items,
                &mut accounts,
                &transaction
            );
        }

        // TODO: Implement balance synchronizer
        // let mut balancer = data.balancer;
        // let accounts_snapshot = accounts.clone();
        // spawn_local(async move {
        //     balancer.update_balances(accounts_snapshot).await.unwrap();
        // });

        // Convert budget items and accounts back into viewable representation
        // and return
        ResolvedBudgetView {
            budget: data.budget.budget,
            items: viewable_budget_items(budget_items),
            accounts: viewable_accounts(accounts),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
