///////////////////////////////////////////////////////////////////////////////
// NAME:            text.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Text input field
//
// CREATED:         05/16/2022
//
// LAST EDITED:     05/17/2022
////

use yew::prelude::*;

pub enum TextMessage {
}

#[derive(Properties, PartialEq)]
pub struct TextProperties {
}

pub struct TextInput {
}

impl Component for TextInput {
    type Message = TextMessage;
    type Properties = TextProperties;

    fn create(_context: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _context: &Context<Self>) -> Html {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
