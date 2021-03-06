///////////////////////////////////////////////////////////////////////////////
// NAME:            budget_items.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     An item in a Budget
//
// CREATED:         07/05/2022
//
// LAST EDITED:     07/07/2022
////

use super::sea_orm_active_enums::Transactiontype;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "budget_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    #[sea_orm(column_type = "Text")]
    pub category: String,
    pub budgeted: i64,
    pub transaction_type: Transactiontype,
    #[sea_orm(column_type = "Text", nullable)]
    pub from_account: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub to_account: Option<String>,
    pub periodic_budget: i32,
    pub one_time_budget: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    PeriodicBudget,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::PeriodicBudget => Entity::belongs_to(
                super::periodic_budgets::Entity)
                .from(Column::PeriodicBudget)
                .to(super::periodic_budgets::Column::Id)
                .into(),
        }
    }
}

impl Related<super::periodic_budgets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PeriodicBudget.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

///////////////////////////////////////////////////////////////////////////////
