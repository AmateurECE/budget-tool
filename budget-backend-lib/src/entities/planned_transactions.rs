//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.4

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
    #[sea_orm(column_type = "Text")]
    pub line_item: String,
    pub periodic_budget: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::line_items::Entity",
        from = "Column::LineItem",
        to = "super::line_items::Column::Summary",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    LineItems,
    #[sea_orm(
        belongs_to = "super::periodic_budgets::Entity",
        from = "Column::PeriodicBudget",
        to = "super::periodic_budgets::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    PeriodicBudgets,
    #[sea_orm(
        belongs_to = "super::transactions::Entity",
        from = "Column::Transaction",
        to = "super::transactions::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Transactions,
}

impl Related<super::line_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LineItems.def()
    }
}

impl Related<super::periodic_budgets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PeriodicBudgets.def()
    }
}

impl Related<super::transactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transactions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
