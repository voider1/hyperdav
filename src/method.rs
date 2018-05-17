use hyper::method::Method as HyperMethod;
use std::convert::AsRef;
use std::str::FromStr;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Method {
    Get,
    Put,
    Mkcol,
    Move,
    Propfind,
}

impl Into<HyperMethod> for Method {
    fn into(self) -> HyperMethod {
        match self {
            Method::Get => HyperMethod::Get,
            Method::Put => HyperMethod::Put,
            _ => HyperMethod::Extension(self.as_ref().to_string()),
        }
    }
}

impl AsRef<str> for Method {
    fn as_ref(&self) -> &str {
        match *self {
            Method::Get => "GET",
            Method::Put => "PUT",
            Method::Mkcol => "MKCOL",
            Method::Move => "MOVE",
            Method::Propfind => "PROPFIND",
        }
    }
}

impl FromStr for Method {
    type Err = ();
    fn from_str(s: &str) -> Result<Method, ()> {
        match s {
            "GET" => Ok(Method::Get),
            "PUT" => Ok(Method::Put),
            "MKCOL" => Ok(Method::Mkcol),
            "MOVE" => Ok(Method::Move),
            "PROPFIND" => Ok(Method::Propfind),
            _ => Err(()),
        }
    }
}
