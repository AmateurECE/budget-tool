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
use clap::{Parser, Subcommand};
use sea_orm::prelude::*;
use sea_orm::{Database, DatabaseConnection};
use std::cmp;

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
// Cheap Table Printing Solution
////

const PADDING_LENGTH: usize = 4;

fn print_table(data: Vec<Vec<String>>, headers: Vec<String>) {
    let row_length = data
        .iter()
        .map(|column| {
            let max = cmp::max(
                column
                    .iter()
                    .map(|element| element.len())
                    .max()
                    .unwrap_or(0),
                headers.first().map(|h| h.len()).unwrap_or(0),
            );
            if 0 == max % PADDING_LENGTH {
                max
            } else {
                max + (PADDING_LENGTH - (max % PADDING_LENGTH))
            }
        })
        .collect::<Vec<usize>>();

    let sum: usize = row_length.iter().sum();
    let mut output = headers
        .iter()
        .enumerate()
        .map(|(i, header)| {
            header.to_owned()
                + &(0..(row_length[i] - header.len()))
                    .map(|_| " ")
                    .collect::<String>()
        })
        .collect::<String>()
        + "\n";
    output += &(0..sum).map(|_| "-").collect::<String>();
    output += "\n";

    for i in 0..data[0].len() {
        for j in 0..data.len() {
            output += &data[j][i];
            output += &(0..(row_length[j] - data[j][i].len()))
                .map(|_| " ")
                .collect::<String>();
        }
    }
    println!("{}", output);
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
            print_table(
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
