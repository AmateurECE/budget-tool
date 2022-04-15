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
    use chrono::offset::Local;
    use diesel::prelude::*;

    use crate::Database;
    use crate::models::{accounts, Account, NewAccount, AccountType};

    pub fn create<'a>(
        db: &Database, name: &'a str, account_type: AccountType
    ) -> Account
    {
        let new_account = NewAccount {
            name, account_type, apr: 0.0,
            accruing_start_date: Local::now().naive_local()
        };
        diesel::insert_into(accounts::table)
            .values(&new_account)
            .get_result(db.get())
            .expect("Error saving new account!")
    }

    pub fn list(db: &Database) -> Vec<Account> {
        accounts::dsl::accounts.load::<Account>(db.get())
            .expect("Error loading accounts from database!")
    }
}

///////////////////////////////////////////////////////////////////////////////
