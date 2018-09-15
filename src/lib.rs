extern crate canapi;
extern crate serde_json;
extern crate serde_qs;
extern crate stdweb;

use canapi::{Endpoint, Error, Fetch};
use serde_qs as qs;
use stdweb::web::{XmlHttpRequest, XhrReadyState};

pub struct WebFetch;

impl Fetch for WebFetch {
    fn fetch<T: Endpoint>(method: &'static str, url: String, query: Option<T>) -> Result<T, Error> {
        let req = XmlHttpRequest::new();
        let url = if method == "GET" {
            if let Some(q) = query {
                let query = qs::to_string(&q);
                if let Err(_) = query {
                    return Err(Error::SerDe(String::from("Query serialization error")))
                }

                format!("{}?{}", url, query.unwrap())
            } else {
                url
            }
        } else {
            url
        };
        req.open(method, url.as_ref()).expect("XHR.open error");
        req.send().expect("XHR.send error");
        while req.ready_state() == XhrReadyState::Loading {}
        if req.ready_state() == XhrReadyState::Done {
            let res = req.response_text().expect("Network error when fetching API endpoint")
                .expect("Response body is empty");
            serde_json::from_str(res.as_ref()).map_err(|_| Error::SerDe(String::from("Error in the response")))
        } else {
            Err(Error::Fetch(String::from("Request didn't ended correctly")))
        }
    }
}
