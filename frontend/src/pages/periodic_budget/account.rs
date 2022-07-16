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
// LAST EDITED:     07/15/2022
////

use yew::prelude::*;
use crate::render::{Render, RenderTable};

///////////////////////////////////////////////////////////////////////////////
// AccountView
////

pub(super) struct AccountView;

impl Render for AccountView {
    fn render(&self) -> Html {
        // let initial = format!(
        //     "{:.2}", (self.0.initial_balance as f64) / 100.0);
        // let current = format!(
        //     "{:.2}", (self.0.current_balance as f64) / 100.0);
        // let expected_end = format!(
        //     "{:.2}", (self.0.expected_end_balance as f64) / 100.0);
        // html! {<tr><td data-label="Name">{
        //     &self.0.account.name
        // }</td><td data-label="Initial Balance">{
        //     initial
        // }</td><td data-label="Current Balance">{
        //     current
        // }</td><td data-label="Expected End Balance">{
        //     expected_end
        // }</td></tr>}
        todo!()
    }
}

impl RenderTable for AccountView {
    fn header() -> Html {
        // html! {<tr><th>{
        //     "Name"
        // }</th><th>{
        //     "Initial Balance"
        // }</th><th>{
        //     "Current Balance"
        // }</th><th>{
        //     "Expected End Balance"
        // }</th></tr>}
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
