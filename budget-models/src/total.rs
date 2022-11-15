///////////////////////////////////////////////////////////////////////////////
// NAME:            total.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic for calculating particular kinds of totals.
//
// CREATED:         07/07/2022
//
// LAST EDITED:     11/15/2022
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

use crate::calculation::Calculation;
use crate::money::Money;

///////////////////////////////////////////////////////////////////////////////
// BurnUpTotal
//  The BurnUpTotal represents an incremental tracking measure of "Burn-up" of
//  a thing. As transactions are "applied" to the thing, the total increases
//  monotonically by the amount of each transaction.
////

#[derive(Clone, Copy, Debug, Default)]
pub struct BurnUpTotal(Money);

impl Calculation for BurnUpTotal {
    type Input = Money;
    type Result = Money;
    fn apply(&mut self, input: Self::Input) {
        self.0.add(input);
    }

    fn calculate(&self) -> &Money {
        &self.0
    }
}

///////////////////////////////////////////////////////////////////////////////
