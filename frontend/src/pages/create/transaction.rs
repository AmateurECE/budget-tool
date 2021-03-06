///////////////////////////////////////////////////////////////////////////////
// NAME:            transaction.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Creation form for new transactions.
//
// CREATED:         07/07/2022
//
// LAST EDITED:     07/10/2022
////

use std::str::FromStr;
use std::rc::Rc;

use budget_models::models::{
    Transaction, TransactionType, PeriodicBudget, NewTransaction,
    PeriodicBudgetSummary,
};
use chrono::{naive::NaiveDate, TimeZone, Utc};
use strum::IntoEnumIterator;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::network::fetch;
use crate::{PERIODIC_BUDGETS_PATH, TRANSACTIONS_PATH};

#[derive(Properties, Clone, PartialEq)]
pub struct TransactionFormProperties {
    pub data: Rc<Transaction>,
}

pub enum TransactionFormMessage {
    BudgetSelected(String),
    ReceivedBudgets(Vec<PeriodicBudget>),
    ReceivedOneBudget(PeriodicBudgetSummary),
    Submitted,
    SubmitResponseReceived(Transaction),
    DateUpdated(String),
    TransactionTypeChanged(TransactionType),
}

#[derive(Default)]
pub struct TransactionForm {
    budget_data: Option<PeriodicBudgetSummary>,
    budgets: Option<Vec<PeriodicBudget>>,
    response_message: String,
    date_mirror: String,

    // NewTransaction data (form state)
    budget_id: i32,
    description: NodeRef,
    line_item: NodeRef,
    transaction_type: TransactionType,
    sending_account: NodeRef,
    receiving_account: NodeRef,
    transfer_fees: NodeRef,
    receiving_entity: NodeRef,
    amount: NodeRef,
    send_date: NodeRef,
    receive_date: NodeRef,
    corrects: NodeRef,
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
        if let BudgetSelected(id) = message {
            self.budget_id = i32::from_str(&id).unwrap();
            self.request_one_budget(self.budget_id, context);
            true
        }

        else if let ReceivedBudgets(budgets) = message {
            self.budgets = Some(budgets);
            true
        }

        else if let ReceivedOneBudget(budget) = message {
            self.budget_data = Some(budget);
            true
        }

        else if let Submitted = message {
            let link = context.link().callback(|t: Transaction| {
                SubmitResponseReceived(t)
            });

            let new_transaction = self.validate_new_transaction();
            web_sys::console::log_1(&format!("{:?}", new_transaction).into());
            spawn_local(async move {
                let headers = web_sys::Headers::new().unwrap();
                headers.set("Content-Type", "application/json").unwrap();

                let mut request_init = web_sys::RequestInit::new();
                request_init.method("POST");
                request_init.headers(&headers.into());
                request_init.body(
                    Some(&serde_json::to_string(&new_transaction).unwrap()
                         .into()));
                let request = web_sys::Request::new_with_str_and_init(
                    TRANSACTIONS_PATH, &request_init).unwrap();
                let transaction: Transaction = fetch(request).await.unwrap();
                link.emit(transaction);
            });
            true
        }

        else if let SubmitResponseReceived(transaction) = message {
            self.response_message = "Successfully Added Transaction '"
                .to_string() + &transaction.description + "'";
            self.budget_data.as_mut().unwrap().transactions.push(transaction);

            // Re-focus the first element in the form upon submit
            self.description.cast::<HtmlInputElement>().unwrap().focus()
                .unwrap();
            true
        }

        else if let DateUpdated(new_date) = message {
            self.date_mirror = new_date;
            true
        }

        else if let TransactionTypeChanged(new_type) = message {
            self.transaction_type = new_type;
            true
        }

        else {
            false
        }
    }

    fn view(&self, context: &Context<Self>) -> Html {
        use TransactionFormMessage::*;
        let accounts: Vec<String> = match &self.budget_data {
            Some(data) => data.initial_balances.iter()
                .map(|a| a.account.to_owned())
                .collect(),
            None => Vec::new(),
        };

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
                     })}>
                        <option value="">{"--Unselected--"}</option>
                    {
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
                    <input type="text" id="description"
                     ref={self.description.clone()} />
                </div>

                <div class="input-group">
                    <label for="line-item">{ "Line Item" }</label>
                    <select name="line-item" ref={self.line_item.clone()}>
                        <option value="">{"--Unselected--"}</option>
                    {
                        if let Some(budget) = &self.budget_data {
                            budget.items.iter().map(|i| {
                                html! {
                                    <option value={i.id.to_string()}>{
                                        &i.description
                                    }</option>
                                }
                            }).collect::<Html>()
                        } else {
                            html! {<option value="">{ "Loading..." }</option>}
                        }
                    }</select>
                </div>

                <div class="input-group">
                    <label for="transaction-type">{
                        "Transaction Type"
                    }</label>
                    <select id="transaction-type"
                     onchange={context.link().batch_callback(|e: Event| {
                         if let Some(select) = e
                             .target_dyn_into::<HtmlSelectElement>()
                         {
                             Some(TransactionTypeChanged(
                                 select.value().try_into().unwrap()))
                         } else {
                             None
                         }
                     })}>{
                        TransactionType::iter().map(|t_type| {
                            let t_type = t_type.to_string();
                            html! {
                                <option value={t_type.clone()}>{
                                    t_type
                                }</option>
                            }
                        }).collect::<Html>()
                    }</select>
                </div>

            if self.transaction_type != TransactionType::Income {
                <div class="input-group">
                    <label for="sending-account">{ "Sending Account" }</label>
                    <select id="sending-account"
                     ref={self.sending_account.clone()}>
                        <option value="">{"--Unselected--"}</option>
                    {
                        accounts.iter().map(|acct| {
                            html! {
                                <option value={acct.clone()}>{ acct }</option>
                            }
                        }).collect::<Html>()
                    }</select>
                </div>
            }

            if self.transaction_type != TransactionType::Expense {
                <div class="input-group">
                    <label for="receiving-account">{
                        "Receiving Account"
                    }</label>
                    <select id="receiving-account"
                     ref={self.receiving_account.clone()}>
                        <option value="">{"--Unselected--"}</option>
                    {
                        accounts.iter().map(|acct| {
                            html! {
                                <option value={acct.clone()}>{ acct }</option>
                            }
                        }).collect::<Html>()
                    }</select>
                </div>
            }

                <div class="input-group">
                    <label for="amount">{ "Amount" }</label>
                    <input type="money" id="amount"
                     ref={self.amount.clone()} />
                </div>

                <div class="input-group">
                    <label for="transfer-fees">{ "Transfer Fees" }</label>
                    <input type="money" id="transfer-fees"
                     ref={self.transfer_fees.clone()} />
                </div>

                <div class="input-group">
                    <label for="receiving-entity">{
                        "Receiving Entity"
                    }</label>
                    <input type="text" id="receiving-entity"
                     ref={self.receiving_entity.clone()} />
                </div>

                <div class="input-group">
                    <label for="send-date">{ "Send Date" }</label>
                    <input type="date" id="send-date"
                     ref={self.send_date.clone()}
                     onblur={context.link().batch_callback(|e: FocusEvent| {
                         if let Some(input) = e.target_dyn_into::<
                                 HtmlInputElement>() {
                             Some(DateUpdated(input.value()))
                         } else {
                             None
                         }
                     })} />
                </div>

                <div class="input-group">
                    <label for="receive-date">{ "Receive Date" }</label>
                    <input type="date" id="receive-date"
                     ref={self.receive_date.clone()}
                     value={self.date_mirror.clone()} />
                </div>

                <div class="input-group">
                    <label for="corrects">{ "Corrects Transaction" }</label>
                    <select id="corrects" ref={self.corrects.clone()}>
                        <option value="">{"--Unselected--"}</option>
                    {
                        if let Some(data) = &self.budget_data {
                            data.transactions.iter().map(|t| {
                                html! {
                                    <option value={t.id.to_string()}>{
                                        t.description.clone() + " ("
                                            + &t.send_date.format("%m-%d")
                                            .to_string() + ")"
                                    }</option>
                                }
                            }).collect::<Html>()
                        } else {
                            html! {<option value="">{ "Loading..." }</option>}
                        }
                    }</select>
                </div>

                <div class="input-group">
                    <button onclick={context.link().callback(|e: MouseEvent| {
                        e.prevent_default();
                        TransactionFormMessage::Submitted
                    })}>{ "Submit" }</button>
                </div>

                <div class="response-message">{ &self.response_message }</div>

            </form>
        }
    }
}

impl TransactionForm {
    fn validate_new_transaction(&self) -> NewTransaction {
        let periodic_budget = self.budget_id;
        let description = self.description.cast::<HtmlInputElement>().unwrap()
            .value();
        let line_item = self.line_item.cast::<HtmlSelectElement>().unwrap()
            .value().as_str().parse().unwrap();
        let transaction_type = self.transaction_type;

        let sending_account = match self.sending_account
            .cast::<HtmlSelectElement>() {
                Some(select) => match select.value().as_str() {
                    "" => None,
                    value => Some(value.to_string()),
                },
                None => None,
            };
        let receiving_account = match self.receiving_account
            .cast::<HtmlSelectElement>() {
                Some(select) => match select.value().as_str() {
                    "" => None,
                    value => Some(value.to_string()),
                },
                None => None,
            };

        let transfer_fees = match self.transfer_fees
            .cast::<HtmlInputElement>().unwrap().value().as_str() {
                "" => None,
                value => Some((value.parse::<f32>().unwrap() * 100.0) as i64),
            };
        let receiving_entity = match self.receiving_entity
            .cast::<HtmlInputElement>().unwrap().value().as_str() {
                "" => None,
                value => Some(value.to_string()),
            };

        let amount = (self.amount.cast::<HtmlInputElement>().unwrap().value()
                      .parse::<f32>().unwrap() * 100.0) as i64;

        const DATE_FORMAT: &'static str = "%Y-%m-%d";
        let send_date = Utc.from_utc_datetime(
            &NaiveDate::parse_from_str(
                &self.send_date.cast::<HtmlInputElement>().unwrap().value(),
                DATE_FORMAT
            ).unwrap().and_hms(0, 0, 0)
        );
        let receive_date = match self.receive_date
            .cast::<HtmlInputElement>().unwrap().value().as_str() {
                "" => None,
                value => Some(
                    Utc.from_utc_datetime(
                        &NaiveDate::parse_from_str(&value, DATE_FORMAT)
                            .unwrap()
                            .and_hms(0, 0, 0)
                    )
                ),
            };
        let corrects = match self.corrects
            .cast::<HtmlInputElement>().unwrap().value().as_str() {
                "" => None,
                value => Some(
                    value.split(",").map(|s| s.parse().unwrap())
                        .collect::<Vec<i32>>()
                ),
            };

        NewTransaction {
            periodic_budget, description, line_item, transaction_type,
            sending_account, receiving_account, transfer_fees,
            receiving_entity, amount, send_date, receive_date, corrects,
        }
    }

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
            |budget: PeriodicBudgetSummary| ReceivedOneBudget(budget)
        );
        let id = id.to_string();
        spawn_local(async move {
            let url = PERIODIC_BUDGETS_PATH.to_string() + "/" + &id;
            let request = web_sys::Request::new_with_str(&url).unwrap();
            let budget: PeriodicBudgetSummary = fetch(request).await.unwrap();
            link.emit(budget);
        })
    }
}

///////////////////////////////////////////////////////////////////////////////
