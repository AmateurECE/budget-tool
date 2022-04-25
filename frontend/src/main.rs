///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Frontend entrypoint.
//
// CREATED:         04/14/2022
//
// LAST EDITED:     04/25/2022
////

use std::str::FromStr;

use budget_models::models::{PeriodicBudget};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew_router::prelude::*;

mod network;
mod pages;

use network::fetch;
use pages::{PeriodicBudgetView, NotFoundView};

const PERIODIC_BUDGETS: &'static str = "/api/periodic_budgets";

// The Different routes we support
#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
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
    Selected(u64),
}

#[derive(Default)]
pub struct BudgetApp {
    selected_budget: u64,
    budgets: Option<Vec<PeriodicBudget>>,
}

impl Component for BudgetApp {
    type Message = AppMessage;
    type Properties = ();

    fn create(context: &Context<Self>) -> Self {
        let link = context.link().callback(
            |budgets: Vec<PeriodicBudget>| AppMessage::Received(budgets)
        );
        spawn_local(async move {
            let request = web_sys::Request::new_with_str(PERIODIC_BUDGETS)
                .unwrap();
            let response: Vec<PeriodicBudget> = fetch(request).await.unwrap();
            link.emit(response);
        });

        Self::default()
    }

    fn update(&mut self, _context: &Context<Self>, message: Self::Message) ->
        bool
    {
        use AppMessage::*;
        match message {
            Received(budgets) => {
                self.budgets = Some(budgets);
                true
            },

            Selected(id) => {
                self.selected_budget = id;
                true
            },
        }
    }

    fn view(&self, context: &Context<Self>) -> Html {
        match &self.budgets {
            Some(_) => self.render(context),
            None => html!{ "Loading..." },
        }
    }
}

impl BudgetApp {
    fn render(&self, context: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(context) }
                <main>
                    <Switch<Route>
                        render={Switch::render(BudgetApp::switch)} />
                </main>
            </BrowserRouter>
        }
    }

    fn view_nav(&self, context: &Context<Self>) -> Html {
        html! {
            <div>
                <ul><li>
                    <Link<Route>
                        to={Route::PeriodicBudget{id: self.selected_budget}}>
                        { "Budget" }
                    </Link<Route>>
                </li></ul>

                <label for="budget-select">{"Budget"}</label>
                <select name="budgets" id="budget-select"
                    onchange={context.link().batch_callback(|e: Event| {
                        if let Some(select) = e.target_dyn_into::<
                                HtmlSelectElement>() {
                            Some(AppMessage::Selected(
                                u64::from_str(&select.value()).unwrap()))
                        } else {
                            None
                        }
                    })}>{
                    self.budgets.as_ref().unwrap().iter()
                        .map(|b| {html!{
                            <option value={b.id.to_string()}>
                                {b.start_date.to_string()}
                            </option>
                        }})
                        .collect::<Html>()
                }</select>
            </div>
        }
    }

    fn switch(routes: &Route) -> Html {
        match routes.clone() {
            Route::Home => {
                html! { "" }
            },

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
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<BudgetApp>::new().render();
}

///////////////////////////////////////////////////////////////////////////////
