///////////////////////////////////////////////////////////////////////////////
// NAME:            create_account.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Quick script to create an account
//
// CREATED:         04/10/2022
//
// LAST EDITED:     04/10/2022
////

use std::env;
use budget_models::{Database, account};
use clap::Parser;
use dotenv::dotenv;

#[derive(Parser, Debug)]
struct Args {
    /// Name of the account
    name: String,
}

fn main() {
    let args = Args::parse();
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let db = Database::connect(database_url);
    let account = account::create(&db, &args.name);
    println!("Created account with name={}, id={}", account.name, account.id);
}

///////////////////////////////////////////////////////////////////////////////
