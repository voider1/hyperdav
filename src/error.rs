use webdav::response::PropfindParseError;
use hyper;
use std;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
    Http(hyper::Error),
    PropfindParse(PropfindParseError),
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Error::Http(e)
    }
}

impl From<PropfindParseError> for Error {
    fn from(e: PropfindParseError) -> Self {
        Error::PropfindParse(e)
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            Http(ref e) => e.description(),
            PropfindParse(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        use self::Error::*;
        match *self {
            Http(ref e) => Some(e as &StdError),
            PropfindParse(ref e) => Some(e as &StdError), 
        }
    }
}
