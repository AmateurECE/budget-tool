///////////////////////////////////////////////////////////////////////////////
// NAME:            accounts.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     An Account model
//
// CREATED:         07/05/2022
//
// LAST EDITED:     07/09/2022
////

use super::sea_orm_active_enums::Accounttype;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "accounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub name: String,
    pub account_type: Accounttype,
    pub date_opened: DateTimeWithTimeZone,
    pub date_closed: Option<DateTimeWithTimeZone>,
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
