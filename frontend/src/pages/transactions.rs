///////////////////////////////////////////////////////////////////////////////
// NAME:            transactions.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     A view to see an incremental breakdown of all transactions.
//
// CREATED:         05/10/2022
//
// LAST EDITED:     07/07/2022
////

use std::collections::HashMap;
use budget_models::models::{
    Transaction, TransactionType, PeriodicBudgetSummary
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::network::fetch;
use crate::PERIODIC_BUDGETS_PATH;
use crate::render::{Render, RenderTable};

///////////////////////////////////////////////////////////////////////////////
// TransactionEntry
////

pub struct TransactionEntry {
    pub transaction: Transaction,
    pub sending_new_balance: i64,
    pub receiving_new_balance: i64,
    pub line_item_name: String,
}

impl From<Transaction> for TransactionEntry {
    fn from(transaction: Transaction) -> Self {
        Self {
            transaction,
            sending_new_balance: 0,
            receiving_new_balance: 0,
            line_item_name: String::new(),
        }
    }
}

impl Render for TransactionEntry {
    fn render(&self) -> Html {
        let receiving = match self.transaction.receiving_account.as_ref() {
            Some(account) => {
                format!("{} / {:.2}", account.clone(),
                        (self.receiving_new_balance as f64) / 100.0)
            },
            None => "".to_string(),
        };

        let sending = match self.transaction.sending_account.as_ref() {
            Some(account) => {
                format!("{} / {:.2}", account.clone(),
                        (self.sending_new_balance as f64) / 100.0)
            },
            None => "".to_string(),
        };

        let transfer_fees = match &self.transaction.transfer_fees {
            Some(fees) => format!("{}", fees),
            None => "".to_string(),
        };
        let corrects = match &self.transaction.corrects {
            Some(correction) => format!("{:?}", correction),
            None => "".to_string(),
        };

        let send_date = self.transaction.send_date.format("%m-%d")
            .to_string();

        let amount = format!(
            "{:.2}", (self.transaction.amount as f64) / 100.0);

        html! {<tr><td>{
            &send_date
        }</td><td>{
            &self.transaction.description
        }</td><td>{
            &self.line_item_name
        }</td><td>{
            &sending
        }</td><td>{
            &receiving
        }</td><td>{
            &amount
        }</td><td>{
            &transfer_fees
        }</td><td>{
            &corrects
        }</td></tr>}
    }
}

impl RenderTable for TransactionEntry {
    fn header() -> Html {
        html! {<tr><th>{
            "Date"
        }</th><th>{
            "Description"
        }</th><th>{
            "Line Item"
        }</th><th>{
            "Sending Account/New Balance"
        }</th><th>{
            "Receiving Account/New Balance"
        }</th><th>{
            "Amount"
        }</th><th>{
            "Transfer Fees"
        }</th><th>{
            "Corrects"
        }</th></tr>}
    }
}

///////////////////////////////////////////////////////////////////////////////
// TransactionView
////

#[derive(Properties, PartialEq)]
pub struct TransactionViewProperties {
    pub id: i32,
}

pub struct TransactionViewContext {
    pub budget: PeriodicBudgetSummary,
}

pub enum TransactionViewMessage {
    Received(TransactionViewContext),
}

#[derive(Default)]
pub struct TransactionView {
    transactions: Option<Vec<TransactionEntry>>
}

impl Component for TransactionView {
    type Message = TransactionViewMessage;
    type Properties = TransactionViewProperties;

    fn create(context: &Context<Self>) -> Self {
        let mut view = TransactionView::default();
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
        use TransactionViewMessage::*;
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
                    <table>{ TransactionEntry::header() }{
                        transactions.iter().map(|item| item.render())
                            .collect::<Html>()
                    }</table>
                    </main>
            },
            None => html! { <p>{ "Loading..." }</p> },
        }
    }
}

impl TransactionView {
    fn request_context(&mut self, context: &Context<Self>) {
        self.transactions = None;

        use TransactionViewMessage::*;
        let link = context.link().callback(
            |budget: TransactionViewContext| Received(budget)
        );
        let id = context.props().id.to_string();
        spawn_local(async move {
            // Get the detailed budget view
            let url = PERIODIC_BUDGETS_PATH.to_string() + "/" + &id;
            let request = web_sys::Request::new_with_str(&url)
                .unwrap();
            let budget: PeriodicBudgetSummary = fetch(request).await
                .unwrap();
            link.emit(TransactionViewContext {
                budget
            });
        });
    }

    fn transactionize(&self, data: TransactionViewContext) ->
        Vec<TransactionEntry>
    {
        let mut accounts = data.budget.initial_balances.into_iter()
            .map(|b| (b.account, b.balance))
            .collect::<HashMap<String, i64>>();

        let line_items = data.budget.items.into_iter()
            .map(|i| (i.id, i.description))
            .collect::<HashMap<i32, String>>();

        let mut transactions = data.budget.transactions.into_iter()
            .map(|t| t.into())
            .collect::<Vec<TransactionEntry>>();
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
