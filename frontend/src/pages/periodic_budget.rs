///////////////////////////////////////////////////////////////////////////////
// NAME:            periodic_budget_view.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Page for view of the Periodic Budget
//
// CREATED:         04/20/2022
//
// LAST EDITED:     04/30/2022
////

use budget_models::entities::PeriodicBudgetEndpoint;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::PERIODIC_BUDGETS;
use crate::network::fetch;

#[derive(Properties, PartialEq)]
pub struct PeriodicBudgetViewProperties {
    pub id: i32,
}

pub enum PeriodicBudgetViewMessage {
    Received(PeriodicBudgetEndpoint),
}

#[derive(Default)]
pub struct PeriodicBudgetView {
    budget: Option<PeriodicBudgetEndpoint>,
}

impl Component for PeriodicBudgetView {
    type Message = PeriodicBudgetViewMessage;
    type Properties = PeriodicBudgetViewProperties;

    fn create(context: &Context<Self>) -> Self {
        let mut view = PeriodicBudgetView::default();
        view.request_budget(context);
        view
    }

    // Here, we abuse the changed handler to request the new budget
    fn changed(&mut self, context: &Context<Self>) -> bool {
        self.request_budget(context);
        true
    }

    fn update(&mut self, _context: &Context<Self>, message: Self::Message) ->
        bool
    {
        use PeriodicBudgetViewMessage::*;
        match message {
            Received(budget) => {
                self.budget = Some(budget);
                true
            }
        }
    }

    fn view(&self, _context: &Context<Self>) -> Html {
        match &self.budget {
            Some(budget) => self.view_budget(budget),
            None => html! { <p>{ "Loading..." }</p> },
        }
    }
}

impl PeriodicBudgetView {
    fn request_budget(&mut self, context: &Context<Self>) {
        self.budget = None;

        use PeriodicBudgetViewMessage::*;
        let link = context.link().callback(
            |budget: PeriodicBudgetEndpoint| Received(budget)
        );
        let url = PERIODIC_BUDGETS.to_string() + "/"
            + &context.props().id.to_string();
        spawn_local(async move {
            let request = web_sys::Request::new_with_str(&url)
                .unwrap();
            let response: PeriodicBudgetEndpoint = fetch(request).await
                .unwrap();
            link.emit(response);
        });
    }

    fn view_budget(&self, _budget: &PeriodicBudgetEndpoint) -> Html {
        html! {
            <p>{ "Rendered!" }</p>
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
