//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize,
)]
#[sea_orm(table_name = "line_item_instances")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub summary: String,
    pub periodic_budget: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub from_account: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub to_account: Option<String>,
    pub amount: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::accounts::Entity",
        from = "Column::FromAccount",
        to = "super::accounts::Column::Name",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Accounts2,
    #[sea_orm(
        belongs_to = "super::periodic_budgets::Entity",
        from = "Column::PeriodicBudget",
        to = "super::periodic_budgets::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    PeriodicBudgets,
    #[sea_orm(
        belongs_to = "super::line_items::Entity",
        from = "Column::Summary",
        to = "super::line_items::Column::Summary",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    LineItems,
    #[sea_orm(
        belongs_to = "super::accounts::Entity",
        from = "Column::ToAccount",
        to = "super::accounts::Column::Name",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Accounts1,
    #[sea_orm(has_many = "super::real_transactions::Entity")]
    RealTransactions,
    #[sea_orm(has_many = "super::planned_transactions::Entity")]
    PlannedTransactions,
}

impl Related<super::periodic_budgets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PeriodicBudgets.def()
    }
}

impl Related<super::line_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LineItems.def()
    }
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
