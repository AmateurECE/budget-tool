///////////////////////////////////////////////////////////////////////////////
// NAME:            navigation.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Components related to navigation elements on the UI.
//
// CREATED:         10/19/2022
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
        <li class={classes!("nav-item", "py-1")}>
            <Link<R> to={props.to}>
                <span class={classes!("text-muted")}>{ &props.label }</span>
            </Link<R>>
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
