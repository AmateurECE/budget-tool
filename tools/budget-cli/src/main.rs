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

use budget_backend_lib::prelude::*;
use clap::{Parser, Subcommand};
use sea_orm::prelude::*;
use sea_orm::{Database, DatabaseConnection};

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
enum Verb {
    List,
}

///////////////////////////////////////////////////////////////////////////////
// PeriodicBudget Operations
////

async fn periodic_budget(
    verb: &Verb,
    db: &DatabaseConnection,
) -> anyhow::Result<()> {
    match &verb {
        Verb::List => {
            let budgets = PeriodicBudgets::find()
                .all(db)
                .await?
                .iter()
                .map(|budget| {
                    vec![
                        budget.id.to_string(),
                        budget.start_date.to_string(),
                        budget.end_date.to_string(),
                    ]
                })
                .collect::<Vec<Vec<String>>>();

            table::print(
                budgets
                    .iter()
                    .map(|row| row.as_slice())
                    .collect::<Vec<&[String]>>()
                    .as_slice(),
                vec![
                    "Id".to_string(),
                    "Start Date".to_string(),
                    "End Date".to_string(),
                ]
                .as_slice(),
            );
        }
    }

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
// Main
////

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let db = Database::connect(args.url).await?;
    match &args.object {
        Object::Periodic { verb } => periodic_budget(verb, &db).await,
    }
}

///////////////////////////////////////////////////////////////////////////////
