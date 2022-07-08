///////////////////////////////////////////////////////////////////////////////
// NAME:            initial_balances.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Account balance snapshot
//
// CREATED:         07/05/2022
//
// LAST EDITED:     07/07/2022
////

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "initial_balances")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub account: String,
    pub budget: i32,
    pub balance: i64,
    pub last_updated: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Account,
    PeriodicBudget,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Account => Entity::belongs_to(super::accounts::Entity)
                .from(Column::Account)
                .to(super::accounts::Column::Name)
                .into(),
            Self::PeriodicBudget =>
                Entity::belongs_to(super::periodic_budgets::Entity)
                .from(Column::Budget)
                .to(super::periodic_budgets::Column::Id)
                .into(),
        }
    }
}

impl Related<super::accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl Related<super::periodic_budgets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PeriodicBudget.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

///////////////////////////////////////////////////////////////////////////////
