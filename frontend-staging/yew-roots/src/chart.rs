///////////////////////////////////////////////////////////////////////////////
// NAME:            chart.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Components for rendering simple charts, powered by ChartJS.
//
// CREATED:         10/20/2022
//
// LAST EDITED:     10/23/2022
////

use chart_js::{Chart, ChartConfiguration, ChartData, ChartOptions};
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

pub use chart_js::ChartDataset;

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
            chart.data()
                .set_datasets(context.props().datasets.iter().map(|dataset| {
                    serde_wasm_bindgen::to_value(&dataset).unwrap()
                }).collect::<Vec<JsValue>>().into_boxed_slice());
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
