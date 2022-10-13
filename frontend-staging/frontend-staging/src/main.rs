///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the wasm application
//
// CREATED:         10/07/2022
//
// LAST EDITED:     10/13/2022
////

use yew::prelude::*;
use yew_roots::{table::Table, FieldNames, Fields};

///////////////////////////////////////////////////////////////////////////////
// Navigation
////

#[function_component]
fn Navigation() -> Html {
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
// App
////

#[derive(Clone, PartialEq, Properties, Fields, FieldNames)]
struct SomeObject {
    #[field_name(rename = "Foo")]
    pub foo: String,
    pub bar: String,
}

#[function_component]
fn App() -> Html {
    let objects = vec![
        SomeObject {
            foo: "a".to_string(),
            bar: "b".to_string(),
        },
        SomeObject {
            foo: "c".to_string(),
            bar: "d".to_string(),
        },
    ];

    html! {
        <>
            <Navigation />
            <main class={classes!("container-fluid")} role={"main"}>
                <Table<SomeObject> class={classes!(
                    "table", "table-striped", "table-hover", "table-sm")}
                row_data={objects} />
            </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

///////////////////////////////////////////////////////////////////////////////
