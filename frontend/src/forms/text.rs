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

use core::marker::PhantomData;
use core::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;

pub enum TextMessage {
    Input,
}

#[derive(Properties, PartialEq)]
pub struct TextProperties<T>
where T: From<String> + PartialEq,
{
    name: String,
    value: Rc<RefCell<T>>,

    #[prop_or_default]
    onchange: Option<Callback<String>>,
}

pub struct TextInput<T>
where T: From<String> + PartialEq,
{
    input: NodeRef,
    phantom_t: PhantomData<T>,
}

impl<T> Component for TextInput<T>
where T: From<String> + 'static + PartialEq,
{
    type Message = TextMessage;
    type Properties = TextProperties<T>;

    fn create(_context: &Context<Self>) -> Self {
        Self {
            input: NodeRef::default(),
            phantom_t: PhantomData,
        }
    }

    fn view(&self, context: &Context<Self>) -> Html {
        let name = (*context.props().name).to_string();
        html! {
            <>
                <label for={name.clone()}>{&name}</label>
                <input id={name} type="text" ref={self.input.clone()}
                 onchange={context.link().callback(|_| TextMessage::Input)} />
            </>
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
