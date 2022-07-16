///////////////////////////////////////////////////////////////////////////////
// NAME:            periodic_budget_view.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Page for view of the Periodic Budget
//
// CREATED:         04/20/2022
//
// LAST EDITED:     07/15/2022
////

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::network::fetch;
use crate::{ACCOUNTS_PATH, PERIODIC_BUDGETS_PATH};
use crate::render::{Render, RenderTable};

///////////////////////////////////////////////////////////////////////////////
// BudgetItemView
////

pub struct BudgetItemView;

impl Render for BudgetItemView {
    fn render(&self) -> Html {
        // let budgeted = format!("{:.2}", (self.0.item.budgeted as f64) / 100.0);
        // let spent: f64 = self.0.spent.get_total().into();
        // html! { <tr><td data-label="Description">{
        //     &self.0.item.description
        // }</td><td data-label="Transaction Type">{
        //     &self.0.item.transaction_type.to_string()
        // }</td><td data-label="From">{
        //     match &self.0.item.from_account {
        //         Some(account) => account,
        //         None => "",
        //     }
        // }</td><td data-label="To">{
        //     match &self.0.item.to_account {
        //         Some(account) => account,
        //         None => "",
        //     }
        // }</td><td data-label="Budgeted">{
        //     budgeted
        // }</td><td data-label="Spent">{
        //     format!("{:.2}", &spent)
        // }</td></tr>}
        todo!()
    }
}

impl RenderTable for BudgetItemView {
    fn header() -> Html {
        // html! {<tr><th>{
        //     "Description"
        // }</th><th>{
        //     "Transaction Type"
        // }</th><th>{
        //     "From"
        // }</th><th>{
        //     "To"
        // }</th><th>{
        //     "Budgeted"
        // }</th><th>{
        //     "Spent"
        // }</th></tr>}
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
// AccountView
////

pub struct AccountView;

impl Render for AccountView {
    fn render(&self) -> Html {
        // let initial = format!(
        //     "{:.2}", (self.0.initial_balance as f64) / 100.0);
        // let current = format!(
        //     "{:.2}", (self.0.current_balance as f64) / 100.0);
        // let expected_end = format!(
        //     "{:.2}", (self.0.expected_end_balance as f64) / 100.0);
        // html! {<tr><td data-label="Name">{
        //     &self.0.account.name
        // }</td><td data-label="Initial Balance">{
        //     initial
        // }</td><td data-label="Current Balance">{
        //     current
        // }</td><td data-label="Expected End Balance">{
        //     expected_end
        // }</td></tr>}
        todo!()
    }
}

impl RenderTable for AccountView {
    fn header() -> Html {
        // html! {<tr><th>{
        //     "Name"
        // }</th><th>{
        //     "Initial Balance"
        // }</th><th>{
        //     "Current Balance"
        // }</th><th>{
        //     "Expected End Balance"
        // }</th></tr>}
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
// ViewContext
////

pub struct ViewContext;

impl ViewContext {
    fn render(&self) -> Html {
        // html! {<div><h1>{
        //     &self.budget.start_date
        // }</h1><h2>{ "Budget" }</h2>{
        //     self.items.iter().map(|(k, v)| {
        //         html! {
        //             <div>
        //                 <h3>{ k }</h3>
        //                 <div><table>{ BudgetItemView::header() }{
        //                     v.iter().map(|item| item.render())
        //                         .collect::<Html>()
        //                 }</table></div>
        //             </div>
        //         }
        //     }).collect::<Html>()
        // }<h2>{ "Accounts" }</h2><table>{ AccountView::header() }{
        //     self.accounts.iter().map(|account| {
        //         account.render()
        //     }).collect::<Html>()
        // }</table></div>}
        todo!()
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
pub struct View(Option<ViewContext>);

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
        //     |budget: ViewContext| Received(budget)
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
        //     link.emit(ViewContext {
        //         budget, accounts, balancer,
        //     });
        // });
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
