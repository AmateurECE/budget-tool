///////////////////////////////////////////////////////////////////////////////
// NAME:            transaction.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Operations on transactions that are abstracted away from
//                  all database-handling code.
//
// CREATED:         11/17/2022
//
// LAST EDITED:     11/19/2022
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

use budget_models::*;
use sea_orm::prelude::*;
use sea_orm::Set;

use crate::prelude::*;

async fn create_transaction(
    db: &sea_orm::DatabaseConnection,
    data: transactions::ActiveModel,
    metadata: TransactionTypedMetadata,
) -> Result<(transactions::Model, i32), sea_orm::DbErr> {
    // Create the transactions instance
    let response = Transactions::insert(data).exec_with_returning(db).await?;

    // Create the planned/real transactions instance
    match metadata {
        TransactionTypedMetadata::Real(real) => {
            let model = real_transactions::ActiveModel {
                transaction: Set(response.id),
                line_item: Set(real.as_ref().map(|t| t.line_item.clone())),
                periodic_budget: Set(real.map(|t| t.periodic_budget)),
                ..Default::default()
            };
            let id = RealTransactions::insert(model)
                .exec(db)
                .await?
                .last_insert_id;
            Ok((response, id))
        }

        TransactionTypedMetadata::Planned(transaction) => {
            let model = planned_transactions::ActiveModel {
                transaction: Set(response.id),
                line_item: Set(transaction.line_item),
                periodic_budget: Set(transaction.periodic_budget),
                ..Default::default()
            };
            let id = PlannedTransactions::insert(model)
                .exec(db)
                .await?
                .last_insert_id;
            Ok((response, id))
        }
    }
}

pub async fn create(
    db: &sea_orm::DatabaseConnection,
    transaction: NewTransaction,
) -> Result<Transaction, sea_orm::DbErr> {
    let NewTransaction { metadata, series } = transaction;
    match series {
        NewTransactionSeries::Single(single) => {
            let (data, id) =
                create_transaction(db, single.into(), metadata.clone())
                    .await?;
            Ok(Transaction {
                id,
                series: TransactionSeries::Single(data.into()),
                metadata,
            })
        }

        NewTransactionSeries::Transfer {
            starting,
            completing,
        } => {
            let (start_data, _) =
                create_transaction(db, starting.into(), metadata.clone())
                    .await?;
            let mut completed: transactions::ActiveModel = completing.into();
            completed.completed_by = Set(Some(start_data.id));
            let (complete_data, id) =
                create_transaction(db, completed, metadata.clone()).await?;
            Ok(Transaction {
                id,
                series: TransactionSeries::Transfer {
                    starting: start_data.into(),
                    completing: complete_data.into(),
                },
                metadata,
            })
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
