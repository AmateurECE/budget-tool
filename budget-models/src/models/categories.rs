///////////////////////////////////////////////////////////////////////////////
// NAME:            categories.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Categories for budget items.
//
// CREATED:         07/05/2022
//
// LAST EDITED:     07/05/2022
////

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub name: String,
}

///////////////////////////////////////////////////////////////////////////////
