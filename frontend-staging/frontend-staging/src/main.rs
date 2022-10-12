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
use yew_velcro::{
    Fields, FieldNames, fields::{FieldSpec, FieldView}, table::Table
};

///////////////////////////////////////////////////////////////////////////////
// Scratch
////

#[derive(Clone, PartialEq, Properties, Fields, FieldNames)]
struct SomeObject {
    pub foo: String,
    pub bar: String,
}

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
        <Table column_names={field_spec} row_data={field_views} />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

///////////////////////////////////////////////////////////////////////////////
