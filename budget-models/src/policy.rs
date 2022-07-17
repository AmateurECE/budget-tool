///////////////////////////////////////////////////////////////////////////////
// NAME:            policy.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Policies for business logic.
//
// CREATED:         07/16/2022
//
// LAST EDITED:     07/16/2022
////

use chrono::{DateTime, offset::Utc};
use crate::models;

pub struct TransactionReceivedPolicy(DateTime<Utc>);

impl TransactionReceivedPolicy {
    pub fn new() -> Self {
        Self(Utc::now())
    }

    pub fn is_received(&self, transaction: &models::Transaction) -> bool {
        transaction.receive_date.map(|date| date < self.0).unwrap_or(false)
    }
}

///////////////////////////////////////////////////////////////////////////////
