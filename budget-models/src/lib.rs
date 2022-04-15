///////////////////////////////////////////////////////////////////////////////
// NAME:            lib.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Library for accessing budget models
//
// CREATED:         04/10/2022
//
// LAST EDITED:     04/14/2022
////

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub mod models;
pub mod entities;

pub struct Database(PgConnection);
impl Database {
    pub fn connect(url: String) -> Self {
        Database(
            PgConnection::establish(&url)
                .expect(&format!("Error connecting to {}", url))
        )
    }

    pub fn get(&self) -> &PgConnection { &self.0 }
}

pub mod account {
}

///////////////////////////////////////////////////////////////////////////////
