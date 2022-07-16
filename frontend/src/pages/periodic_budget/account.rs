///////////////////////////////////////////////////////////////////////////////
// NAME:            account.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     A simple "component" to render accounts under the Periodic
//                  Budget view.
//
// CREATED:         07/15/2022
//
// LAST EDITED:     07/16/2022
////

use budget_models::money::Money;
use yew::prelude::*;
use crate::render::{Render, RenderTable};

///////////////////////////////////////////////////////////////////////////////
// AccountView
////

pub(super) struct AccountView {
    initial_balance: Money,
    current_balance: Money,
    expected_end_balance: Money,
    account_name: String,
}

impl Render for AccountView {
    fn render(&self) -> Html {
        html! {<tr><td data-label="Name">{
            &self.account_name
        }</td><td data-label="Initial Balance">{
            &self.initial_balance.to_string()
        }</td><td data-label="Current Balance">{
            &self.current_balance.to_string()
        }</td><td data-label="Expected End Balance">{
            &self.expected_end_balance.to_string()
        }</td></tr>}
    }
}

impl RenderTable for AccountView {
    fn header() -> Html {
        html! {<tr><th>{
            "Name"
        }</th><th>{
            "Initial Balance"
        }</th><th>{
            "Current Balance"
        }</th><th>{
            "Expected End Balance"
        }</th></tr>}
    }
}

///////////////////////////////////////////////////////////////////////////////
