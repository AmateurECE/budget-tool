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
// LAST EDITED:     07/06/2022
////

use std::convert::TryInto;
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

impl Into<models::TransactionType> for sea_orm_active_enums::Transactiontype {
    fn into(self) -> models::TransactionType {
        match self {
            Self::Expense => models::TransactionType::Expense,
            Self::Income => models::TransactionType::Income,
            Self::Transfer => models::TransactionType::Transfer,
            Self::Payment => models::TransactionType::Payment,
        }
    }
}

impl TryInto<models::Transaction> for transactions::Model {
    type Error = serde_json::Error;
    fn try_into(self) -> Result<models::Transaction, Self::Error> {
        let tags = self.tags
            .map(|tags| serde_json::from_str::<Vec<String>>(&tags))
            .transpose()?;
        let corrects = self.corrects
            .map(|corrects| serde_json::from_str::<Vec<i32>>(&corrects))
            .transpose()?;
        Ok(models::Transaction {
            id: self.id,
            description: self.description,
            line_item: self.line_item,
            transaction_type: self.transaction_type.into(),
            sending_account: self.sending_account,
            receiving_account: self.receiving_account,
            transfer_fees: self.transfer_fees,
            receiving_entity: self.receiving_entity,
            amount: self.amount,
            tags,
            send_date: self.send_date.into(),
            receive_date: self.receive_date.map(|date| date.into()),
            corrects,
            periodic_budget: self.periodic_budget,
        })
    }
}

impl Into<models::BudgetItem> for budget_items::Model {
    fn into(self) -> models::BudgetItem {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
