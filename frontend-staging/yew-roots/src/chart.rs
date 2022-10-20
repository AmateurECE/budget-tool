///////////////////////////////////////////////////////////////////////////////
// NAME:            chart.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Components for rendering simple charts, powered by ChartJS.
//
// CREATED:         10/20/2022
//
// LAST EDITED:     10/20/2022
////

use chart_js::{Chart, ChartConfig, ChartData, ChartDataset, ChartOptions};
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

///////////////////////////////////////////////////////////////////////////////
// SingleSeriesLineChart
////

#[derive(Properties, PartialEq)]
pub struct SingleSeriesLineChartProps {
    pub x_labels: Vec<String>,
    // TODO: Use Num from num-traits here
    pub y_data: Vec<i32>,
    pub title: String,
}

pub struct SingleSeriesLineChart {
    chart: Option<Chart>,
    canvas: NodeRef,
}

impl Component for SingleSeriesLineChart {
    type Message = ();
    type Properties = SingleSeriesLineChartProps;

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
        let data = ChartData {
            labels: context.props().x_labels.clone(),
            datasets: vec![ ChartDataset {
                label: context.props().title.clone(),
                background_color: "rgb(255, 99, 132)".to_string(),
                border_color: "rgb(255, 99, 132)".to_string(),
                data: context.props().y_data.clone(),
            }],
        };

        let chart_config = ChartConfig {
            chart_type: "line".to_string(),
            data,
            options: ChartOptions {},
        };

        let canvas = self.canvas.cast::<HtmlCanvasElement>().unwrap();
        let config = serde_wasm_bindgen::to_value(&chart_config).unwrap();
        self.chart = Some(Chart::new(canvas, config));
    }
}

///////////////////////////////////////////////////////////////////////////////
