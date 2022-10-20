///////////////////////////////////////////////////////////////////////////////
// NAME:            navigation.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Components related to navigation elements on the UI.
//
// CREATED:         10/19/2022
//
// LAST EDITED:     10/19/2022
////

use strum::IntoEnumIterator;
use yew::prelude::*;
use yew_router::prelude::*;

///////////////////////////////////////////////////////////////////////////////
// ViewLink
////

#[derive(Clone, Properties, PartialEq)]
struct ViewLinkProps<R>
where R: Routable + Copy + 'static,
{
    to: R,
    label: String,
}

#[function_component]
fn ViewLink<R>(props: &ViewLinkProps<R>) -> Html
where R: Routable + Copy + 'static,
{
    html! {
        <li class={classes!("nav-item")}>
            <Link<R> to={props.to}>{ &props.label }</Link<R>>
        </li>
    }
}

///////////////////////////////////////////////////////////////////////////////
// Navigation
////

#[function_component]
pub fn Navigation<R>() -> Html
where R: Routable + IntoEnumIterator + ToString + Copy + 'static,
{
    html! {
        <nav class={classes!("col-md-3", "col-lg-2", "d-md-block", "bg-light",
                             "sidebar", "collapse")} id={"sidebarMenu"}>
            <div class={classes!("position-sticky", "pt-3", "ps-3")}>
                <ul class={classes!("nav", "flex-column")}>{
                    R::iter().map(|elem| html! {
                        <ViewLink<R> to={elem} label={ elem.to_string() } />
                    }).collect::<Html>()
                }</ul>
            </div>
        </nav>
    }
}

///////////////////////////////////////////////////////////////////////////////
