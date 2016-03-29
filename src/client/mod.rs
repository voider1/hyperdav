use hyper::client::Client as HttpClient;
use hyper::client::RequestBuilder;
use hyper::method::Method;
use hyper::client::IntoUrl;
use hyper::client::Body;
use std::io::Read;
use hyper::header::{Authorization, Basic};
use webdav::header::{Destination};

pub struct Client {
    http_client: HttpClient,
}

impl Client {
    pub fn new() -> Self {
        Client {
            http_client: HttpClient::new(),
        }
    }

    /// Put a file
    pub fn put<'a, U: IntoUrl>(&'a self, body: &'a mut Read, url: U) -> RequestBuilder<'a> {
        self.request(Method::Put, url).body(Body::ChunkedBody(body))
    }

    /// Create a directory
    pub fn create_dir<'a, U: IntoUrl>(&'a self, url: U) -> RequestBuilder<'a> {
        self.request(Method::Extension("MKCOL".to_string()), url)
    }

    /// Rename/move a directory or file
    pub fn rename<'a, U: IntoUrl>(&'a self, from: U, to: U) -> RequestBuilder<'a> {
        let mut req = self.request(Method::Extension("MOVE".to_string()), from);

        let actual_url = to.into_url().unwrap().to_string(); // FIXME :/

        req = req.header(Destination(actual_url));


        req
    }

    // curl --digest --user 'user:pass' -X MKCOL http://uwiki.net/uwiki/

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
