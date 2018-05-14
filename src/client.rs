use std::borrow::Cow;
use std::io::Read;

use reqwest::{header, Body, Client as HttpClient, IntoUrl, Method, RequestBuilder, Response};

use super::error::Result;
use super::header::{Depth, Destination};
use super::response::{parse_propfind_response, PropfindResponse};
use Error;

/// The WebDAV client. Make a client for each server.
pub struct Client {
    http_client: HttpClient,
    credentials: Option<Credentials>,
}

/// The credentials for the WebDAV server.
pub struct Credentials {
    username: Cow<'static, str>,
    password: Cow<'static, str>,
}

/// The builder for the `Client`.
#[derive(Default)]
pub struct ClientBuilder {
    username: Option<Cow<'static, str>>,
    password: Option<Cow<'static, str>>,
}

impl ClientBuilder {
    /// Set the credentials for the server.
    pub fn credentials<S>(mut self, username: S, password: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }

    /// Build the WebDAV `Client`.
    pub fn build(self) -> Client {
        let ClientBuilder { username, password } = self;
        let http_client = HttpClient::new();
        let credentials = if let (Some(u), Some(p)) = (username, password) {
            Some(Credentials {
                username: u,
                password: p,
            })
        } else {
            None
        };

        Client {
            http_client,
            credentials,
        }
    }
}

impl Client {
    /// Get a file from the WebDAV server.
    pub fn get<U>(&self, url: U) -> Result<Response>
    where
        U: IntoUrl + Clone,
    {
        let res = self.request(Method::Get, url).send()?;

        if !res.status().is_success() {
            Err(Error::FailedRequest(res.status()))?;
        }

        Ok(res)
    }

    ///// Put a file on the WebDAV server, make sure the URL is pointing to the location where you
    /// want the file to be.
    pub fn put<R, U>(&self, body: R, url: U) -> Result<()>
    where
        U: IntoUrl + Clone,
        R: Read + Send + 'static,
    {
        let mut req = self.request(Method::Put, url);
        req.body(Body::new(body));
        println!("{:#?}", req);
        let res = req.send()?;

        if !res.status().is_success() {
            Err(Error::FailedRequest(res.status()))?;
        }

        Ok(())
    }

    /// Create a directory on the WebDAV server.
    pub fn mkcol<U>(&self, url: U) -> Result<()>
    where
        U: IntoUrl + Clone,
    {
        let res = self.request(Method::Extension("Mkcol".to_string()), url)
            .send()?;

        if !res.status().is_success() {
            Err(Error::FailedRequest(res.status()))?
        }

        Ok(())
    }

    /// Rename/move a directory or file on the WebDAV server.
    pub fn mv<U>(&self, from: U, to: U) -> Result<()>
    where
        U: IntoUrl + Clone,
    {
        let mut req = self.request(Method::Extension("Move".to_string()), from);

        // Set destination header
        let req = if let Ok(url) = to.into_url() {
            req.header(Destination(url.to_string()))
        } else {
            &mut req
        };

        let res = req.send()?;

        if !res.status().is_success() {
            Err(Error::FailedRequest(res.status()))?;
        }
        Ok(())
    }

    /// List files in a directory on the WebDAV server.
    pub fn list<U>(&self, url: U) -> Result<Vec<PropfindResponse>>
    where
        U: IntoUrl + Clone,
    {
        let body = r#"<?xml version="1.0" encoding="utf-8" ?>
            <D:propfind xmlns:D="DAV:">
                <D:allprop/>
            </D:propfind>
        "#;

        let res = self.request(Method::Extension("Propfind".to_string()), url)
            .header(Depth("Infinity".into()))
            .body(body)
            .send();

        println!("{:#?}", res);
        let res = res?;

        if !res.status().is_success() {
            Err(Error::FailedRequest(res.status()))?;
        }

        Ok(parse_propfind_response(res)?)
    }

    /// Prepare a request for use.
    pub fn request<U>(&self, method: Method, url: U) -> RequestBuilder
    where
        U: IntoUrl + Clone,
    {
        let mut request = self.http_client.request(method, url);

        if let Some(Credentials {
            ref username,
            ref password,
        }) = self.credentials
        {
            request.header(header::Authorization(header::Basic {
                username: username.to_string(),
                password: Some(password.to_string()),
            }));
        }

        request
    }
}
