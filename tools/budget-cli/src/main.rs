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
            let budgets: Vec<periodic_budgets::Model> =
                PeriodicBudgets::find().all(db).await?;
            let output = vec![
                budgets
                    .iter()
                    .map(|budget| budget.id.to_string())
                    .collect::<Vec<String>>(),
                budgets
                    .iter()
                    .map(|budget| budget.start_date.to_string())
                    .collect::<Vec<String>>(),
                budgets
                    .iter()
                    .map(|budget| budget.end_date.to_string())
                    .collect::<Vec<String>>(),
            ];
            table::print_table(
                output,
                vec![
                    "Id".to_string(),
                    "Start Date".to_string(),
                    "End Date".to_string(),
                ],
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
