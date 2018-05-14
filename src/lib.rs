#![deny(warnings)]
#![deny(missing_docs)]

//! A simple WebDav crate.

#[macro_use]
extern crate failure;
#[macro_use]
extern crate hyper;
extern crate reqwest;
extern crate xml;

mod client;
/// Contains our Error type.
pub mod error;
mod header;
mod response;

pub use client::{Client, ClientBuilder};
pub use error::Error;
