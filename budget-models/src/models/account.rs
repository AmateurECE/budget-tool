///////////////////////////////////////////////////////////////////////////////
// NAME:            account.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     An account with a financial institution.
//
// CREATED:         07/05/2022
//
// LAST EDITED:     07/05/2022
////

use serde::{Serialize, Deserialize};
use super::AccountType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub account_type: AccountType,
}

///////////////////////////////////////////////////////////////////////////////
