///////////////////////////////////////////////////////////////////////////////
// NAME:            one_time_budget.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     A budget for a series of one-time transactions.
//
// CREATED:         07/05/2022
//
// LAST EDITED:     07/05/2022
////

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OneTimeBudget {
    pub id: i32,
    pub description: String,
}

///////////////////////////////////////////////////////////////////////////////
