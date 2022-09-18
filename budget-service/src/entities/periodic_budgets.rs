///////////////////////////////////////////////////////////////////////////////
// NAME:            periodic_budgets.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Budgets for a period of time.
//
// CREATED:         07/05/2022
//
// LAST EDITED:     07/06/2022
////

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "periodic_budgets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub start_date: DateTimeWithTimeZone,
    pub end_date: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    BudgetItem,
    InitialBalance,
    Transaction,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::BudgetItem => {
                Entity::has_many(super::budget_items::Entity).into()
            }
            Self::InitialBalance => {
                Entity::has_many(super::initial_balances::Entity).into()
            }
            Self::Transaction => {
                Entity::has_many(super::transactions::Entity).into()
            }
        }
    }
}

impl Related<super::transactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transaction.def()
    }
}

impl Related<super::initial_balances::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InitialBalance.def()
    }
}

impl Related<super::budget_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BudgetItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

///////////////////////////////////////////////////////////////////////////////
