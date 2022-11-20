///////////////////////////////////////////////////////////////////////////////
// NAME:            account_balance.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Component which displays balance over time for an account.
//
// CREATED:         10/14/2022
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

use crate::view::ViewHeader;
use chart_js::ChartDatasetBuilder;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlSelectElement};
use yew::prelude::*;
use yew_roots::chart::MultiSeriesLineChart;

///////////////////////////////////////////////////////////////////////////////
// BalanceHistoryChart
////

#[derive(Properties, PartialEq)]
pub struct BalanceHistoryChartProps {
    pub actual: Vec<Option<i32>>,
    pub predicted: Vec<Option<i32>>,
}

#[function_component]
pub fn BalanceHistoryChart(props: &BalanceHistoryChartProps) -> Html {
    let title = "Account Balance".to_string();
    let labels = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ]
    .into_iter()
    .map(|month| month.to_string())
    .collect::<Vec<String>>();
    let datasets = vec![
        ChartDatasetBuilder::default()
            .label("Account Balance".to_string())
            .data(props.actual.clone())
            .background_color("rgb(255, 99, 132)".to_string())
            .border_color("rgb(255, 99, 132)".to_string())
            .stepped(true)
            .build()
            .unwrap(),
        ChartDatasetBuilder::default()
            .label("Predicted Balance".to_string())
            .data(props.predicted.clone())
            .background_color("rgb(75, 192, 192)".to_string())
            .border_color("rgb(75, 192, 192)".to_string())
            .stepped(true)
            .build()
            .unwrap(),
    ];

    html! {
        <MultiSeriesLineChart x_labels={labels} {datasets} {title} />
    }
}

///////////////////////////////////////////////////////////////////////////////
// SwitchedBalanceHistoryChart
////

#[derive(Properties, PartialEq)]
pub struct SwitchedBalanceHistoryChartProps {
    pub actual: HashMap<String, Vec<Option<i32>>>,
    pub predicted: HashMap<String, Vec<Option<i32>>>,
}

#[function_component]
pub fn SwitchedBalanceHistoryChart(
    props: &SwitchedBalanceHistoryChartProps,
) -> Html {
    let account = use_context::<Account>().expect("No context found!");

    html! {
        <BalanceHistoryChart
         actual={props.actual.get(&account.name).unwrap().clone()}
         predicted={props.predicted.get(&account.name).unwrap().clone()} />
    }
}

///////////////////////////////////////////////////////////////////////////////
// AccountRouter
////

#[derive(Clone, Debug, PartialEq)]
struct Account {
    pub name: String,
}

#[derive(Properties, PartialEq)]
pub struct AccountRouterProps {
    pub options: Vec<String>,
    pub children: Children,
}

#[function_component]
pub fn AccountRouter(props: &AccountRouterProps) -> Html {
    let name = props.options.first().map(|op| op.clone()).unwrap();
    let state = use_state(|| Account { name });
    let onchange = {
        let state = state.clone();
        Callback::from(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let select = target
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
                .expect("Event must have occurred on select element");
            state.set(Account {
                name: select.value().clone(),
            });
        })
    };

    html! {
        <>
            <select name="account" {onchange}>{
                props.options.iter().map(|value| html! {
                    <option value={value.clone()}>{value}</option>
                }).collect::<Html>()
            }</select>
            <ContextProvider<Account> context={(*state).clone()}>{
                for props.children.iter()
            }</ContextProvider<Account>>
        </>
    }
}

///////////////////////////////////////////////////////////////////////////////
// BalanceHistory
////

const DISCOVER_ACTUAL: [Option<i32>; 8] = [
    Some(0),
    Some(10),
    Some(5),
    Some(2),
    Some(20),
    Some(30),
    Some(45),
    None,
];

const DISCOVER_PREDICTED: [Option<i32>; 12] = [
    None,
    None,
    None,
    None,
    None,
    None,
    Some(45),
    Some(60),
    Some(53),
    Some(50),
    Some(60),
    Some(57),
];

#[function_component]
pub fn BalanceHistory() -> Html {
    let accounts = vec!["FECCCU", "Nicolet", "Discover"]
        .into_iter()
        .map(|account| account.to_string())
        .collect::<Vec<String>>();
    let actual = HashMap::<String, Vec<Option<i32>>>::from([
        ("Discover".to_string(), DISCOVER_ACTUAL.to_vec()),
        ("FECCCU".to_string(), vec![]),
        ("Nicolet".to_string(), vec![]),
    ]);
    let predicted = HashMap::from([
        ("Discover".to_string(), DISCOVER_PREDICTED.to_vec()),
        ("FECCCU".to_string(), vec![]),
        ("Nicolet".to_string(), vec![]),
    ]);

    html! {
        <>
        <ViewHeader text={"Account Balance History".to_string()} />
        <AccountRouter options={accounts}>
            <SwitchedBalanceHistoryChart {actual} {predicted} />
        </AccountRouter>
        </>
    }
}

///////////////////////////////////////////////////////////////////////////////
