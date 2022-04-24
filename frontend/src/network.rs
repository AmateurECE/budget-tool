///////////////////////////////////////////////////////////////////////////////
// NAME:            network.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Fetch implementation for the application.
//
// CREATED:         04/22/2022
//
// LAST EDITED:     04/24/2022
////

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use serde::Deserialize;

#[derive(Debug)]
pub struct FetchError(String);

///////////////////////////////////////////////////////////////////////////////
// Fetch
////

pub async fn fetch<T>(request: web_sys::Request) -> Result<T, FetchError>
where for <'a> T: Deserialize<'a>
{
    // Get a reference to the window global
    let window = web_sys::window().unwrap();

    // Submit the request
    let value = JsFuture::from(window.fetch_with_request(&request)).await
        .unwrap();

    // Convert the response body to JSON.
    assert!(value.is_instance_of::<web_sys::Response>());
    let response: web_sys::Response = value.dyn_into().unwrap();
    let json = JsFuture::from(response.json().unwrap()).await.unwrap();

    // Get the result
    json.into_serde::<T>().map_err(|err| FetchError(err.to_string()))
}

///////////////////////////////////////////////////////////////////////////////
