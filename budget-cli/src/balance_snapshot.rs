///////////////////////////////////////////////////////////////////////////////
// NAME:            balance_snapshot.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     CLI frontend for the collection of balance snapshots in
//                  the balance_snapshots table.
//
// CREATED:         11/27/2022
//
// LAST EDITED:     12/02/2022
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

use budget_backend_lib::prelude::*;
use clap::Subcommand;
use sea_orm::prelude::*;
use sea_orm::DatabaseConnection;
use sea_orm::QueryOrder;

async fn verify_account_snapshots<'a, I>(
    account: accounts::Model,
    mut account_snapshots: I,
    transactions: Vec<&transactions::Model>,
) -> anyhow::Result<()>
where
    I: Iterator<Item = &'a balance_snapshots::Model>,
{
    let previous = account_snapshots.next();
    if None == previous {
        return Ok(());
    }
    let mut previous = previous.unwrap();

    for snapshot in account_snapshots {
        let calculated = previous.amount
            + transactions
                .iter()
                .filter_map(|t| {
                    if t.date >= previous.date && t.date < snapshot.date {
                        Some(t.amount)
                    } else {
                        None
                    }
                })
                .sum::<i64>();
        if calculated != snapshot.amount {
            println!(
                "{}, {}: Calculated amount: {}, Snapshot amount: {}",
                account.name, snapshot.date, calculated, snapshot.amount
            );
        }
        previous = snapshot;
    }

    Ok(())
}

async fn verify(db: &DatabaseConnection) -> anyhow::Result<()> {
    // Trying to get away with as few transactions as possible.
    let accounts = Accounts::find().all(db).await?;
    let balance_snapshots = BalanceSnapshots::find()
        .order_by_asc(<BalanceSnapshots as EntityTrait>::Column::Date)
        .all(db)
        .await?;
    let transactions = Transactions::find()
        .order_by_asc(<BalanceSnapshots as EntityTrait>::Column::Date)
        .all(db)
        .await?;
    for account in accounts {
        print!("{}...", &account.name);
        let name = account.name.clone();
        let account_snapshots = balance_snapshots
            .iter()
            .filter(|snapshot| snapshot.account == name);
        let name = account.name.clone();
        let account_transactions = transactions
            .iter()
            .filter(|transaction| transaction.account == name)
            .collect::<Vec<&transactions::Model>>();
        verify_account_snapshots(
            account,
            account_snapshots,
            account_transactions,
        )
        .await?;
        println!("OK");
    }

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////
// Public Interface
////

#[derive(Subcommand)]
pub(crate) enum Verb {
    /// Verify the consistency of any balance snapshots by recalculating them.
    Verify,
}

pub(crate) async fn op(
    verb: &Verb,
    db: &DatabaseConnection,
) -> anyhow::Result<()> {
    match &verb {
        Verb::Verify => verify(db).await,
    }
}

///////////////////////////////////////////////////////////////////////////////
