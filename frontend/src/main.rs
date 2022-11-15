///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the wasm application
//
// CREATED:         10/07/2022
//
// LAST EDITED:     11/15/2022
//
// Copyright 2022, Ethan D. Twardy
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
////

use strum_macros::EnumIter;
use yew::prelude::*;
use yew_router::prelude::*;

mod navigation;
mod performance;
mod view;

///////////////////////////////////////////////////////////////////////////////
// Header
////

#[function_component]
fn Header() -> Html {
    html! {
        <header class={classes!("navbar", "navbar-dark", "sticky-top",
                                "bg-dark", "flex-md-nowrap", "p-0", "shadow")}>
            <a class={classes!("navbar-brand", "col-md-3", "col-lg-2", "me-0",
                               "px-3")} href={"#"}>{
                "Budgetizer"
            }</a>
        </header>
    }
}

///////////////////////////////////////////////////////////////////////////////
// Routing
////

#[derive(Copy, Routable, PartialEq, Clone, Debug, EnumIter)]
enum Route {
    #[at("/spending_history")]
    SpendingHistory,
    #[at("/balance_history")]
    BalanceHistory,
}

impl ToString for Route {
    fn to_string(&self) -> String {
        match self {
            Route::SpendingHistory => "Spending History".to_string(),
            Route::BalanceHistory => "Account Balance History".to_string(),
        }
    }
}

fn app_switch(route: Route) -> Html {
    match route {
        Route::SpendingHistory => html! { <performance::SpendingHistory /> },
        Route::BalanceHistory => html! { <performance::BalanceHistory /> },
    }
}

///////////////////////////////////////////////////////////////////////////////
// Main
////

#[function_component]
fn Main() -> Html {
    html! {
        <BrowserRouter>
            <navigation::Navigation<Route> />
            <main class={classes!("col-md-9", "ms-sm-auto", "col-lg-10",
                                  "px-md-4")} role={"main"}>
                <Switch<Route> render={app_switch} />
            </main>
        </BrowserRouter>
    }
}

///////////////////////////////////////////////////////////////////////////////
// App
////

#[function_component]
fn App() -> Html {
    html! {
        <>
            <Header />
            <div class={classes!("container-fluid")}>
                <div class={classes!("row")}>
                    <Main />
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

///////////////////////////////////////////////////////////////////////////////
