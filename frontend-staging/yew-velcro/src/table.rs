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

use crate::fields::{Fields, FieldNames, FieldView};
use yew::prelude::*;

///////////////////////////////////////////////////////////////////////////////
// TableRow
////

#[derive(Properties, PartialEq)]
struct TableRowProps {
    data: FieldView,
}

#[function_component]
fn TableRow(props: &TableRowProps) -> Html {
    html! {
        <tr>{props.data.iter().map(|item| html! { <td>{&item}</td> })
             .collect::<Html>()}</tr>
    }
}

///////////////////////////////////////////////////////////////////////////////
// Table
////

#[derive(Properties, PartialEq, Eq)]
pub struct TableProps<T>
where T: Fields + FieldNames + PartialEq,
{
    pub row_data: Vec<T>,
}

#[function_component]
pub fn Table<T>(props: &TableProps<T>) -> Html
where T: Fields + FieldNames + PartialEq,
{
    let field_views = props.row_data.iter().map(|object| object.fields())
        .collect::<Vec<FieldView>>();
    html! {
        <table>
            <th>{ for T::field_names().iter().map(|header| html!{
                <td>{&header}</td>
            })}</th>{
                for field_views.iter().map(|child| html! {
                    <TableRow data={child.clone()} />
                })
            }
        </table>
    }
}

///////////////////////////////////////////////////////////////////////////////
