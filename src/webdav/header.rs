use std::fmt::{self, Display};
use std::str::FromStr;
use std::ascii::AsciiExt;

header! { (Destination, "Destination") => [String] }

header! { (Depth, "Depth") => [String] }
// TODO header! { (Depth, "Depth") => [DepthValue] }
