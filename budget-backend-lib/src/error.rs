///////////////////////////////////////////////////////////////////////////////
// NAME:            error.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Some common error types for all crates.
//
// CREATED:         11/14/2022
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
