use std::borrow::Cow;
use std::io::Read;
use std::path::Path;

use reqwest::{header, Body, Client as HttpClient, Method, RequestBuilder, Response};

use super::error::Result;
use super::header::{Depth, Destination};
use super::response::{parse_propfind_response, PropfindResponse};
use Error;

/// The WebDAV client. Make a client for each server.
pub struct Client {
    http_client: HttpClient,
    webdav_url: Cow<'static, str>,
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
    pub fn build<S>(self, webdav_url: S) -> Client
    where
        S: Into<Cow<'static, str>>,
    {
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
            webdav_url: webdav_url.into(),
            credentials,
        }
    }
}

impl Client {
    /// Get a file from the WebDAV server.
    pub fn get(&self, path: impl AsRef<Path>) -> Result<Response> {
        let res = self.request(Method::Get, path).send()?;

        if !res.status().is_success() {
            Err(Error::FailedRequest(res.status()))?;
        }

        Ok(res)
    }

    /// Put a file on the WebDAV server, make sure the URL is pointing to the location where you
    /// want the file to be.
    pub fn put<R>(&self, body: R, path: impl AsRef<Path>) -> Result<()>
    where
        R: Read + Send + 'static,
    {
        let mut req = self.request(Method::Put, path);
        req.body(Body::new(body)).send()?;

        if !res.status().is_success() {
            Err(Error::FailedRequest(res.status()))?;
        }

        Ok(())
    }

    /// Create a directory on the WebDAV server.
    pub fn mkcol(&self, path: impl AsRef<Path>) -> Result<()> {
        let res = self.request(Method::Extension("Mkcol".to_string()), path)
            .send()?;

        if !res.status().is_success() {
            Err(Error::FailedRequest(res.status()))?
        }

        Ok(())
    }

    /// Rename/move a directory or file on the WebDAV server.
    pub fn mv<P>(&self, from: P, to: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let mut req = self.request(Method::Extension("Move".to_string()), from);

        // Set destination header
        let to = to.as_ref().to_str().unwrap();
        let url = [self.webdav_url.as_ref(), &to].join("/");

        req.header(Destination(url.to_string()));

        let res = req.send()?;

        if !res.status().is_success() {
            Err(Error::FailedRequest(res.status()))?;
        }
        Ok(())
    }

    /// List files in a directory on the WebDAV server.
    pub fn list(&self, path: impl AsRef<Path>) -> Result<Vec<PropfindResponse>> {
        let body = r#"<?xml version="1.0" encoding="utf-8" ?>
            <D:propfind xmlns:D="DAV:">
                <D:allprop/>
            </D:propfind>
        "#;

        let res = self.request(Method::Extension("Propfind".to_string()), path)
            .header(Depth("Infinity".into()))
            .body(body)
            .send()?;

        if !res.status().is_success() {
            Err(Error::FailedRequest(res.status()))?;
        }

        Ok(parse_propfind_response(res)?)
    }

    /// Prepare a request for use.
    pub fn request(&self, method: Method, path: impl AsRef<Path>) -> RequestBuilder {
        let path = path.as_ref().to_str().unwrap();
        let url = [self.webdav_url.as_ref(), &path].join("/");
        let mut request = self.http_client.request(method, &url);

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
