///////////////////////////////////////////////////////////////////////////////
// NAME:            sea_orm_active_enums.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Enumerations in use in the project.
//
// CREATED:         07/05/2022
//
// LAST EDITED:     07/05/2022
////

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "accounttype")]
pub enum Accounttype {
    #[sea_orm(string_value = "checking")]
    Checking,
    #[sea_orm(string_value = "credit")]
    Credit,
    #[sea_orm(string_value = "loan")]
    Loan,
    #[sea_orm(string_value = "saving")]
    Saving,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "transactiontype")]
pub enum Transactiontype {
    #[sea_orm(string_value = "expense")]
    Expense,
    #[sea_orm(string_value = "income")]
    Income,
    #[sea_orm(string_value = "payment")]
    Payment,
    #[sea_orm(string_value = "transfer")]
    Transfer,
}

///////////////////////////////////////////////////////////////////////////////
