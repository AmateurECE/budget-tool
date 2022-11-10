//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize,
)]
#[sea_orm(table_name = "planned_transactions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub transaction: i32,
    pub line_item_instance: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::line_item_instances::Entity",
        from = "Column::LineItemInstance",
        to = "super::line_item_instances::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    LineItemInstances,
    #[sea_orm(
        belongs_to = "super::transactions::Entity",
        from = "Column::Transaction",
        to = "super::transactions::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Transactions,
}

impl Related<super::line_item_instances::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LineItemInstances.def()
    }
}

impl Related<super::transactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transactions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
