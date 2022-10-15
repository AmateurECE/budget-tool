///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the wasm application
//
// CREATED:         10/07/2022
//
// LAST EDITED:     10/15/2022
////

use yew::prelude::*;
use yew_router::prelude::*;

mod performance;

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

#[derive(Routable, PartialEq, Clone, Debug)]
enum Route {
    #[at("/spending_history")]
    SpendingHistory,
}

fn app_switch(route: Route) -> Html {
    match route {
        Route::SpendingHistory => html! { <performance::SpendingHistory /> },
    }
}

///////////////////////////////////////////////////////////////////////////////
// Navigation
////

#[function_component]
fn Navigation() -> Html {
    html! {
        <nav class={classes!("col-md-3", "col-lg-2", "d-md-block", "bg-light",
                             "sidebar", "collapse")} id={"sidebarMenu"}>
            <div class={classes!("position-sticky", "pt-3", "ps-3")}>
                <ul class={classes!("nav", "flex-column")}>
                    <li class={classes!("nav-item")}>
                        <Link<Route> to={Route::SpendingHistory}>{
                            "Spending History"
                        }</Link<Route>>
                    </li>
                </ul>
            </div>
        </nav>
    }
}

///////////////////////////////////////////////////////////////////////////////
// Main
////

#[function_component]
fn Main() -> Html {
    html! {
        <BrowserRouter>
            <Navigation />
            <main class={classes!("col-md-9", "ms-sm-auto", "col-lg-10",
                                  "px-md-4")} role={"main"}>
                <div class={classes!(
                    "d-flex", "justify-content-between", "flex-wrap",
                    "flex-md-nowrap", "align-items-center", "pt-3", "pb-2",
                    "mb-3", "border-bottom")}>
                    <h1 class={classes!("h2")}>{ "Budget Performance" }</h1>
                </div>
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
            <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/js/bootstrap.bundle.min.js" integrity="sha384-MrcW6ZMFYlzcLA8Nl+NtUVF0sA7MsXsP1UyJoMp4YLEuNSfAP+JcXn/tWtIaxVXM" crossorigin="anonymous"></script>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

///////////////////////////////////////////////////////////////////////////////
