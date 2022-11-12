///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the data-import convenience tool
//
// CREATED:         09/22/2022
//
// LAST EDITED:     11/11/2022
////

use clap::{Parser, Subcommand};
use sea_orm::Database;

mod line_item_instance;
mod periodic_budget;
mod table;
mod transaction;

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
        #[command(subcommand)]
        verb: periodic_budget::Verb,
    },

    /// Actions available on the set of line_item_instances
    LineItemInstance {
        #[command(subcommand)]
        verb: line_item_instance::Verb,
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
    let db = Database::connect(args.url).await?;
    match &args.object {
        Object::Periodic { verb } => periodic_budget::op(verb, &db).await,
        Object::LineItemInstance { verb } => {
            line_item_instance::op(verb, &db).await
        }
        Object::Transaction {
            transaction_type,
            verb,
        } => transaction::op(verb, *transaction_type, &db).await,
    }
}

///////////////////////////////////////////////////////////////////////////////
