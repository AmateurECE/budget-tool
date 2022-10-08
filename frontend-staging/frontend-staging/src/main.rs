///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the wasm application
//
// CREATED:         10/07/2022
//
// LAST EDITED:     10/08/2022
////

use yew::prelude::*;

pub trait Fields {
    fn fields() -> FieldSpec;
}

#[derive(PartialEq)]
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

#[derive(PartialEq, Properties)]
struct SomeObjectProps {
    pub foo: String,
    pub bar: String,
}

#[derive(PartialEq)]
struct SomeObject;

impl Fields for SomeObject {
    fn fields() -> FieldSpec {
        FieldSpec(vec!["foo".to_string(), "bar".to_string()])
    }
}

impl Component for SomeObject {
    type Properties = SomeObjectProps;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn view(&self, context: &Context<Self>) -> Html {
        html! {
            <><td>{&context.props().foo}</td><td>{&context.props().bar}</td></>
        }
    }
}

// TODO: Implement iterator for this object, which allows us to iterate over
// its fields. Then, the code to write all of this to a table is easy!
struct SomeObjectIterator;

#[derive(Properties, PartialEq)]
pub struct TableProps<T>
where T: Fields + PartialEq + Component,
{
    #[prop_or_default]
    pub children: ChildrenWithProps<T>,
    pub fields: FieldSpec,
}

#[function_component]
fn Table<T>(props: &TableProps<T>) -> Html
where T: Fields + PartialEq + Component
{
    html! {
        <table>
            <th>{ for props.fields.iter().map(|header| html!{
                <td>{&header}</td>
            })}</th>{
                for props.children.iter().map(|child| html! {
                    <tr>{ child }</tr>
                })
            }
        </table>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <Table<SomeObject> fields={SomeObject::fields()}>
            <SomeObject foo="a" bar="b" />
            <SomeObject foo="c" bar="d" />
        </Table<SomeObject>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

///////////////////////////////////////////////////////////////////////////////
