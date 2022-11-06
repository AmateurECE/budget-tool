///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Budget migration manager
//
// CREATED:         09/18/2022
//
// LAST EDITED:     11/06/2022
////

use budget_backend_lib::SecretManager;
use clap::Parser;
use std::env;
use sqlx::PgPool;

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    /// Path to a JSON file containing database credentials
    #[clap(short, long, value_parser)]
    secret_file: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let url_template =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let secret_manager = SecretManager::new(args.secret_file);
    let url = secret_manager.with_url(url_template.parse()?)?;

    // Run database migrations
    let pool = PgPool::connect(&url).await?;
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await?;

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
