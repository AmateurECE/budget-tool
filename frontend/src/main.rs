///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Frontend entrypoint.
//
// CREATED:         04/14/2022
//
// LAST EDITED:     04/24/2022
////

use budget_models::models::{PeriodicBudget};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

mod fetch;
mod pages;

use fetch::fetch;
use pages::{PeriodicBudgetView, NotFoundView};

const PERIODIC_BUDGETS: &'static str = "/api/periodic_budgets";

// The Different routes we support
#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/periodic_budgets/:id")]
    PeriodicBudget{ id: u64 },
    #[not_found]
    #[at("/404")]
    NotFound,
}

///////////////////////////////////////////////////////////////////////////////
// BudgetApp
////

pub enum AppMessage {
    Received(Vec<PeriodicBudget>),
}

#[derive(Default)]
pub struct BudgetApp {
    selected_budget: u64,
    budgets: usize,
}

impl Component for BudgetApp {
    type Message = AppMessage;
    type Properties = ();

    fn create(context: &Context<Self>) -> Self {
        let link = context.link().callback(
            |budgets: Vec<PeriodicBudget>| AppMessage::Received(budgets)
        );
        spawn_local(async move {
            let request = http::Request::builder()
                .uri(PERIODIC_BUDGETS)
                .header("Accept", "application/json")
                .body(())
                .unwrap();
            let response: http::Response<Vec<PeriodicBudget>> = fetch(request)
                .await.unwrap();
            link.emit(response.into_body());
        });

        Self::default()
    }

    fn update(&mut self, _context: &Context<Self>, message: Self::Message) ->
        bool
    {
        use AppMessage::*;
        match &message {
            Received(budgets) => {
                self.budgets = budgets.len();
                true
            },
        }
    }

    fn view(&self, _context: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send
        // messages, etc to the component.
        html! {
            <BrowserRouter>
                { self.view_nav() }

                <main>
                    <Switch<Route>
                        render={Switch::render(BudgetApp::switch)} />
                </main>
            </BrowserRouter>
        }
    }
}

impl BudgetApp {
    fn view_nav(&self) -> Html {
        html! {
            <ul>
                <li>{ self.budgets }</li>
                <li>
                    <Link<Route>
                        to={Route::PeriodicBudget{id: self.selected_budget}}>
                        { "Budget" }
                    </Link<Route>>
                </li>
                <li>
                    <Link<Route> to={Route::NotFound}>
                        { "Not Found" }
                    </Link<Route>>
                </li>
            </ul>
        }
    }

    fn switch(routes: &Route) -> Html {
        match routes.clone() {
            Route::PeriodicBudget{id} => {
                html! { <PeriodicBudgetView {id} /> }
            },

            _ => {
                html! { <NotFoundView /> }
            },
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Main
////

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<BudgetApp>::new().render();
}

///////////////////////////////////////////////////////////////////////////////
