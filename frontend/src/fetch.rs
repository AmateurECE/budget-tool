///////////////////////////////////////////////////////////////////////////////
// NAME:            fetch.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Fetch implementation for the application.
//
// CREATED:         04/22/2022
//
// LAST EDITED:     04/23/2022
////

use js_sys::Promise;
use web_sys::console;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use wasm_bindgen_futures::JsFuture;

use serde::Deserialize;

///////////////////////////////////////////////////////////////////////////////
// http/web-sys Request conversion
////

pub enum RequestWrapper<B> {
    Http(http::Request<B>),
    Web(web_sys::Request),
}

impl<B> From<http::Request<B>> for RequestWrapper<B> {
    fn from(request: http::Request<B>) -> Self {
        RequestWrapper::Http(request)
    }
}

impl<B> From<web_sys::Request> for RequestWrapper<B> {
    fn from(request: web_sys::Request) -> Self {
        RequestWrapper::Web(request)
    }
}

impl<B> Into<http::Request<B>> for RequestWrapper<B> {
    fn into(self) -> http::Request<B> {
        todo!()
    }
}

impl<B> Into<web_sys::Request> for RequestWrapper<B> {
    fn into(self) -> web_sys::Request {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
// http/web-sys Response conversion
////

pub enum ResponseWrapper<B> {
    Http(http::Response<B>),
    Web(web_sys::Response),
}

impl<B> From<http::Response<B>> for ResponseWrapper<B> {
    fn from(response: http::Response<B>) -> Self {
        ResponseWrapper::Http(response)
    }
}

impl<B> From<web_sys::Response> for ResponseWrapper<B> {
    fn from(response: web_sys::Response) -> Self {
        ResponseWrapper::Web(response)
    }
}

impl<B> Into<http::Response<B>> for ResponseWrapper<B> {
    fn into(self) -> http::Response<B> {
        todo!()
    }
}

impl<B> Into<web_sys::Response> for ResponseWrapper<B> {
    fn into(self) -> web_sys::Response {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
// Fetch
////

pub async fn fetch<T, B, C>(request: http::Request<B>) ->
    Result<http::Response<C>, http::Error>
where for<'b> T: serde::Deserialize<'b>,
{
    // Get a reference to the window global
    let window = web_sys::window().unwrap();

//     // This closure converts the JSON to a Rust object.
//     let body_closure = Closure::once(Box::new(move |json: JsValue| {
//         let result: T = json.into_serde().unwrap();
//         callback(result);
//     }) as Box<dyn FnOnce(JsValue)>);

//     let response = JsFuture::from(window.fetch_with_request(&request)).await;
//     // This closure converts the response body to JSON.
//     assert!(value.is_instance_of::<Response>());
//     let response: Response = value.dyn_into().unwrap();
//     let json = JsFuture::from(response.json().unwrap()).await;
    todo!()
}

///////////////////////////////////////////////////////////////////////////////
// Eventual
////

// pub struct Eventual<T>
// where for<'a> T: serde::Deserialize<'a>,
// {
//     future: Option<VolatileFuture>,
//     eventual: Option<T>,
// }

// impl<T: 'static> Eventual<T>
// where for<'a> T: serde::Deserialize<'a>,
// {
//     pub fn with_request(request: Request) -> Self {
//         let mut eventual: Eventual<T> = Eventual {
//             future: None,
//             eventual: None,
//         };

//         eventual.future = Some(fetch(request, |result| {
//             eventual.eventual = Some(result);
//         }));

//         eventual
//     }
// }

///////////////////////////////////////////////////////////////////////////////
