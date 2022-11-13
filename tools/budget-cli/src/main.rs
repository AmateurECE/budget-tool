///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the data-import convenience tool
//
// CREATED:         09/22/2022
//
// LAST EDITED:     11/13/2022
////

use clap::{Parser, Subcommand};
use sea_orm::Database;
use std::env;

mod display;
mod line_item;
mod periodic_budget;
mod table;
mod transaction;

///////////////////////////////////////////////////////////////////////////////
// CLI
////

#[derive(Parser)]
#[clap(author, version)]
struct Args {
    #[command(subcommand)]
    pub object: Object,
}

#[derive(Subcommand)]
enum Object {
    /// Actions available on the set of periodic budgets
    PeriodicBudget {
        #[command(subcommand)]
        verb: periodic_budget::Verb,
    },

    /// Actions available on the set of line items
    LineItem {
        #[command(subcommand)]
        verb: line_item::Verb,
    },

    /// Actions available on the set of transactions, real and planned
    Transaction {
        /// The type of transaction to act on
        #[arg(value_enum)]
        #[clap(short, long, value_parser)]
        transaction_type: transaction::TransactionType,

        #[command(subcommand)]
        verb: transaction::Verb,
    },
}

///////////////////////////////////////////////////////////////////////////////
// Main
////

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let url = env::var("DATABASE_URL")?;
    let db = Database::connect(&url).await?;
    match &args.object {
        Object::PeriodicBudget { verb } => {
            periodic_budget::op(verb, &db).await
        }
        Object::LineItem { verb } => line_item::op(verb, &db).await,
        Object::Transaction {
            transaction_type,
            verb,
        } => transaction::op(verb, *transaction_type, &db).await,
    }
}

///////////////////////////////////////////////////////////////////////////////
