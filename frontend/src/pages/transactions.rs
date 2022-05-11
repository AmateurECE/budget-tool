///////////////////////////////////////////////////////////////////////////////
// NAME:            transactions.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     A view to see an incremental breakdown of all transactions.
//
// CREATED:         05/10/2022
//
// LAST EDITED:     05/10/2022
////

use yew::prelude::*;

use budget_models::{
    models::Transaction,
    entities::PeriodicBudgetEndpoint,
};
use wasm_bindgen_futures::spawn_local;

use crate::network::fetch;

use crate::PERIODIC_BUDGETS_PATH;

///////////////////////////////////////////////////////////////////////////////
// TransactionEntry
////

pub struct TransactionEntry {
    transaction: Transaction,
}

impl From<Transaction> for TransactionEntry {
    fn from(transaction: Transaction) -> Self {
        Self {
            transaction,
        }
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
    pub budget: PeriodicBudgetEndpoint,
}

pub enum TransactionViewMessage {
    Received(TransactionViewContext),
}

#[derive(Default)]
pub struct TransactionView {
    budget: Option<Vec<TransactionEntry>>
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
                self.budget = Some(self.transactionize(budget));
                true
            }
        }
    }

    fn view(&self, _context: &Context<Self>) -> Html {
        match &self.budget {
            Some(budget) => html! { <p>{ "Done!" }</p> },
            None => html! { <p>{ "Loading..." }</p> },
        }
    }
}

impl TransactionView {
    fn request_context(&mut self, context: &Context<Self>) {
        self.budget = None;

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
            let budget: PeriodicBudgetEndpoint = fetch(request).await
                .unwrap();
            link.emit(TransactionViewContext {
                budget
            });
        });
    }

    fn transactionize(&self, data: TransactionViewContext) ->
        Vec<TransactionEntry>
    {
        Vec::new()
    }
}

///////////////////////////////////////////////////////////////////////////////
