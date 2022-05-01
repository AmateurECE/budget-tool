///////////////////////////////////////////////////////////////////////////////
// NAME:            render.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Trait for rendering arbitrary objects into Html.
//
// CREATED:         04/30/2022
//
// LAST EDITED:     05/01/2022
////

use yew::Html;

pub trait Render {
    fn render(&self) -> Html;
}

pub trait RenderTable: Render {
    fn header() -> Html;
}

///////////////////////////////////////////////////////////////////////////////
