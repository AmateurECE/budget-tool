///////////////////////////////////////////////////////////////////////////////
// NAME:            periodic_budget.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     View/Presenter related to the periodic budget view.
//
// CREATED:         07/15/2022
//
// LAST EDITED:     07/15/2022
////

mod account;
mod budget_item;
mod presenter;
mod view;

pub use presenter::Presenter as PeriodicBudgetPresenter;
pub use view::View as PeriodicBudgetView;

///////////////////////////////////////////////////////////////////////////////
