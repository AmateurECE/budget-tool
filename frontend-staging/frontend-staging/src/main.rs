///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the wasm application
//
// CREATED:         10/07/2022
//
// LAST EDITED:     10/07/2022
////

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TableProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
fn Table(props: &TableProps) -> Html {
    html! {
        <ul>{
            for props.children.iter().map(|child| html! { <li>{ child }</li> })
        }</ul>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <Table>
            <p>{"Some text"}</p>
            <p>{"Other text"}</p>
        </Table>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

///////////////////////////////////////////////////////////////////////////////
