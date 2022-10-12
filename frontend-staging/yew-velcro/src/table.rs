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

use crate::fields::{FieldSpec, FieldView};
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

#[derive(Properties, PartialEq)]
pub struct TableProps {
    pub row_data: Vec<FieldView>,
    pub column_names: FieldSpec,
}

#[function_component]
pub fn Table(props: &TableProps) -> Html {
    html! {
        <table>
            <th>{ for props.column_names.iter().map(|header| html!{
                <td>{&header}</td>
            })}</th>{
                for props.row_data.iter().map(|child| html! {
                    <TableRow data={child.clone()} />
                })
            }
        </table>
    }
}

///////////////////////////////////////////////////////////////////////////////
