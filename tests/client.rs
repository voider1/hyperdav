extern crate hyper;
extern crate hyperdav;
extern crate url;
extern crate uuid;

use hyper::status::StatusCode;
use hyperdav::Client;
use url::Url;

const OWNCLOUD_URL: &'static str = "https://test:test@demo.owncloud.org/remote.php/webdav/";

macro_rules! url {
    ($e:expr) => (Url::parse(&format!("{}{}", OWNCLOUD_URL, $e)).unwrap());
}

macro_rules! random_url {
    () => (url!(uuid::Uuid::new_v4()));
}

#[test]
fn get() {
    let client = Client::new();
    let url = random_url!();
    let mut f = std::io::empty();
    let res = client.put(&mut f, url.clone()).send().unwrap();
    assert_eq!(res.status, StatusCode::Created);
    let res = client.get(url).send().unwrap();
    assert_eq!(res.status, StatusCode::Ok);
    // TODO test body
}

#[test]
fn put() {
    let client = Client::new();
    let mut f = std::io::empty();
    let res = client.put(&mut f, random_url!()).send().unwrap();
    assert_eq!(res.status, StatusCode::Created);
}

#[test]
fn mkdir() {
    let client = Client::new();
    let res = client.mkdir(random_url!()).send().unwrap();
    assert_eq!(res.status, StatusCode::Created);
}

#[test]
fn mv() {
    let client = Client::new();
    let from = random_url!();
    let to = random_url!();
    let res = client.mkdir(from.clone()).send().unwrap();
    assert_eq!(res.status, StatusCode::Created);
    let res = client.mv(from, to).send().unwrap();
    assert_eq!(res.status, StatusCode::Created);
}

#[test]
fn ls() {
    let client = Client::new();
    let folder_url = random_url!();
    let res = client.mkdir(folder_url.clone()).send().unwrap();
    assert_eq!(res.status, StatusCode::Created);
    let res = client.ls(OWNCLOUD_URL).unwrap();
    let mut found = false;
    for item in res {
        if item.href.unwrap() == format!("{}/", folder_url.path()) {
            found = true;
        }
    }
    assert!(found);
}
