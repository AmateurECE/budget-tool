///////////////////////////////////////////////////////////////////////////////
// NAME:            account_balance.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Component which displays balance over time for an account.
//
// CREATED:         10/14/2022
//
// LAST EDITED:     10/23/2022
////

use yew::prelude::*;
use yew_roots::chart::{ChartDataset, MultiSeriesLineChart};
use crate::view::ViewHeader;

///////////////////////////////////////////////////////////////////////////////
// BalanceHistory
////

pub struct BalanceHistory {
    labels: Vec<String>,
    datasets: Vec<ChartDataset>,
    title: String,
}

impl Component for BalanceHistory {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let labels = vec![
            "January", "February", "March", "April", "May", "June", "July",
            "August", "September", "October", "November", "December",
        ].into_iter().map(|month| month.to_string()).collect::<Vec<String>>();
        let datasets = vec![
            ChartDataset {
                label: "Account Balance".to_string(),
                background_color: "rgb(255, 99, 132)".to_string(),
                border_color: "rgb(255, 99, 132)".to_string(),
                data: vec![
                    Some(0), Some(10), Some(5), Some(2), Some(20), Some(30),
                    Some(45), None],
            },
            ChartDataset {
                label: "Predicted Balance".to_string(),
                background_color: "rgb(0, 99, 132)".to_string(),
                border_color: "rgb(0, 99, 132)".to_string(),
                data: vec![
                    None, None, None, None, None, None, Some(45), Some(60),
                    Some(53), Some(50), Some(60), Some(57)
                ],
            },
        ];

        Self {
            labels, datasets,
            title: "My First Dataset".to_string(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <ViewHeader text={"Account Balance History".to_string()} />
            <MultiSeriesLineChart x_labels={self.labels.clone()}
             datasets={self.datasets.clone()} title={self.title.clone()} />
            </>
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
