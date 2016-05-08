use std::io::Read;
use xml::reader::{EventReader, XmlEvent, Error as XmlError};
use std;
use std::error::Error;

#[derive(Default)]
pub struct PropfindResponse {
    pub href: Option<String>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum PropfindParseError {
    UnknownDocument,
    InvalidFieldValue,
    UnknownElement,
    UnknownField,
    ExpectedEndOfDocument,
    Xml(XmlError),
}

impl From<XmlError> for PropfindParseError {
    fn from(e: XmlError) -> Self {
        Error::Xml(e)
    }
}

impl std::fmt::Display for PropfindParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for PropfindParseError {
    fn description(&self) -> &str {
        use PropfindParseError::*;
        match *self {
            UnknownDocument => "not a propfind response",
            InvalidFieldValue => "field must only contain text",
            UnknownElement => "document must only contain responses",
            UnkownField => "unsupported field",
            ExpectedEndOfDocument => "expected end of document",
            Xml(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        use PropfindParseError::*;
        match *self {
            Xml(ref e) => Some(e as &Error),
            _ => None,
        }
    }
}

pub fn parse_propfind<R: Read>(read: R) -> Result<Vec<PropfindResponse>, PropfindParseError> {
    enum Field {
        Href,
    }

    enum State {
        Items,
        Item {
            item: PropfindResponse,
            field: Option<Field>,
        },
        Start,
        End,
    }

    let parser = EventReader::new(read);
    let items = Vec::new();
    let mut state = State::Start;

    for e in parser {
        let e = try!(e);
        state = match state {
            State::Start => {
                match e {
                    XmlEvent::StartDocument { .. } => State::Start,
                    XmlEvent::StartElement { ref name, .. } if name.local_name == "multistatus" => {
                        State::Items
                    }
                    _ => return Err(PropfindParseError::UnknownDocument),
                }
            }
            State::End => {
                match e {
                    XmlEvent::EndDocument => return Ok(items),
                    _ => return Err(PropfindParseError::ExpectedEndOfDocument),
                }
            }
            State::Items => {
                match e {
                    XmlEvent::EndElement { .. } => State::End,
                    XmlEvent::StartElement { ref name, .. } if &*name.local_name == "response" => {
                        State::Item {
                            item: PropfindResponse::default(),
                            field: None,
                        }
                    }
                    _ => return Err(PropfindParseError::UnknownDocument),
                }
            }
            State::Item { field: None, item } => {
                match e {
                    XmlEvent::StartElement { name, .. } => {
                        match &*name.local_name {
                            "href" => {
                                State::Item {
                                    field: Some(Field::Href),
                                    item: item,
                                }
                            }
                            _ => {
                                State::Item {
                                    field: None,
                                    item: item,
                                }
                            }
                        }
                    }
                    XmlEvent::EndElement { .. } => {
                        items.push(item);
                        State::Items
                    }
                    _ => {
                        State::Item {
                            field: None,
                            item: item,
                        }
                    }
                }
            }
            State::Item { field: Some(field), mut item } => {
                match e {
                    XmlEvent::Characters(s) => {
                        match field {
                            Field::Href => item.href = Some(s),
                        };
                        State::Item {
                            field: Some(field),
                            item: item,
                        }
                    }
                    XmlEvent::EndElement { .. } => {
                        State::Item {
                            field: None,
                            item: item,
                        }
                    }
                    _ => return Err(PropfindParseError::InvalidFieldValue),
                }
            }
        }
    }
}
