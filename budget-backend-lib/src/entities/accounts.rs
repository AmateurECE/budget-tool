//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.4

use super::sea_orm_active_enums::Accounttype;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize,
)]
#[sea_orm(table_name = "accounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub name: String,
    pub account_type: Accounttype,
    pub date_opened: DateTimeWithTimeZone,
    pub date_closed: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::initial_balances::Entity")]
    InitialBalances,
    #[sea_orm(has_many = "super::transactions::Entity")]
    Transactions,
}

impl Related<super::initial_balances::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InitialBalances.def()
    }
}

impl Related<super::transactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transactions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
