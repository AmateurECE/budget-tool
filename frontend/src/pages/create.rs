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

use budget_models::models::Transaction;

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
            <h2>{ "Transaction" }</h2>
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
