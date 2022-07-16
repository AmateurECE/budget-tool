///////////////////////////////////////////////////////////////////////////////
// NAME:            budget_item.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Simple "component" to render a budget item under the
//                  Periodic Budget View.
//
// CREATED:         07/15/2022
//
// LAST EDITED:     07/16/2022
////

use budget_models::{money::Money, models::BudgetItem};
use yew::prelude::*;
use crate::render::{Render, RenderTable};

///////////////////////////////////////////////////////////////////////////////
// BudgetItemView
////

pub(super) struct BudgetItemView {
    item: BudgetItem,
    spent: Money,
}

impl Render for BudgetItemView {
    fn render(&self) -> Html {
        let budgeted: Money = self.item.budgeted.into();
        html! {<tr><td data-label="Description">{
            &self.item.description
        }</td><td data-label="Transaction Type">{
            &self.item.transaction_type.to_string()
        }</td><td data-label="From">{
            self.item.from_account.as_ref().map(|a| a.as_str()).unwrap_or("")
        }</td><td data-label="To">{
            self.item.from_account.as_ref().map(|a| a.as_str()).unwrap_or("")
        }</td><td data-label="Budgeted">{
            &budgeted.to_string()
        }</td><td data-label="Spent">{
            &self.spent.to_string()
        }</td></tr>}
    }
}

impl RenderTable for BudgetItemView {
    fn header() -> Html {
        html! {<tr><th>{
            "Description"
        }</th><th>{
            "Transaction Type"
        }</th><th>{
            "From"
        }</th><th>{
            "To"
        }</th><th>{
            "Budgeted"
        }</th><th>{
            "Spent"
        }</th></tr>}
    }
}

///////////////////////////////////////////////////////////////////////////////
