///////////////////////////////////////////////////////////////////////////////
// NAME:            conversions.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic for conversion between persistence entities and model
//                  entities.
//
// CREATED:         07/04/2022
//
// LAST EDITED:     07/04/2022
////

use budget_models::models;
use crate::entities::*;

impl Into<models::Account> for accounts::Model {
    fn into(self) -> models::Account {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
