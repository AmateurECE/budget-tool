///////////////////////////////////////////////////////////////////////////////
// NAME:            pages.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Module containing pages in use by the application.
//
// CREATED:         04/20/2022
//
// LAST EDITED:     05/10/2022
////

mod not_found;
mod periodic_budget;
mod transactions;

pub use not_found::NotFoundView;
pub use periodic_budget::PeriodicBudgetView;
pub use transactions::TransactionView;

///////////////////////////////////////////////////////////////////////////////
