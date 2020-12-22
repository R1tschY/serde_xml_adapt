use quick_xml::Error as XmlError;
use std::fmt;
use std::result::Result as StdResult;
use std::str::Utf8Error;

pub type Result<T> = StdResult<T, Error>;

/// (De)serialization error
#[derive(Debug)]
pub enum Error {
    /// Serde custom error
    Custom(String),
    /// Cannot parse to integer
    Int(std::num::ParseIntError),
    /// Cannot parse to float
    Float(std::num::ParseFloatError),
    /// Xml parsing error
    Xml(XmlError),
    /// Unexpected end of attributes
    EndOfAttributes,
    /// Unexpected end of file
    Eof,
    /// Invalid value for a boolean
    InvalidBoolean(String),
    /// Invalid unit value
    InvalidUnit(String),
    /// Invalid event for Enum
    InvalidEnum(quick_xml::events::Event<'static>),
    /// Expecting Text event
    Text,
    /// Expecting Start event
    Start,
    /// Expecting End event
    End,
    /// Unsupported operation
    Unsupported(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> StdResult<(), fmt::Error> {
        match self {
            Error::Custom(s) => write!(f, "{}", s),
            Error::Xml(e) => write!(f, "{}", e),
            Error::Int(e) => write!(f, "{}", e),
            Error::Float(e) => write!(f, "{}", e),
            Error::EndOfAttributes => write!(f, "Unexpected end of attributes"),
            Error::Eof => write!(f, "Unexpected `Event::Eof`"),
            Error::InvalidBoolean(v) => write!(f, "Invalid boolean value '{}'", v),
            Error::InvalidUnit(v) => write!(f, "Invalid unit value '{}', expected empty string", v),
            Error::InvalidEnum(e) => write!(
                f,
                "Invalid event for Enum, expecting Text or Start, got: {:?}",
                e
            ),
            Error::Text => write!(f, "Expecting Text event"),
            Error::Start => write!(f, "Expecting Start event"),
            Error::End => write!(f, "Expecting End event"),
            Error::Unsupported(s) => write!(f, "Unsupported operation {}", s),
        }
    }
}

impl ::std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Int(e) => Some(e),
            Error::Float(e) => Some(e),
            Error::Xml(e) => Some(e),
            _ => None,
        }
    }
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl From<XmlError> for Error {
    fn from(e: XmlError) -> Self {
        Error::Xml(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::Int(e)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(e: std::num::ParseFloatError) -> Self {
        Error::Float(e)
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Error::Xml(quick_xml::Error::Utf8(e))
    }
}
