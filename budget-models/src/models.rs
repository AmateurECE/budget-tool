///////////////////////////////////////////////////////////////////////////////
// NAME:            models.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Models
//
// CREATED:         04/10/2022
//
// LAST EDITED:     04/10/2022
////

use serde::{Serialize, Deserialize};
use super::schema::accounts;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Account {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="accounts"]
pub struct NewAccount<'a> {
    pub name: &'a str,
}

///////////////////////////////////////////////////////////////////////////////
