///////////////////////////////////////////////////////////////////////////////
// NAME:            view.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Page for view of the Periodic Budget
//
// CREATED:         04/20/2022
//
// LAST EDITED:     07/16/2022
////

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::render::Render;

use super::data::DataView;

///////////////////////////////////////////////////////////////////////////////
// View
////

// Properties for this component
#[derive(Properties, PartialEq)]
pub struct ViewProperties {
    pub id: i32,
}

pub enum ViewMessage {
    Received(DataView),
}

// Component for this view
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
            Received(view) => {
                self.0 = Some(view);
                true
            },
        }
    }

    fn view(&self, _context: &Context<Self>) -> Html {
        match &self.0 {
            Some(context) => context.render(),
            None => html! { <p>{ "Loading..." }</p> },
        }
    }
}

impl View {
    // Submit HTTP request to get the budget and related data from the server
    fn request_context(&mut self, context: &Context<Self>) {
        use ViewMessage::*;

        self.0 = None;
        let link = context.link().callback(
            |budget: DataView| Received(budget)
        );
        let id = context.props().id.to_string();
        spawn_local(async move {
            DataView::get(&id, link).await.unwrap();
        });
    }
}

///////////////////////////////////////////////////////////////////////////////
