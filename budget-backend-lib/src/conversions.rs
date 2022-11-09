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
// LAST EDITED:     11/09/2022
////

use crate::entities::*;
use budget_models::models;
use sea_orm::Set;

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

impl Into<models::Transaction> for transactions::Model {
    fn into(self) -> models::Transaction {
        models::Transaction {
            id: self.id,
            summary: self.summary,
            date: self.date.into(),
            from_account: self.from_account,
            to_account: self.to_account,
            amount: self.amount,
        }
    }
}

impl From<models::NewTransaction> for transactions::ActiveModel {
    fn from(value: models::NewTransaction) -> Self {
        Self {
            summary: Set(value.summary),
            date: Set(value.date.into()),
            from_account: Set(value.from_account),
            to_account: Set(value.to_account),
            amount: Set(value.amount),
            ..Default::default()
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
