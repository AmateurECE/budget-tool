///////////////////////////////////////////////////////////////////////////////
// NAME:            tag.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     A tag for a transaction (provides an additional method of
//                  searching/indexing).
//
// CREATED:         09/17/2022
//
// LAST EDITED:     09/17/2022
////

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub id: i32,
    pub tag: String,
}

///////////////////////////////////////////////////////////////////////////////
