///////////////////////////////////////////////////////////////////////////////
// NAME:            select.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Select input component.
//
// CREATED:         05/16/2022
//
// LAST EDITED:     05/17/2022
////

use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::string::ToString;

use web_sys::HtmlSelectElement;
use yew::prelude::*;

pub enum SelectMessage {
    Selected(String),
}

#[derive(Properties, PartialEq)]
pub struct SelectProperties<S, T>
where S: ToString + PartialEq,
      T: From<String> + PartialEq,
{
    name: String,
    value: Rc<RefCell<T>>,
    options: Vec<S>,
    onchange: Callback<String>,
}

pub struct SelectInput<S, T>
where S: ToString + PartialEq,
      T: From<String> + PartialEq,
{
    phantom_s: PhantomData<S>,
    phantom_t: PhantomData<T>,
}

impl<S, T> Component for SelectInput<S, T>
where S: ToString + 'static + PartialEq,
      T: From<String> + 'static + PartialEq,
{
    type Message = SelectMessage;
    type Properties = SelectProperties<S, T>;

    fn create(_context: &Context<Self>) -> Self {
        Self {
            phantom_s: PhantomData,
            phantom_t: PhantomData,
        }
    }

    fn update(&mut self, context: &Context<Self>, message: Self::Message) ->
        bool
    {
        match message {
            SelectMessage::Selected(value) => {
                let mut state = context.props().value.borrow_mut();
                *state = value.into();
                true
            },
        }
    }

    fn view(&self, context: &Context<Self>) -> Html {
        let name = (*context.props().name).to_string();
        let onchange = context.props().onchange.clone();
        html! {
            <>
                <label for={name.clone()}>{&name}</label>
                <select id={name}
                 onchange={context.link().batch_callback(move |e: Event| {
                     if let Some(select) = e.target_dyn_into::<
                             HtmlSelectElement>() {
                         let value = select.value();
                         onchange.emit(value.clone());
                         Some(SelectMessage::Selected(value))
                     } else {
                         None
                     }
                 })}>{
                    context.props().options.iter().map(|i| {
                        let value = i.to_string();
                        html! {
                            <option value={value.clone()}>{value}</option>
                        }
                    }).collect::<Html>()
                }</select>
            </>
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
