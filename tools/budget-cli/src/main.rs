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

use budget_backend_lib::prelude::*;
use clap::Parser;
use sea_orm::prelude::*;
use sea_orm::Database;

///////////////////////////////////////////////////////////////////////////////
// Main
////

#[derive(Parser)]
#[clap(author, version)]
struct Args {
    /// Database URL
    #[clap(short, long, value_parser)]
    pub url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let db = Database::connect(args.url).await?;
    let budgets: Vec<periodic_budgets::Model> =
        PeriodicBudgets::find().all(&db).await?;
    println!("{:?}", &budgets);
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
