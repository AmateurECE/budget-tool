///////////////////////////////////////////////////////////////////////////////
// NAME:            amount.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic for handling monetary amounts in the model.
//
// CREATED:         07/08/2022
//
// LAST EDITED:     07/16/2022
////

///////////////////////////////////////////////////////////////////////////////
// Money
//  This type represents a value of money.
////

#[derive(Clone, Copy, Debug, Default)]
pub struct Money(i64);

impl Money {
    pub fn add(&mut self, value: Money) {
        self.0 += value.0;
    }

    pub fn subtract(&mut self, value: Money) {
        self.0 -= value.0;
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

impl ToString for Money {
    fn to_string(&self) -> String {
        let amount: f64 = (*self).into();
        format!("{:.2}", amount)
    }
}

///////////////////////////////////////////////////////////////////////////////
