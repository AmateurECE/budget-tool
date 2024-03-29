///////////////////////////////////////////////////////////////////////////////
// NAME:            money.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic for handling monetary amounts in the model.
//
// CREATED:         07/08/2022
//
// LAST EDITED:     11/19/2022
//
// Copyright 2022, Ethan D. Twardy
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
////

use serde::{Deserialize, Serialize};
use std::fmt;

///////////////////////////////////////////////////////////////////////////////
// Money
//  This type represents a value of money.
////

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize, Serialize,
)]
pub struct Money(i64);

impl Money {
    pub fn add(&mut self, value: Money) {
        self.0 += value.0;
    }

    pub fn subtract(&mut self, value: Money) {
        self.0 -= value.0;
    }
}

impl From<f64> for Money {
    fn from(value: f64) -> Self {
        Self((value * 100.0) as i64)
    }
}

impl Into<f64> for Money {
    fn into(self) -> f64 {
        self.0 as f64 / 100.0
    }
}

impl From<i64> for Money {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl Into<i64> for Money {
    fn into(self) -> i64 {
        self.0
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let amount: f64 = (*self).into();
        write!(f, "{:.2}", amount)
    }
}

///////////////////////////////////////////////////////////////////////////////
