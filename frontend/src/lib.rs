///////////////////////////////////////////////////////////////////////////////
// NAME:            lib.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint.
//
// CREATED:         04/16/2022
//
// LAST EDITED:     04/16/2022
////

mod app;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::BudgetService>();

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
