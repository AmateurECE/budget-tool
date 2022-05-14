///////////////////////////////////////////////////////////////////////////////
// NAME:            create.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Creation view
//
// CREATED:         05/13/2022
//
// LAST EDITED:     05/13/2022
////

use std::rc::Rc;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use strum::IntoEnumIterator;

use budget_models::models::{Transaction, TransactionType};

///////////////////////////////////////////////////////////////////////////////
// TransactionForm
////

#[derive(Properties, Clone, PartialEq)]
pub struct TransactionFormProperties {
    pub data: Rc<Transaction>,
}

pub struct TransactionForm;
impl Component for TransactionForm {
    type Message = ();
    type Properties = TransactionFormProperties;

    fn create(_context: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _context: &Context<Self>) -> Html {
        html! {
            <form>
                <h2>{ "Transaction" }</h2>
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
                    <input type="text" id="sending-account" />
                </div>

                <div class="input-group">
                    <label for="receiving-account">{
                        "Receiving Account"
                    }</label>
                    <input type="text" id="receiving-account" />
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

                <div class="input-group">
                    <label for="budget">{ "Budget" }</label>
                    <select id="budget">
                    </select>
                </div>

            </form>
        }
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
