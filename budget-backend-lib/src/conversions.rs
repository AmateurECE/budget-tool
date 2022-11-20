///////////////////////////////////////////////////////////////////////////////
// NAME:            conversions.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic for conversion between persistence entities and model
//                  entities.
//
// CREATED:         07/04/2022
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

use budget_models::models;
use chrono::FixedOffset;
use sea_orm::Set;

use crate::entities::*;

impl Into<models::AccountType> for sea_orm_active_enums::Accounttype {
    fn into(self) -> models::AccountType {
        match self {
            Self::Checking => models::AccountType::Checking,
            Self::Saving => models::AccountType::Saving,
            Self::Credit => models::AccountType::Credit,
            Self::Loan => models::AccountType::Loan,
        }
    }
}

impl Into<models::Account> for accounts::Model {
    fn into(self) -> models::Account {
        models::Account {
            name: self.name,
            account_type: self.account_type.into(),
            date_opened: self.date_opened.into(),
            date_closed: self.date_closed.map(|date| date.into()),
        }
    }
}

impl Into<models::PeriodicBudget> for periodic_budgets::Model {
    fn into(self) -> models::PeriodicBudget {
        models::PeriodicBudget {
            id: self.id,
            start_date: self.start_date.into(),
            end_date: self.end_date.into(),
        }
    }
}

impl Into<models::TransactionData> for transactions::Model {
    fn into(self) -> models::TransactionData {
        models::TransactionData {
            id: self.id,
            summary: self.summary,
            date: self.date.into(),
            account: self.account,
            amount: self.amount,
            completed_by: self.completed_by,
        }
    }
}

impl From<models::NewTransactionData> for transactions::ActiveModel {
    fn from(value: models::NewTransactionData) -> Self {
        transactions::ActiveModel {
            summary: Set(value.summary),
            amount: Set(value.amount.into()),
            account: Set(value.account),
            date: Set(value
                .date
                .with_timezone(&FixedOffset::east_opt(0).unwrap())),
            ..Default::default()
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
