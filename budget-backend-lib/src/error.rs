///////////////////////////////////////////////////////////////////////////////
// NAME:            error.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Some common error types for all crates.
//
// CREATED:         11/14/2022
//
// LAST EDITED:     11/14/2022
////

use std::error::Error;
use std::fmt;

///////////////////////////////////////////////////////////////////////////////
// MissingInstanceError
////

#[derive(Debug)]
pub struct MissingInstanceError {
    budget: i32,
    line_item: String,
}

impl MissingInstanceError {
    pub fn new(budget: i32, line_item: String) -> Self {
        Self { budget, line_item }
    }
}

impl Error for MissingInstanceError {}

impl fmt::Display for MissingInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "periodic budget {} has no instance of line item {}",
            self.budget, &self.line_item
        )
    }
}

///////////////////////////////////////////////////////////////////////////////
// MissingBudgetError
////

#[derive(Debug)]
pub struct MissingBudgetError(String);

impl MissingBudgetError {
    pub fn new(date: String) -> Self {
        Self(date)
    }
}

impl Error for MissingBudgetError {}

impl fmt::Display for MissingBudgetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "no periodic budget exists which includes date {}",
            &self.0
        )
    }
}

///////////////////////////////////////////////////////////////////////////////
