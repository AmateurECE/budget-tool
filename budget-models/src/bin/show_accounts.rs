///////////////////////////////////////////////////////////////////////////////
// NAME:            show_accounts.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Quick script to show accounts.
//
// CREATED:         04/10/2022
//
// LAST EDITED:     04/10/2022
////

extern crate budget_models;
extern crate diesel;

use self::budget_models::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use budget_models::schema::accounts::dsl::*;

    let connection = establish_connection();
    let results = accounts.load::<Account>(&connection)
        .expect("Error loading accounts");

    println!("Displaying {} accounts", results.len());
    for account in results {
        println!("{}", account.name);
    }
}

///////////////////////////////////////////////////////////////////////////////
