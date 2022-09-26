///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the data-import convenience tool
//
// CREATED:         09/22/2022
//
// LAST EDITED:     09/25/2022
////

use budget_models::models::NewTransaction;
use clap::Parser;
use http::Uri;
use reqwest::Client;
use serde::Deserialize;
use std::fs::File;

///////////////////////////////////////////////////////////////////////////////
// Partial New Transactions--Schema for pre-imported data
////

#[derive(Deserialize)]
struct PartialNewTransaction {
    account: String,
    description: String,
    date: String,
    amount: f64,
}

impl Into<NewTransaction> for PartialNewTransaction {
    fn into(self) -> NewTransaction {
        todo!()
    }
}

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
    let client = Client::builder().user_agent("budget-import").build()?;

    let transactions: Vec<PartialNewTransaction> =
        serde_json::from_reader(File::open(&args.file)?)?;
    let endpoint = Uri::builder()
        .scheme("https")
        .authority(args.host.as_str())
        .path_and_query("/api/transactions")
        .build()?
        .to_string();
    for transaction in transactions {
        let new_transaction: NewTransaction = transaction.into();
        client
            .post(&endpoint)
            .body(serde_json::to_string(&new_transaction)?)
            .send()
            .await?;
    }

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
