///////////////////////////////////////////////////////////////////////////////
// NAME:            lib.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the yew-velcro library
//
// CREATED:         10/12/2022
//
// LAST EDITED:     10/12/2022
////

pub mod fields;
pub mod table;

pub use fields::Fields;
pub use yew_velcro_macros::Fields;

///////////////////////////////////////////////////////////////////////////////
