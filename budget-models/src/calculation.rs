///////////////////////////////////////////////////////////////////////////////
// NAME:            calculation.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Trait for abstracting calculations.
//
// CREATED:         07/12/2022
//
// LAST EDITED:     07/14/2022
////

pub trait Calculation {
    type Input;
    type Result;
    fn apply(&mut self, input: &Self::Input);
    fn calculate(&self) -> &Self::Result;
}

///////////////////////////////////////////////////////////////////////////////
