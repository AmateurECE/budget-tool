///////////////////////////////////////////////////////////////////////////////
// NAME:            fetch.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Fetch implementation for the application.
//
// CREATED:         04/22/2022
//
// LAST EDITED:     04/24/2022
////

use web_sys::RequestInit;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

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

pub async fn fetch<T, B>(_request: http::Request<B>) ->
    Result<http::Response<T>, http::Error>
where for<'b> T: serde::Deserialize<'b>,
{
    // Generate the response
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = web_sys::Request::new_with_str_and_init(
        "/api/periodic_budgets", &opts).unwrap();
    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json").unwrap();

    // Get a reference to the window global
    let window = web_sys::window().unwrap();

    // Submit the request
    let value = JsFuture::from(window.fetch_with_request(&request))
        .await.unwrap();

    // Convert the response body to JSON.
    assert!(value.is_instance_of::<web_sys::Response>());
    let response: web_sys::Response = value.dyn_into().unwrap();
    let json = JsFuture::from(response.json().unwrap()).await.unwrap();

    // Get the result
    let result: T = json.into_serde().unwrap();
    Ok(http::Response::new(result))
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
