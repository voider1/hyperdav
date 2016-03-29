use hyper::client::Client as HttpClient;
use hyper::client::RequestBuilder;
use hyper::method::Method;
use hyper::client::IntoUrl;
use hyper::client::Body;
use std::io::Read;
use hyper::header::{Authorization, Basic};

pub struct Client {
    http_client: HttpClient,
}

impl Client {
    pub fn new() -> Self {
        Client {
            http_client: HttpClient::new(),
        }
    }

    pub fn put<'a, U: IntoUrl>(&'a self, body: &'a mut Read, url: U) -> RequestBuilder<'a> {
        self.request(Method::Put, url).body(Body::ChunkedBody(body))
    }

    // FIXME this function is a mess
    pub fn request<'a, U: IntoUrl>(&'a self, method: Method, url: U) -> RequestBuilder<'a> {
        let actual_url = url.into_url().unwrap(); // FIXME :/

        let mut req = self.http_client.request(method, actual_url.clone());

        // Set auth header
        if let Some(scheme) = actual_url.relative_scheme_data() {
            if scheme.username != "" {
                req = req.header(Authorization(Basic {
                    username: scheme.username.clone(),
                    password: scheme.password.clone(),
                }));
            }
        }

        req
    }
}
