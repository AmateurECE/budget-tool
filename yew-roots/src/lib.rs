///////////////////////////////////////////////////////////////////////////////
// NAME:            lib.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the yew-roots library
//
// CREATED:         10/12/2022
//
// LAST EDITED:     11/12/2022
////

#[cfg(feature = "table")]
pub mod table;

#[cfg(feature = "chart")]
pub mod chart;

///////////////////////////////////////////////////////////////////////////////
