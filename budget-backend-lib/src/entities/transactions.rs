//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize,
)]
#[sea_orm(table_name = "transactions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub summary: String,
    pub date: DateTimeWithTimeZone,
    #[sea_orm(column_type = "Text", nullable)]
    pub from_account: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub to_account: Option<String>,
    pub amount: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::real_transactions::Entity")]
    RealTransactions,
    #[sea_orm(has_many = "super::planned_transactions::Entity")]
    PlannedTransactions,
}

impl Related<super::real_transactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RealTransactions.def()
    }
}

impl Related<super::planned_transactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlannedTransactions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
