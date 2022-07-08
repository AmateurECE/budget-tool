///////////////////////////////////////////////////////////////////////////////
// NAME:            amount.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic for handling monetary amounts in the model.
//
// CREATED:         07/08/2022
//
// LAST EDITED:     07/08/2022
////

///////////////////////////////////////////////////////////////////////////////
// Money
//  This type represents a value of money.
////

#[derive(Clone, Copy, Debug, Default)]
pub struct Money(i64);

impl Money {
    pub fn add(&mut self, value: i64) {
        self.0 += value;
    }
}

impl Into<f64> for Money {
    fn into(self) -> f64 {
        self.0 as f64 / 100.0
    }
}

///////////////////////////////////////////////////////////////////////////////
