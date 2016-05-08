use hyper::client::Client as HttpClient;
use hyper::client::RequestBuilder;
use hyper::client::IntoUrl;
use hyper::client::Body;
use std::io::Read;
use hyper::header::{Authorization, Basic};
use webdav::header::{Destination, Depth};
use webdav::Method;
use webdav::response::{parse_propfind_response, PropfindResponse};
use Error;

pub struct Client {
    http_client: HttpClient,
}

impl Client {
    pub fn new() -> Self {
        Client { http_client: HttpClient::new() }
    }

    /// Get a file
    pub fn get<'a, U: IntoUrl + Clone>(&'a self, url: U) -> RequestBuilder<'a> {
        self.request(Method::Get, url)
    }

    /// Put a file
    pub fn put<'a, U: IntoUrl + Clone>(&'a self, body: &'a mut Read, url: U) -> RequestBuilder<'a> {
        self.request(Method::Put, url).body(Body::ChunkedBody(body))
    }

    /// Create a directory
    pub fn mkdir<'a, U: IntoUrl + Clone>(&'a self, url: U) -> RequestBuilder<'a> {
        self.request(Method::Mkcol, url)
    }

    /// Rename/move a directory or file
    pub fn mv<'a, U: IntoUrl + Clone>(&'a self, from: U, to: U) -> RequestBuilder<'a> {
        let req = self.request(Method::Move, from);

        // Set destination header
        if let Ok(url) = to.into_url() {
            req.header(Destination(url.to_string()))
        } else {
            req
        }
    }

    /// List files in a directory
    pub fn ls<'a, U: IntoUrl + Clone>(&'a self, url: U) -> Result<Vec<PropfindResponse>, Error> {
        let body = r#"<?xml version="1.0" encoding="utf-8" ?>
            <D:propfind xmlns:D="DAV:">
                <D:allprop/>
            </D:propfind>
        "#;

        let res = try!(self.request(Method::Propfind, url)
                           .header(Depth("Infinity".into()))
                           .body(body)
                           .send());

        Ok(try!(parse_propfind_response(res)))
    }


    // FIXME can we somehow parse the url AND get rid of Clone?
    pub fn request<'a, U: IntoUrl + Clone>(&'a self, method: Method, url: U) -> RequestBuilder<'a> {
        let auth_header = match url.clone().into_url() {
            Ok(url) => {
                match url.relative_scheme_data() {
                    Some(scheme) => {
                        Some(Authorization(Basic {
                            username: scheme.username.clone(),
                            password: scheme.password.clone(),
                        }))
                    }
                    None => None,
                }
            }
            Err(_) => None,
        };

        let req = self.http_client.request(method.into(), url);

        if let Some(header) = auth_header {
            req.header(header)
        } else {
            req
        }
    }
}
