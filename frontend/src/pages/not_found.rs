///////////////////////////////////////////////////////////////////////////////
// NAME:            not_found.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     A page for URL's that result in HTTP 404
//
// CREATED:         04/20/2022
//
// LAST EDITED:     04/21/2022
////

use yew::{Component, Context, Html, html};

pub struct NotFoundView;

impl Component for NotFoundView {
    type Message = ();
    type Properties = ();

    fn create(_context: &Context<Self>) -> Self {
        NotFoundView
    }

    fn view(&self, _context: &Context<Self>) -> Html {
        html! { <p>{ "Not Found!" }</p> }
    }
}

///////////////////////////////////////////////////////////////////////////////
