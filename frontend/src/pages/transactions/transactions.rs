///////////////////////////////////////////////////////////////////////////////
// NAME:            transactions.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Data component for the TransactionView.
//
// CREATED:         07/17/2022
//
// LAST EDITED:     07/17/2022
////

use budget_models::models;
use yew::prelude::*;
use crate::render::{Render, RenderTable};

///////////////////////////////////////////////////////////////////////////////
// TransactionView
////

pub struct TransactionView {
    pub transaction: models::Transaction,
    pub sending_new_balance: i64,
    pub receiving_new_balance: i64,
    pub line_item_name: String,
}

impl From<models::Transaction> for TransactionView {
    fn from(transaction: models::Transaction) -> Self {
        Self {
            transaction,
            sending_new_balance: 0,
            receiving_new_balance: 0,
            line_item_name: String::new(),
        }
    }
}

impl Render for TransactionView {
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

impl RenderTable for TransactionView {
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
