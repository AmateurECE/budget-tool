///////////////////////////////////////////////////////////////////////////////
// NAME:            periodic_budget.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     A budget that concerns a period of time.
//
// CREATED:         07/05/2022
//
// LAST EDITED:     07/05/2022
////

use chrono::{DateTime, offset::Utc};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PeriodicBudget {
    pub id: i32,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub start_date: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub end_date: DateTime<Utc>,
}

///////////////////////////////////////////////////////////////////////////////
