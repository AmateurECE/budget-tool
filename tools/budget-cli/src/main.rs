///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the data-import convenience tool
//
// CREATED:         09/22/2022
//
// LAST EDITED:     11/10/2022
////

use clap::{Parser, Subcommand};
use sea_orm::Database;

mod periodic_budget;
mod table;

///////////////////////////////////////////////////////////////////////////////
// CLI
////

#[derive(Parser)]
#[clap(author, version)]
struct Args {
    /// Database URL
    #[clap(short, long, value_parser)]
    pub url: String,

    #[command(subcommand)]
    pub object: Object,
}

#[derive(Subcommand)]
enum Object {
    /// Actions available on the set of periodic budgets
    Periodic {
        /// List periodic budgets in the database
        #[command(subcommand)]
        verb: Verb,
    },
}

#[derive(Subcommand)]
pub(crate) enum Verb {
    List,
}

///////////////////////////////////////////////////////////////////////////////
// Main
////

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let db = Database::connect(args.url).await?;
    match &args.object {
        Object::Periodic { verb } => periodic_budget::op(verb, &db).await,
    }
}

///////////////////////////////////////////////////////////////////////////////
