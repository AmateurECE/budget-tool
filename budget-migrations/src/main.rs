///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Budget migration manager
//
// CREATED:         09/18/2022
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

use budget_backend_lib::secret::SecretManager;
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
    sqlx::migrate!("../migrations")
        .run(&pool)
        .await?;

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
