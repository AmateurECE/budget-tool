///////////////////////////////////////////////////////////////////////////////
// NAME:            view.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     View implementing the Transactions page.
//
// CREATED:         07/17/2022
//
// LAST EDITED:     07/17/2022
////

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::render::Render;

use super::data::DataView;

///////////////////////////////////////////////////////////////////////////////
// View
////

#[derive(Properties, PartialEq)]
pub struct ViewProperties {
    pub id: i32,
}

pub enum ViewMessage {
    Received(DataView),
}

#[derive(Default)]
pub struct View(Option<DataView>);

impl Component for View {
    type Message = ViewMessage;
    type Properties = ViewProperties;

    fn create(context: &Context<Self>) -> Self {
        let mut view = View::default();
        view.request_context(context);
        view
    }

    fn changed(&mut self, context: &Context<Self>) -> bool {
        self.request_context(context);
        true
    }

    fn update(&mut self, _context: &Context<Self>, message: Self::Message) ->
        bool
    {
        use ViewMessage::*;
        match message {
            Received(data) => {
                self.0 = Some(data);
                true
            }
        }
    }

    fn view(&self, _context: &Context<Self>) -> Html {
        match &self.0 {
            Some(transactions) => transactions.render(),
            None => html! { <p>{ "Loading..." }</p> },
        }
    }
}

impl View {
    fn request_context(&mut self, context: &Context<Self>) {
        use ViewMessage::*;

        self.0 = None;
        let link = context.link().callback(|data: DataView| Received(data));
        let id = context.props().id.to_string();
        spawn_local(async move {
            DataView::get(id.as_str(), link).await.unwrap();
        });
    }
}

///////////////////////////////////////////////////////////////////////////////
