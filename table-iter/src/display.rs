///////////////////////////////////////////////////////////////////////////////
// NAME:            display.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Some utilities for displaying types that don't implement
//                  ToString.
//
// CREATED:         11/12/2022
//
// LAST EDITED:     11/12/2022
////

pub fn option<T: ToString>(value: &Option<T>) -> String {
    value
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or("".to_string())
}

///////////////////////////////////////////////////////////////////////////////
