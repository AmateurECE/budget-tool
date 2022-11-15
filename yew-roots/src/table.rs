///////////////////////////////////////////////////////////////////////////////
// NAME:            table.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Yew components for rendering HTML tables.
//
// CREATED:         10/12/2022
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

use table_iter::prelude::*;
use yew::prelude::*;

///////////////////////////////////////////////////////////////////////////////
// TableRow
////

#[derive(Properties, PartialEq)]
struct TableRowProps {
    view: FieldView,
}

#[function_component]
fn TableRow(props: &TableRowProps) -> Html {
    html! {
        <tr>{props.view.iter().map(|item| html! { <td>{&item}</td> })
             .collect::<Html>()}</tr>
    }
}

///////////////////////////////////////////////////////////////////////////////
// HeaderRow
////

#[derive(Properties, PartialEq)]
struct HeaderRowProps {
    spec: FieldSpec,
}

#[function_component]
fn HeaderRow(props: &HeaderRowProps) -> Html {
    html! {
        <tr>{props.spec.iter().map(|header| html!{ <th>{&header}</th> })
             .collect::<Html>()}</tr>
    }
}

///////////////////////////////////////////////////////////////////////////////
// Table
////

#[derive(Properties, PartialEq, Eq)]
pub struct TableProps<T>
where
    T: Fields + FieldNames + PartialEq,
{
    pub row_data: Vec<T>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn Table<T>(props: &TableProps<T>) -> Html
where
    T: Fields + FieldNames + PartialEq,
{
    let field_views = props
        .row_data
        .iter()
        .map(|object| object.fields())
        .collect::<Vec<FieldView>>();
    html! {
        <table class={props.class.clone()}>
            <thead>
                <HeaderRow spec={T::field_names()} />
            </thead>
            <tbody>{
                for field_views.iter().map(|child| html! {
                    <TableRow view={child.clone()} />
                })
            }</tbody>
        </table>
    }
}

///////////////////////////////////////////////////////////////////////////////
