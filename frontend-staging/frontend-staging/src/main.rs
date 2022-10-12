///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the wasm application
//
// CREATED:         10/07/2022
//
// LAST EDITED:     10/12/2022
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
// FieldView
////

#[derive(Clone, Default, PartialEq)]
pub struct FieldView(Vec<String>);
impl FieldView {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a String> + 'a {
        self.0.iter()
    }
}

impl FromIterator<String> for FieldView {
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
    fn fields(&self) -> FieldView;
}

///////////////////////////////////////////////////////////////////////////////
// TableRow
////

#[derive(Properties, PartialEq)]
pub struct TableRowProps {
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
    pub fields: Vec<FieldView>,
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
// TODO: It would be super cool if this could also use Cow<'_>
impl Fields for SomeObject {
    fn fields(&self) -> FieldView {
        FieldView(vec![self.foo.clone(), self.bar.clone()])
    }
}

///////////////////////////////////////////////////////////////////////////////
// Scratch - Calling Application
////

#[function_component]
fn App() -> Html {
    let objects = vec![
        SomeObject { foo: "a".to_string(), bar: "b".to_string() },
        SomeObject { foo: "c".to_string(), bar: "d".to_string() },
    ];

    let field_spec = SomeObject::field_names();
    let field_views = objects.iter().map(|object| object.fields())
        .collect::<Vec<FieldView>>();
    html! {
        <Table field_names={field_spec} fields={field_views} />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

///////////////////////////////////////////////////////////////////////////////
