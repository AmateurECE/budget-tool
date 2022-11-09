///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the data-import convenience tool
//
// CREATED:         09/22/2022
//
// LAST EDITED:     11/09/2022
////

use clap::Parser;

///////////////////////////////////////////////////////////////////////////////
// Main
////

#[derive(Parser)]
#[clap(author, version)]
struct Args {
    /// JSON file of data to import
    #[clap(value_parser)]
    pub file: String,

    /// URL of the API server
    #[clap(value_parser)]
    pub host: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
