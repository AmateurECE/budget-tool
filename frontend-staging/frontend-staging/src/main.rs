///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the wasm application
//
// CREATED:         10/07/2022
//
// LAST EDITED:     10/11/2022
////

use yew::prelude::*;

///////////////////////////////////////////////////////////////////////////////
// FieldSpec
////

#[derive(Clone, Default, PartialEq)]
pub struct FieldSpec(Vec<String>);
impl FieldSpec {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a String> + 'a {
        self.0.iter()
    }
}

impl FromIterator<String> for FieldSpec {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self(iter.into_iter().collect::<Vec<String>>())
    }
}

///////////////////////////////////////////////////////////////////////////////
// FieldIterator
////

#[derive(Clone, Default, PartialEq)]
pub struct FieldIterator(Vec<String>);
impl FieldIterator {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a String> + 'a {
        self.0.iter()
    }
}

impl FromIterator<String> for FieldIterator {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self(iter.into_iter().collect::<Vec<String>>())
    }
}

///////////////////////////////////////////////////////////////////////////////
// Traits
////

pub trait FieldNames {
    fn field_names() -> FieldSpec;
}

pub trait Fields {
    fn fields(&self) -> FieldIterator;
}

///////////////////////////////////////////////////////////////////////////////
// TableRow
////

#[derive(Properties, PartialEq)]
pub struct TableRowProps {
    data: FieldIterator,
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
    pub fields: Vec<FieldIterator>,
    pub field_names: FieldSpec,
}

#[function_component]
fn Table(props: &TableProps) -> Html {
    html! {
        <table>
            <th>{ for props.field_names.iter().map(|header| html!{
                <td>{&header}</td>
            })}</th>{
                for props.fields.iter().map(|child| html! {
                    <TableRow data={child.clone()} />
                })
            }
        </table>
    }
}

///////////////////////////////////////////////////////////////////////////////
// Scratch - Example
////

#[derive(Clone, PartialEq, Properties)]
struct SomeObject {
    pub foo: String,
    pub bar: String,
}

// TODO: This should be a derive macro
impl FieldNames for SomeObject {
    fn field_names() -> FieldSpec {
        FieldSpec(vec!["foo".to_string(), "bar".to_string()])
    }
}

// TODO: This should be a derive macro
impl Fields for SomeObject {
    fn fields(&self) -> FieldIterator {
        FieldIterator(vec![self.foo.clone(), self.bar.clone()])
    }
}

///////////////////////////////////////////////////////////////////////////////
// Scratch - Calling Application
////

#[function_component]
fn App() -> Html {
    let objects = vec![
        SomeObject { foo: "a".to_string(), bar: "b".to_string() }.fields(),
        SomeObject { foo: "c".to_string(), bar: "d".to_string() }.fields(),
    ];
    html! {
        <Table field_names={SomeObject::field_names()} fields={objects} />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

///////////////////////////////////////////////////////////////////////////////
