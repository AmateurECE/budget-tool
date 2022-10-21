///////////////////////////////////////////////////////////////////////////////
// NAME:            account_balance.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Component which displays balance over time for an account.
//
// CREATED:         10/14/2022
//
// LAST EDITED:     10/20/2022
////

use yew::prelude::*;
use yew_roots::chart::SingleSeriesLineChart;
use crate::view::ViewHeader;

///////////////////////////////////////////////////////////////////////////////
// BalanceHistory
////

pub struct BalanceHistory {
    labels: Vec<String>,
    y_data: Vec<i32>,
    title: String,
}

impl Component for BalanceHistory {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let labels = vec![
            "January", "February", "March", "April", "May", "June",
        ].into_iter().map(|month| month.to_string()).collect::<Vec<String>>();
        let y_data = vec![0, 10, 5, 2, 20, 30, 45];

        Self {
            labels, y_data,
            title: "My First Dataset".to_string(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <ViewHeader text={"Account Balance History".to_string()} />
            <SingleSeriesLineChart x_labels={self.labels.clone()}
             y_data={self.y_data.clone()} title={self.title.clone()} />
            </>
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
