///////////////////////////////////////////////////////////////////////////////
// NAME:            periodic_budget_view.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Page for view of the Periodic Budget
//
// CREATED:         04/20/2022
//
// LAST EDITED:     04/25/2022
////

use yew::{Component, Context, Html, Properties, html};

#[derive(Properties, PartialEq)]
pub struct PeriodicBudgetViewProperties {
    pub id: i32,
}

pub struct PeriodicBudgetView;

impl Component for PeriodicBudgetView {
    type Message = ();
    type Properties = PeriodicBudgetViewProperties;

    fn create(_context: &Context<Self>) -> Self {
        PeriodicBudgetView
    }

    fn view(&self, _context: &Context<Self>) -> Html {
        html! { <p>{ "Periodic Budget View" }</p> }
    }
}

///////////////////////////////////////////////////////////////////////////////
