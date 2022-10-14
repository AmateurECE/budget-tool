///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the wasm application
//
// CREATED:         10/07/2022
//
// LAST EDITED:     10/14/2022
////

use yew::prelude::*;
use yew_roots::{table::Table, FieldNames, Fields};

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
// Navigation
////

#[function_component]
fn Navigation() -> Html {
    html! {
        <nav class={classes!("col-md-3", "col-lg-2", "d-md-block", "bg-light",
                             "sidebar", "collapse")}>
            <div class={classes!("position-sticky", "pt-3")}>
                <ul class={classes!("nav", "flex-column")}>
                    <li class={classes!("nav-item")}>{
                        "Budget Performance"
                    }</li>
                </ul>
            </div>
        </nav>
    }
}

///////////////////////////////////////////////////////////////////////////////
// Main
////

#[derive(Clone, Default, PartialEq, Fields, FieldNames)]
struct LineItem {
    #[field_name(rename = "Name")]
    pub name: String,
    #[field_name(rename = "Last Month (Budgeted)")]
    pub last_month_budgeted: i64,
    #[field_name(rename = "Last Month (Spent)")]
    pub last_month_spent: i64,
    #[field_name(rename = "Last Six Months (Average Spent)")]
    pub last_six_months_spent: i64,
}

#[function_component]
fn Main() -> Html {
    let objects = vec![
        LineItem::default(),
        LineItem::default(),
        LineItem::default(),
        LineItem::default(),
        LineItem::default(),
        LineItem::default(),
        LineItem::default(),
    ];

    html! {
        <main class={classes!("col-md-9", "ms-sm-auto", "col-lg-10",
                              "px-md-4")} role={"main"}>
            <div class={classes!(
                "d-flex", "justify-content-between", "flex-wrap",
                "flex-md-nowrap", "align-items-center", "pt-3", "pb-2", "mb-3",
                "border-bottom")}>
                <h1 class={classes!("h2")}>{ "Budget Performance" }</h1>
            </div>
            <h2>{ "Spending History By Line Item" }</h2>
            <Table<LineItem> class={classes!(
                "table", "table-striped", "table-hover", "table-responsive",
                "table-sm")}
             row_data={objects} />
        </main>
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
                    <Navigation />
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
