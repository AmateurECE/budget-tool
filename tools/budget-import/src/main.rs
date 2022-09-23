///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the data-import convenience tool
//
// CREATED:         09/22/2022
//
// LAST EDITED:     09/22/2022
////

use clap::Parser;

#[derive(Parser)]
#[clap(author, version)]
struct Args {
    /// JSON file of data to import
    #[clap(value_parser)]
    pub file: String,
}

fn main() {
    let args = Args::parse();
}

///////////////////////////////////////////////////////////////////////////////
