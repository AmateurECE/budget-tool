///////////////////////////////////////////////////////////////////////////////
// NAME:            total.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic for calculating particular kinds of totals.
//
// CREATED:         07/07/2022
//
// LAST EDITED:     07/15/2022
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
