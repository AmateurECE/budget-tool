///////////////////////////////////////////////////////////////////////////////
// NAME:            lib.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Library for accessing budget models
//
// CREATED:         04/10/2022
//
// LAST EDITED:     07/14/2022
////

pub mod models;

#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(feature = "business-logic")] {
        pub mod budget_tracker;
        pub mod calculation;
        pub mod money;
        pub mod total;
        pub mod transaction_breakdown;
    }
}

///////////////////////////////////////////////////////////////////////////////
