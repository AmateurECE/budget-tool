//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize,
)]
#[sea_orm(table_name = "periodic_budgets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub start_date: DateTimeWithTimeZone,
    pub end_date: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::line_item_instances::Entity")]
    LineItemInstances,
}

impl Related<super::line_item_instances::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LineItemInstances.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
