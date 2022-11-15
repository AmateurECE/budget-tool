///////////////////////////////////////////////////////////////////////////////
// NAME:            chart.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Components for rendering simple charts, powered by ChartJS.
//
// CREATED:         10/20/2022
//
// LAST EDITED:     11/15/2022
//
// Copyright 2022, Ethan D. Twardy
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
////

use chart_js::{
    Chart, ChartConfiguration, ChartData, ChartDataset, ChartOptions,
};
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

///////////////////////////////////////////////////////////////////////////////
// MultiSeriesLineChart
////

#[derive(Properties, PartialEq)]
pub struct MultiSeriesLineChartProps {
    pub x_labels: Vec<String>,
    // TODO: Use Num from num-traits here
    pub datasets: Vec<ChartDataset>,
    pub title: String,
}

pub struct MultiSeriesLineChart {
    chart: Option<Chart>,
    canvas: NodeRef,
}

impl Component for MultiSeriesLineChart {
    type Message = ();
    type Properties = MultiSeriesLineChartProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            chart: None,
            canvas: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <canvas ref={self.canvas.clone()}></canvas>
            </div>
        }
    }

    fn rendered(&mut self, context: &Context<Self>, _first_render: bool) {
        if let Some(chart) = &self.chart {
            chart.data().set_datasets(
                context
                    .props()
                    .datasets
                    .iter()
                    .map(|dataset| {
                        serde_wasm_bindgen::to_value(&dataset).unwrap()
                    })
                    .collect::<Vec<JsValue>>()
                    .into_boxed_slice(),
            );
            chart.update();
        } else {
            let data = ChartData {
                labels: context.props().x_labels.clone(),
                datasets: context.props().datasets.clone(),
            };

            let chart_config = ChartConfiguration {
                chart_type: "line".to_string(),
                data,
                options: ChartOptions {},
            };

            let canvas = self.canvas.cast::<HtmlCanvasElement>().unwrap();
            let config = serde_wasm_bindgen::to_value(&chart_config).unwrap();
            self.chart = Some(Chart::new(canvas, config.into()));
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
