///////////////////////////////////////////////////////////////////////////////
// NAME:            account_balance.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Component which displays balance over time for an account.
//
// CREATED:         10/14/2022
//
// LAST EDITED:     10/18/2022
////

use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use web_sys::HtmlCanvasElement;

///////////////////////////////////////////////////////////////////////////////
// ChartData
////

#[derive(Serialize, Deserialize)]
pub struct ChartDataset {
    label: String,

    #[serde(rename = "backgroundColor")]
    background_color: String,

    #[serde(rename = "borderColor")]
    border_color: String,

    data: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ChartData {
    labels: Vec<String>,
    datasets: Vec<ChartDataset>,
}

#[derive(Serialize, Deserialize)]
pub struct ChartOptions {}

#[derive(Serialize, Deserialize)]
pub struct ChartConfig {
    #[serde(rename = "type")]
    chart_type: String,
    data: ChartData,
    options: ChartOptions,
}

///////////////////////////////////////////////////////////////////////////////
// ChartJS Bindings
////

#[wasm_bindgen]
extern "C" {
    type Chart;

    #[wasm_bindgen(constructor)]
    fn new(item: HtmlCanvasElement, config: JsValue) -> Chart;
}

///////////////////////////////////////////////////////////////////////////////
// BalanceHistory
////

pub struct BalanceHistory {
    chart: Option<Chart>,
    canvas: NodeRef,
}

impl Component for BalanceHistory {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            chart: None,
            canvas: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <h2>{ "Account Balance History" }</h2>
            <div>
                <canvas ref={self.canvas.clone()}></canvas>
            </div>
            </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let labels = vec![
                "January".to_string(),
                "February".to_string(),
                "March".to_string(),
                "April".to_string(),
                "May".to_string(),
                "June".to_string(),
            ];

            let y_data = vec![0, 10, 5, 2, 20, 30, 45];

            let data = ChartData {
                labels,
                datasets: vec![ ChartDataset {
                    label: "My First dataset".to_string(),
                    background_color: "rgb(255, 99, 132)".to_string(),
                    border_color: "rgb(255, 99, 132)".to_string(),
                    data: y_data,
                }],
            };

            let chart_config = ChartConfig {
                chart_type: "line".to_string(),
                data,
                options: ChartOptions {},
            };

            let config = serde_wasm_bindgen::to_value(&chart_config).unwrap();
            web_sys::console::log_1(&config);
            self.chart = Some(Chart::new(
                self.canvas.cast::<HtmlCanvasElement>().unwrap(),
                config
            ));
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
