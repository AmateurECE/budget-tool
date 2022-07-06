///////////////////////////////////////////////////////////////////////////////
// NAME:            transactions.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     A transaction model.
//
// CREATED:         07/05/2022
//
// LAST EDITED:     07/06/2022
////

use super::sea_orm_active_enums::Transactiontype;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "transactions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub line_item: i32,
    pub transaction_type: Transactiontype,
    #[sea_orm(column_type = "Text", nullable)]
    pub sending_account: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub receiving_account: Option<String>,
    pub transfer_fees: Option<i64>,
    #[sea_orm(column_type = "Text", nullable)]
    pub receiving_entity: Option<String>,
    pub amount: i64,
    #[sea_orm(column_type = "Text", nullable)]
    pub tags: Option<String>,
    pub send_date: DateTimeWithTimeZone,
    pub receive_date: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text", nullable)]
    pub corrects: Option<String>,
    pub periodic_budget: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    PeriodicBudget,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::PeriodicBudget => Entity::belongs_to(
                super::periodic_budgets::Entity).into(),
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
