///////////////////////////////////////////////////////////////////////////////
// NAME:            lib.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Library for accessing budget models
//
// CREATED:         04/10/2022
//
// LAST EDITED:     05/01/2022
////

#[cfg(not(target_family = "wasm"))]
#[macro_use]
extern crate diesel;

pub mod models;
pub mod entities;

///////////////////////////////////////////////////////////////////////////////
