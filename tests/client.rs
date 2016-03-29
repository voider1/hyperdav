extern crate hyper;
extern crate hyperdav;
extern crate url;
extern crate uuid;

use hyper::status::StatusCode;
use hyperdav::Client;
use url::Url;

macro_rules! url {
    ($e:expr) => (Url::parse(&format!("https://test:test@demo.owncloud.org/remote.php/webdav/{}", $e)).unwrap());
}

macro_rules! random_url {
    () => (url!(uuid::Uuid::new_v4()));
}

#[test]
fn put() {
    let client = Client::new();
    let mut f = std::io::empty();
    let res = client.put(&mut f, random_url!()).send().unwrap();
    assert_eq!(res.status, StatusCode::Created);
}
