///////////////////////////////////////////////////////////////////////////////
// NAME:            view.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Components for assisting in the creation of views.
//
// CREATED:         10/20/2022
//
// LAST EDITED:     10/20/2022
////

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ViewHeaderProps {
    pub text: String,
}

#[function_component]
pub fn ViewHeader(props: &ViewHeaderProps) -> Html {
    html! {
        <div class={classes!(
            "d-flex", "justify-content-between", "flex-wrap", "flex-md-nowrap",
            "align-items-center", "pt-3", "pb-2", "mb-3", "border-bottom")}>
            <h1 class={classes!("h2")}>{ &props.text }</h1>
        </div>
    }
}

///////////////////////////////////////////////////////////////////////////////
