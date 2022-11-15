///////////////////////////////////////////////////////////////////////////////
// NAME:            display.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Utilities for displaying some types.
//
// CREATED:         11/12/2022
//
// LAST EDITED:     11/13/2022
////

use chrono::{DateTime, FixedOffset};

pub fn date(value: &DateTime<FixedOffset>) -> String {
    value.format("%d %b %Y").to_string()
}

///////////////////////////////////////////////////////////////////////////////
