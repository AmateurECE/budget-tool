///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the budget-cli convenience tool
//
// CREATED:         09/22/2022
//
// LAST EDITED:     11/15/2022
//
// Copyright 2022, Ethan D. Twardy
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
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
