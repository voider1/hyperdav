extern crate hyper;
extern crate hyperdav;
extern crate url;
extern crate uuid;

use hyperdav::webdav::Client;
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
    client.put(&mut f, url.clone()).unwrap();
    client.get(url).unwrap();
}

#[test]
fn put() {
    let client = Client::new();
    let mut f = std::io::empty();
    client.put(&mut f, random_url!()).unwrap();
}

#[test]
fn mkdir() {
    let client = Client::new();
    client.mkdir(random_url!()).unwrap();
}

#[test]
fn mv() {
    let client = Client::new();
    let from = random_url!();
    let to = random_url!();
    client.mkdir(from.clone()).unwrap();
    client.mv(from, to).unwrap();
}

#[test]
fn ls() {
    let client = Client::new();
    let folder_url = random_url!();
    client.mkdir(folder_url.clone()).unwrap();
    let res = client.ls(OWNCLOUD_URL).unwrap();
    let mut found = false;
    for item in res {
        if item.href == format!("{}/", folder_url.path()) {
            found = true;
        }
    }
    assert!(found);
}
