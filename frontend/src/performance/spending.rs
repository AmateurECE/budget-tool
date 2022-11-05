///////////////////////////////////////////////////////////////////////////////
// NAME:            spending.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Component that tracks spending history by line item.
//
// CREATED:         10/14/2022
//
// LAST EDITED:     10/20/2022
////

use yew::prelude::*;
use yew_roots::prelude::*;
use yew_roots::table::Table;

use crate::view::ViewHeader;

///////////////////////////////////////////////////////////////////////////////
// SpendingHistory
////

#[derive(Clone, Default, PartialEq, Fields, FieldNames)]
struct LineItem {
    #[field_name(rename = "Name")]
    pub name: String,
    #[field_name(rename = "Last Month (Budgeted)")]
    pub last_month_budgeted: i64,
    #[field_name(rename = "Last Month (Spent)")]
    pub last_month_spent: i64,
    #[field_name(rename = "This Month Last Year (Spent)")]
    pub this_month_last_year_spent: i64,
    #[field_name(rename = "Last Six Months (Average Spent)")]
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
