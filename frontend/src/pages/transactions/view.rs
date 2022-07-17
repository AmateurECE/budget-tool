///////////////////////////////////////////////////////////////////////////////
// NAME:            view.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     View implementing the Transactions page.
//
// CREATED:         07/17/2022
//
// LAST EDITED:     07/17/2022
////

use std::collections::HashMap;
use budget_models::models::{TransactionType, PeriodicBudgetSummary};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::network::fetch;
use crate::PERIODIC_BUDGETS_PATH;
use crate::render::{Render, RenderTable};

use super::transactions::TransactionView;

///////////////////////////////////////////////////////////////////////////////
// View
////

#[derive(Properties, PartialEq)]
pub struct ViewProperties {
    pub id: i32,
}

pub struct ViewContext {
    pub budget: PeriodicBudgetSummary,
}

pub enum ViewMessage {
    Received(ViewContext),
}

#[derive(Default)]
pub struct View {
    transactions: Option<Vec<TransactionView>>
}

impl Component for View {
    type Message = ViewMessage;
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
    {
        use ViewMessage::*;
        match message {
            Received(budget) => {
                self.transactions = Some(self.transactionize(budget));
                true
            }
        }
    }

    fn view(&self, _context: &Context<Self>) -> Html {
        match &self.transactions {
            Some(transactions) => html! {
                <main>
                    <h1>{ "Transactions" }</h1>
                    <table>{ TransactionView::header() }{
                        transactions.iter().map(|item| item.render())
                            .collect::<Html>()
                    }</table>
                    </main>
            },
            None => html! { <p>{ "Loading..." }</p> },
        }
    }
}

impl View {
    fn request_context(&mut self, context: &Context<Self>) {
        self.transactions = None;

        use ViewMessage::*;
        let link = context.link().callback(
            |budget: ViewContext| Received(budget)
        );
        let id = context.props().id.to_string();
        spawn_local(async move {
            // Get the detailed budget view
            let url = PERIODIC_BUDGETS_PATH.to_string() + "/" + &id;
            let request = web_sys::Request::new_with_str(&url)
                .unwrap();
            let budget: PeriodicBudgetSummary = fetch(request).await
                .unwrap();
            link.emit(ViewContext {
                budget
            });
        });
    }

    fn transactionize(&self, data: ViewContext) ->
        Vec<TransactionView>
    {
        let mut accounts = data.budget.initial_balances.into_iter()
            .map(|b| (b.account, b.balance))
            .collect::<HashMap<String, i64>>();

        let line_items = data.budget.items.into_iter()
            .map(|i| (i.id, i.description))
            .collect::<HashMap<i32, String>>();

        let mut transactions = data.budget.transactions.into_iter()
            .map(|t| t.into())
            .collect::<Vec<TransactionView>>();
        transactions.sort_by(|a, b| a.transaction.send_date.partial_cmp(
            &b.transaction.send_date).unwrap());

        // Have to map in info about sending/receiving accounts
        use TransactionType::*;
        for transaction in &mut transactions {
            transaction.line_item_name = line_items
                .get(&transaction.transaction.line_item)
                .unwrap()
                .clone();

            if Income != transaction.transaction.transaction_type {
                // Update sending_account balance
                let sending_account = accounts.get_mut(
                    transaction.transaction.sending_account.as_ref()
                        .unwrap()
                ).unwrap();
                *sending_account += transaction.transaction.amount;
                if let Some(fees) = transaction.transaction.transfer_fees {
                    *sending_account -= fees;
                }
                transaction.sending_new_balance = *sending_account;
            }

            if Expense != transaction.transaction.transaction_type {
                // Update receiving_account balance
                let account_name = transaction.transaction
                    .receiving_account.as_ref().unwrap();
                let receiving_account = accounts.get_mut(account_name)
                    .expect(&format!("No account named '{}'!", account_name));

                if Payment == transaction.transaction.transaction_type {
                    *receiving_account += -1 * transaction.transaction.amount;
                } else {
                    *receiving_account += transaction.transaction.amount;
                }
                if let Some(fees) = transaction.transaction.transfer_fees {
                    *receiving_account += fees;
                }
                transaction.receiving_new_balance = *receiving_account;
            }
        }

        transactions
    }
}

///////////////////////////////////////////////////////////////////////////////
