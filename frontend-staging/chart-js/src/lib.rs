///////////////////////////////////////////////////////////////////////////////
// NAME:            lib.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Wasm bindings for ChartJS.
//
// CREATED:         10/19/2022
//
// LAST EDITED:     10/23/2022
////

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;

///////////////////////////////////////////////////////////////////////////////
// ChartData
////

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ChartDataset {
    pub label: String,

    #[serde(rename = "backgroundColor")]
    pub background_color: String,

    #[serde(rename = "borderColor")]
    pub border_color: String,
    pub data: Vec<Option<i32>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub datasets: Vec<ChartDataset>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ChartOptions {}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ChartConfig {
    #[serde(rename = "type")]
    pub chart_type: String,
    pub data: ChartData,
    pub options: ChartOptions,
}

///////////////////////////////////////////////////////////////////////////////
// ChartJS Bindings
////

#[wasm_bindgen]
extern "C" {
    pub type Chart;

    #[wasm_bindgen(constructor)]
    pub fn new(item: HtmlCanvasElement, config: JsValue) -> Chart;
}

///////////////////////////////////////////////////////////////////////////////
