///////////////////////////////////////////////////////////////////////////////
// NAME:            spending.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Component that tracks spending history by line item.
//
// CREATED:         10/14/2022
//
// LAST EDITED:     11/12/2022
////

use yew::prelude::*;
use table_iter::prelude::*;
use yew_roots::table::Table;

use crate::view::ViewHeader;

///////////////////////////////////////////////////////////////////////////////
// SpendingHistory
////

#[derive(Clone, Default, PartialEq, Fields, FieldNames)]
struct LineItem {
    #[fields(rename = "Name")]
    pub name: String,
    #[fields(rename = "Last Month (Budgeted)")]
    pub last_month_budgeted: i64,
    #[fields(rename = "Last Month (Spent)")]
    pub last_month_spent: i64,
    #[fields(rename = "This Month Last Year (Spent)")]
    pub this_month_last_year_spent: i64,
    #[fields(rename = "Last Six Months (Average Spent)")]
    pub last_six_months_spent: i64,
}

#[function_component]
pub fn SpendingHistory() -> Html {
    let objects = vec![
        LineItem::default(),
        LineItem::default(),
        LineItem::default(),
        LineItem::default(),
        LineItem::default(),
        LineItem::default(),
        LineItem::default(),
    ];

    html! {
        <>
        <ViewHeader text={"Spending History By Line Item".to_string()} />
        <Table<LineItem> class={classes!(
            "table", "table-striped", "table-hover", "table-responsive",
            "table-sm")}
         row_data={objects} />
        </>
    }
}

///////////////////////////////////////////////////////////////////////////////
