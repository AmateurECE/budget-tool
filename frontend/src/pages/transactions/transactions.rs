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

use budget_models::{models, money::Money};
use yew::prelude::*;
use crate::render::{Render, RenderTable};

///////////////////////////////////////////////////////////////////////////////
// TransactionView
////

pub struct TransactionView {
    transaction: models::Transaction,
    sending_new_balance: Money,
    receiving_new_balance: Money,
    line_item_name: String,
}

impl TransactionView {
    pub fn new(
        transaction: models::Transaction, sending_new_balance: Money,
        receiving_new_balance: Money, line_item_name: String,
    ) -> Self {
        Self {
            transaction,
            sending_new_balance,
            receiving_new_balance,
            line_item_name,
        }
    }
}

impl Render for TransactionView {
    fn render(&self) -> Html {
        html! {<tr><td>{
            self.transaction.id
        }</td><td>{
            &self.transaction.send_date.format("%m-%d").to_string()
        }</td><td>{
            &self.transaction.description
        }</td><td>{
            &self.line_item_name
        }</td><td>{
            self.transaction.sending_account.as_ref()
                .map(|account|
                     format!("{} / {}", account.as_str(),
                             self.sending_new_balance)
                ).as_ref()
                .map(|account| account.as_str())
                .unwrap_or("")
        }</td><td>{
            self.transaction.receiving_account.as_ref()
                .map(|account|
                     format!("{} / {}", account.as_str(),
                             self.receiving_new_balance)
                ).as_ref()
                .map(|account| account.as_str())
                .unwrap_or("")
        }</td><td>{
            &Money::from(self.transaction.amount).to_string()
        }</td><td>{
            self.transaction.transfer_fees
                .map(|fees| Money::from(fees).to_string())
                .as_ref()
                .map(|fees| fees.as_str())
                .unwrap_or("")
        }</td><td>{
            self.transaction.corrects.as_ref()
                .map(|corrects| format!("{:?}", corrects))
                .as_ref()
                .map(|corrects| corrects.as_str())
                .unwrap_or("")
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
