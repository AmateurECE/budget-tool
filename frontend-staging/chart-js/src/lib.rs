///////////////////////////////////////////////////////////////////////////////
// NAME:            lib.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Wasm bindings for ChartJS.
//
// CREATED:         10/19/2022
//
// LAST EDITED:     10/24/2022
////

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

///////////////////////////////////////////////////////////////////////////////
// ChartJS Bindings
////

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ChartDataset")]
    pub type IChartDataset;
    #[wasm_bindgen(typescript_type = "ChartOptions")]
    pub type IChartOptions;
    #[wasm_bindgen(typescript_type = "ChartConfiguration")]
    pub type IChartConfiguration;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ChartData")]
    pub type IChartData;

    #[wasm_bindgen(method, getter)]
    pub fn datasets(data: &IChartData) -> Box<[JsValue]>;
    #[wasm_bindgen(method, setter)]
    pub fn set_datasets(data: &IChartData, datasets: Box<[JsValue]>);
}

#[wasm_bindgen]
extern "C" {
    pub type Chart;

    #[wasm_bindgen(constructor)]
    pub fn new(item: HtmlCanvasElement, config: IChartConfiguration) -> Chart;
    #[wasm_bindgen(method)]
    pub fn update(chart: &Chart);
    #[wasm_bindgen(method, getter)]
    pub fn data(chart: &Chart) -> IChartData;
}

///////////////////////////////////////////////////////////////////////////////
// ChartDataset
////

#[derive(Builder, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChartDataset {
    pub label: String,
    pub data: Vec<Option<i32>>,

    #[serde(rename = "backgroundColor")]
    #[builder(default, setter(strip_option))]
    pub background_color: Option<String>,
    #[serde(rename = "borderColor")]
    #[builder(default, setter(strip_option))]
    pub border_color: Option<String>,

    #[builder(default, setter(strip_option))]
    pub fill: Option<bool>,
    #[builder(default, setter(strip_option))]
    pub stepped: Option<bool>,
}

impl Into<IChartDataset> for ChartDataset {
    fn into(self) -> IChartDataset {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
// ChartData
////

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub datasets: Vec<ChartDataset>,
}

impl Into<IChartData> for ChartData {
    fn into(self) -> IChartData {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
// ChartOptions
////

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ChartOptions {}

impl Into<IChartOptions> for ChartOptions {
    fn into(self) -> IChartOptions {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
// ChartConfiguration
////

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ChartConfiguration {
    #[serde(rename = "type")]
    pub chart_type: String,
    pub data: ChartData,
    pub options: ChartOptions,
}

impl Into<IChartConfiguration> for ChartConfiguration {
    fn into(self) -> IChartConfiguration {
        serde_wasm_bindgen::to_value(&self)
            .expect(
                "Failed to convert ChartConfiguration to IChartConfiguration",
            )
            .into()
    }
}

///////////////////////////////////////////////////////////////////////////////
