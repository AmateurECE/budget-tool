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
// LAST EDITED:     07/04/2022
////

use budget_models::models;
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

impl Into<models::InitialBalance> for initial_balances::Model {
    fn into(self) -> models::InitialBalance {
        models::InitialBalance {
            id: self.id,
            account: self.account,
            budget: self.budget,
            balance: self.balance,
            last_updated: self.last_updated.into(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
