use failure::Error as FailureError;
use reqwest::StatusCode;

use super::response::PropfindParseError;

/// Result which uses failure::Error by default.
pub type Result<T> = ::std::result::Result<T, FailureError>;

/// Our custom error type using Failure.
#[derive(Fail, Debug)]
pub enum Error {
    /// Used when a networking error occurs.
    #[fail(display = "{:?}", _0)]
    NetworkingError(#[cause] ::reqwest::Error),
    /// Used when propfind fails.
    #[fail(display = "Failed to propfind: {}", _0)]
    PropfindParse(PropfindParseError),
    /// Used when the request failes.
    #[fail(display = "Request failed, error code: {}", _0)]
    FailedRequest(StatusCode),
}
