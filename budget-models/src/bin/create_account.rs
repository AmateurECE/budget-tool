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

extern crate budget_models;
extern crate diesel;

use clap::Parser;

use self::budget_models::*;

#[derive(Parser, Debug)]
struct Args {
    /// Name of the account
    name: String,
}

fn main() {
    let args = Args::parse();
    let connection = establish_connection();
    let account = create_account(&connection, &args.name);
    println!("Created account with name={}, id={}", account.name, account.id);
}

///////////////////////////////////////////////////////////////////////////////
