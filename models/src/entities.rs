///////////////////////////////////////////////////////////////////////////////
// NAME:            entities.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entities--types that have behavior and wrap DB objects.
//
// CREATED:         04/14/2022
//
// LAST EDITED:     04/25/2022
////

use crate::models::{Account, Transaction};

pub struct Checking;
impl Into<Account> for Checking {
    fn into(self) -> Account { unimplemented!() }
}

pub struct Saving;
impl Into<Account> for Saving {
    fn into(self) -> Account { unimplemented!() }
}

pub struct CreditCard;
impl Into<Account> for CreditCard {
    fn into(self) -> Account { unimplemented!() }
}

pub struct Loan;
impl Into<Account> for Loan {
    fn into(self) -> Account { unimplemented!() }
}

pub struct Expense;
impl Into<Transaction> for Expense {
    fn into(self) -> Transaction { unimplemented!() }
}

pub struct Income;
impl Into<Transaction> for Income {
    fn into(self) -> Transaction { unimplemented!() }
}

pub struct Transfer;
impl Into<Transaction> for Transfer {
    fn into(self) -> Transaction { unimplemented!() }
}

pub struct Payment;
impl Into<Transaction> for Payment {
    fn into(self) -> Transaction { unimplemented!() }
}

///////////////////////////////////////////////////////////////////////////////
