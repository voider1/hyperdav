use std::fmt::{self, Display};
use std::str::FromStr;
use std::ascii::AsciiExt;

header! { (Destination, "Destination") => [String] }

header! { (Depth, "Depth") => [String] }
// TODO header! { (Depth, "Depth") => [DepthValue] }

pub enum DepthValue {
    Zero,
    One,
    Infinity,
    Ext(String),
}

impl DepthValue {
    fn name(&self) -> &str {
        match *self {
            DepthValue::Zero => "0",
            DepthValue::One => "1",
            DepthValue::Infinity => "Infinity",
            DepthValue::Ext(ref s) => &s
        }
    }
}

impl Display for DepthValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for DepthValue {
    type Err = ();
    fn from_str(s: &str) -> Result<DepthValue, ()> {
        Ok(match s.to_ascii_uppercase().as_ref() {
            "0" => DepthValue::Zero,
            "1" => DepthValue::One,
            "INFINITY" => DepthValue::Infinity,
            s => DepthValue::Ext(s.to_owned()),
        })
    }
}
