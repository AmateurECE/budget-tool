///////////////////////////////////////////////////////////////////////////////
// NAME:            create.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Creation view
//
// CREATED:         05/13/2022
//
// LAST EDITED:     05/14/2022
////

use std::rc::Rc;
use std::str::FromStr;

use strum::IntoEnumIterator;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use budget_models::{
    entities::PeriodicBudgetEndpoint,
    models::{
        Transaction, TransactionType, PeriodicBudget,
    }
};

use crate::network::fetch;
use crate::PERIODIC_BUDGETS_PATH;

///////////////////////////////////////////////////////////////////////////////
// TransactionForm
////

#[derive(Properties, Clone, PartialEq)]
pub struct TransactionFormProperties {
    pub data: Rc<Transaction>,
}

pub enum TransactionFormMessage {
    BudgetSelected(String),
    ReceivedBudgets(Vec<PeriodicBudget>),
    ReceivedOneBudget(PeriodicBudgetEndpoint),
}

#[derive(Default)]
pub struct TransactionForm {
    budget_data: Option<PeriodicBudgetEndpoint>,
    budgets: Option<Vec<PeriodicBudget>>,
}

impl Component for TransactionForm {
    type Message = TransactionFormMessage;
    type Properties = TransactionFormProperties;

    fn create(context: &Context<Self>) -> Self {
        let mut form = Self::default();
        form.request_budgets(context);
        form
    }

    fn update(&mut self, context: &Context<Self>, message: Self::Message) ->
        bool
    {
        use TransactionFormMessage::*;
        if let ReceivedBudgets(budgets) = message {
            if !budgets.is_empty() {
                self.request_one_budget(budgets.first().unwrap().id, context);
            }
            self.budgets = Some(budgets);
        } else if let ReceivedOneBudget(budget) = message {
            self.budget_data = Some(budget);
        } else if let BudgetSelected(id) = message {
            self.request_one_budget(i32::from_str(&id).unwrap(), context);
        }
        true
    }

    fn view(&self, context: &Context<Self>) -> Html {
        use TransactionFormMessage::*;
        html! {
            <form>

                <h2>{ "Transaction" }</h2>
                <div class="input-group">
                    <label for="budget">{ "Budget" }</label>
                    <select id="budget"
                     onchange={context.link().batch_callback(|e: Event| {
                         if let Some(select) = e.target_dyn_into::<
                                 HtmlSelectElement>() {
                             Some(BudgetSelected(select.value()))
                         } else {
                             None
                         }
                     })}>{
                        match &self.budgets {
                            Some(budgets) => budgets.iter()
                                .map(|budget| html! {
                                    <option value={budget.id.to_string()}>{
                                        budget.start_date
                                    }</option>
                                })
                                .collect::<Html>(),
                            None => html! {
                                <option value="">{ "Loading..." }</option>
                            },
                        }
                    }</select>
                </div>

                <div class="input-group">
                    <label for="description">{ "Description" }</label>
                    <input type="text" id="description" />
                </div>

                <div class="input-group">
                    <label for="line-item">{ "Line Item" }</label>
                    <input type="text" id="line-item" />
                </div>

                <div class="input-group">
                    <label for="transaction-type">{
                        "Transaction Type"
                    }</label>
                    <select id="transaction-type">{
                        TransactionType::iter().map(|t_type| {
                            let t_type = t_type.to_string();
                            html! {
                                <option value={t_type.to_lowercase()}>{
                                    t_type
                                }</option>
                            }
                        }).collect::<Html>()
                    }</select>
                </div>

                <div class="input-group">
                    <label for="sending-account">{ "Sending Account" }</label>
                    <select id="sending-account">
                    </select>
                </div>

                <div class="input-group">
                    <label for="receiving-account">{
                        "Receiving Account"
                    }</label>
                    <select id="sending-account">
                    </select>
                </div>

                <div class="input-group">
                    <label for="transfer-fees">{ "Transfer Fees" }</label>
                    <input type="money" id="transfer-fees" />
                </div>

                <div class="input-group">
                    <label for="receiving-entity">{
                        "Receiving Entity"
                    }</label>
                    <input type="text" id="receiving-entity" />
                </div>

                <div class="input-group">
                    <label for="amount">{ "Amount" }</label>
                    <input type="money" id="amount" />
                </div>

                <div class="input-group">
                    <label for="tags">{ "Tags" }</label>
                    <input type="text" id="tags" />
                </div>

                <div class="input-group">
                    <label for="send-date">{ "Send Date" }</label>
                    <input type="date" id="send-date" />
                </div>

                <div class="input-group">
                    <label for="receive-date">{ "Receive Date" }</label>
                    <input type="date" id="receive-date" />
                </div>

                <div class="input-group">
                    <label for="corrects">{ "Corrects Transaction" }</label>
                    <select id="corrects">
                    </select>
                </div>

            </form>
        }
    }
}

impl TransactionForm {
    fn request_budgets(&mut self, context: &Context<Self>) {
        self.budgets = None;

        use TransactionFormMessage::*;
        let link = context.link().callback(
            |budgets: Vec<PeriodicBudget>| ReceivedBudgets(budgets)
        );
        spawn_local(async move {
            // Get the list of budgets
            let request = web_sys::Request::new_with_str(PERIODIC_BUDGETS_PATH)
                .unwrap();
            let budgets: Vec<PeriodicBudget> = fetch(request).await.unwrap();
            link.emit(budgets);
        });
    }

    fn request_one_budget(&mut self, id: i32, context: &Context<Self>) {
        self.budget_data = None;

        use TransactionFormMessage::*;
        let link = context.link().callback(
            |budget: PeriodicBudgetEndpoint| ReceivedOneBudget(budget)
        );
        let id = id.to_string();
        spawn_local(async move {
            let url = PERIODIC_BUDGETS_PATH.to_string() + "/" + &id;
            let request = web_sys::Request::new_with_str(&url).unwrap();
            let budget: PeriodicBudgetEndpoint = fetch(request).await.unwrap();
            link.emit(budget);
        })
    }
}

///////////////////////////////////////////////////////////////////////////////
// EntitySelect
////

pub enum EntitySelect {
    Select,
    Transaction(Rc<Transaction>),
}

impl Default for EntitySelect {
    fn default() -> Self {
        Self::Select
    }
}

///////////////////////////////////////////////////////////////////////////////
// CreateView
////

pub enum CreateViewMessage {
    Selected(String),
}

#[derive(Default)]
pub struct CreateView(EntitySelect);

impl Component for CreateView {
    type Message = CreateViewMessage;
    type Properties = ();

    fn create(_context: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _context: &Context<Self>, message: Self::Message) ->
        bool
    {
        if let CreateViewMessage::Selected(entity_type) = message {
            match entity_type.as_str() {
                "transaction" => {
                    self.0 = EntitySelect::Transaction(
                        Rc::new(Transaction::default()));
                },

                "" => {
                    self.0 = EntitySelect::Select;
                },

                &_ => {
                    panic!("Unexpected string in form")
                },
            }
            true
        } else {
            false
        }
    }

    fn view(&self, context: &Context<Self>) -> Html {
        html! {
            <div id="create-form">
                <label for="entity-select">{ "Type" }</label>
                <select name="type" id="entity-select"
                    onchange={context.link().batch_callback(|e: Event| {
                        if let Some(select) = e.target_dyn_into::<
                                HtmlSelectElement>() {
                            Some(CreateViewMessage::Selected(select.value()))
                        } else {
                            None
                        }
                    })}>
                    <option value="">{ "--Unselected--" }</option>
                    <option value="transaction">{ "Transaction" }</option>
                </select>
            {
                match &self.0 {
                    EntitySelect::Select => html! { "" },
                    EntitySelect::Transaction(data) => html! {
                        <TransactionForm {data} />
                    },
                }
            }
            </div>
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
