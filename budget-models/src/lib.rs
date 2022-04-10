///////////////////////////////////////////////////////////////////////////////
// NAME:            lib.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Library for accessing budget models
//
// CREATED:         04/10/2022
//
// LAST EDITED:     04/10/2022
////

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate clap;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use self::models::{Account, NewAccount};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_account<'a>(conn: &PgConnection, name: &'a str) -> Account {
    use schema::accounts;

    let new_account = NewAccount { name };
    diesel::insert_into(accounts::table)
        .values(&new_account)
        .get_result(conn)
        .expect("Error saving new account!")
}

///////////////////////////////////////////////////////////////////////////////
