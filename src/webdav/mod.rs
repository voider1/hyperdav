use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use std::ascii::AsciiExt;

pub mod header;
pub mod method;

pub use self::method::Method;

pub enum Depth {
    Zero,
    One,
    Infinity,
}

impl Depth {
    fn name(&self) -> &str {
        match *self {
            Depth::Zero => "0",
            Depth::One => "1",
            Depth::Infinity => "Infinity",
        }
    }
}

impl fmt::Display for Depth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Depth {
    type Err = ();
    fn from_str(s: &str) -> Result<Depth, ()> {
        match s.to_ascii_uppercase().as_ref() {
            "0" => Ok(Depth::Zero),
            "1" => Ok(Depth::One),
            "INFINITY" => Ok(Depth::Infinity),
            _ => Err(())
        }
    }
}
