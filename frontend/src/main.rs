///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Frontend entrypoint.
//
// CREATED:         04/14/2022
//
// LAST EDITED:     04/21/2022
////

use budget_models::models::{PeriodicBudget};
use yew::prelude::*;
use yew_router::prelude::*;

mod pages;

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

pub struct BudgetApp {
    selected_budget: u64,
}

impl Component for BudgetApp {
    type Message = ();
    type Properties = ();

    fn create(_context: &Context<Self>) -> Self {
        Self {
            selected_budget: 0,
        }
    }

    fn update(&mut self, _context: &Context<Self>, _props: Self::Properties) ->
        bool
    { false }

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
