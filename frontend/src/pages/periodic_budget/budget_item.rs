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
// LAST EDITED:     07/15/2022
////

use yew::prelude::*;
use crate::render::{Render, RenderTable};

///////////////////////////////////////////////////////////////////////////////
// BudgetItemView
////

pub(super) struct BudgetItemView;

impl Render for BudgetItemView {
    fn render(&self) -> Html {
        // let budgeted = format!("{:.2}", (self.0.item.budgeted as f64) / 100.0);
        // let spent: f64 = self.0.spent.get_total().into();
        // html! { <tr><td data-label="Description">{
        //     &self.0.item.description
        // }</td><td data-label="Transaction Type">{
        //     &self.0.item.transaction_type.to_string()
        // }</td><td data-label="From">{
        //     match &self.0.item.from_account {
        //         Some(account) => account,
        //         None => "",
        //     }
        // }</td><td data-label="To">{
        //     match &self.0.item.to_account {
        //         Some(account) => account,
        //         None => "",
        //     }
        // }</td><td data-label="Budgeted">{
        //     budgeted
        // }</td><td data-label="Spent">{
        //     format!("{:.2}", &spent)
        // }</td></tr>}
        todo!()
    }
}

impl RenderTable for BudgetItemView {
    fn header() -> Html {
        // html! {<tr><th>{
        //     "Description"
        // }</th><th>{
        //     "Transaction Type"
        // }</th><th>{
        //     "From"
        // }</th><th>{
        //     "To"
        // }</th><th>{
        //     "Budgeted"
        // }</th><th>{
        //     "Spent"
        // }</th></tr>}
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
