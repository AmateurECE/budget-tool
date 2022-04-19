///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Frontend entrypoint.
//
// CREATED:         04/14/2022
//
// LAST EDITED:     04/19/2022
////

use budget_models::models::{PeriodicBudget};
use yew::prelude::*;
use yew::{
    format::{Json, Nothing},
    services::fetch::{FetchService, FetchTask, Request, Response},
};

const PERIODIC_BUDGETS: &'static str = "/api/periodic_budgets";

pub enum Msg {
    GetBudgets,
    ReceiveResponse(Result<Vec<PeriodicBudget>, anyhow::Error>),
}

pub struct BudgetService {
    fetch_task: Option<FetchTask>,
    budgets: Option<Vec<PeriodicBudget>>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Component for BudgetService {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            budgets: None,
            link,
            error: None,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn update(&mut self, message: Self::Message) -> bool {
        match message {
            Msg::GetBudgets => {
                let request = Request::get(PERIODIC_BUDGETS)
                    .body(Nothing)
                    .expect("Couldn't build request!");

                let callback = self.link
                    .callback(
                        |response: Response<Json<Result<Vec<PeriodicBudget>,
                                                        anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::ReceiveResponse(data)
                        });

                let task = FetchService::fetch(request, callback)
                    .expect("Failed to start request!");
                self.fetch_task = Some(task);
                true
            },

            Msg::ReceiveResponse(response) => {
                self.budgets = Some(response.unwrap());
                self.fetch_task = None;
                true
            }
        }
    }

    fn view(&self) -> Html {
        // This gives us a component's "`Scope`" which allows us to send
        // messages, etc to the component.
        html! {
            <div>
                <button onclick={self.link.callback(|_| Msg::GetBudgets)}>
                    { "Get Budgets" }
                </button>
                <p>{
                    match &self.budgets {
                        Some(budgets) => budgets.len().to_string(),
                        None => String::new(),
                    }
                }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<BudgetService>();
}

///////////////////////////////////////////////////////////////////////////////
