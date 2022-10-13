///////////////////////////////////////////////////////////////////////////////
// NAME:            table.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Yew components for rendering HTML tables.
//
// CREATED:         10/12/2022
//
// LAST EDITED:     10/12/2022
////

use crate::fields::{FieldNames, FieldSpec, FieldView, Fields};
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
        <table class="table">
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
