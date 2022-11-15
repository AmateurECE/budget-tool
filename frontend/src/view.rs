///////////////////////////////////////////////////////////////////////////////
// NAME:            view.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Components for assisting in the creation of views.
//
// CREATED:         10/20/2022
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
