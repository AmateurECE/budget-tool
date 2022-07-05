///////////////////////////////////////////////////////////////////////////////
// NAME:            periodic_budgets.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Budgets for a period of time.
//
// CREATED:         07/05/2022
//
// LAST EDITED:     07/05/2022
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
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

///////////////////////////////////////////////////////////////////////////////
