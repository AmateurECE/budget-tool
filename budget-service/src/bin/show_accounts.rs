///////////////////////////////////////////////////////////////////////////////
// NAME:            show_accounts.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Quick script to show accounts.
//
// CREATED:         04/10/2022
//
// LAST EDITED:     04/12/2022
////

use std::env;
use budget_models::{Database, account};
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let db = Database::connect(database_url);
    let results = account::list(&db);
    println!("Displaying {} accounts", results.len());
    for account in results {
        println!("{}, {:?}, {}", account.name, account.account_type,
                 account.apr);
    }
}

///////////////////////////////////////////////////////////////////////////////
