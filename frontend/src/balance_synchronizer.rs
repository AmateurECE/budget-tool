///////////////////////////////////////////////////////////////////////////////
// NAME:            balance_synchronizer.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Synchronizes initial balances from one month to another.
//
// CREATED:         05/09/2022
//
// LAST EDITED:     05/09/2022
////

use std::collections::HashMap;
use std::error::Error;
use budget_models::models::InitialBalance;

use crate::budgetizer::TrackedAccount;
use crate::network::fetch;
use crate::INITIAL_BALANCES_PATH;

pub struct BalanceSynchronizer {
    balances: Vec<InitialBalance>,
}

impl BalanceSynchronizer {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let request = web_sys::Request::new_with_str(INITIAL_BALANCES_PATH)
            .unwrap();
        let balances: Vec<InitialBalance> = fetch(request).await.unwrap();

        Ok(Self { balances })
    }

    pub async fn update_balances(
        &mut self, _balances: HashMap<String, TrackedAccount>
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
