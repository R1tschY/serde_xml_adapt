//! A module to handle serde (de)serialization errors

use std::fmt;

/// (De)serialization error
#[derive(Debug)]
pub enum SerError {
    /// Serde custom error
    Custom(String),
    /// Cannot parse to integer
    Int(std::num::ParseIntError),
    /// Cannot parse to float
    Float(std::num::ParseFloatError),
    /// Xml parsing error
    Xml(quick_xml::Error),
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

impl fmt::Display for SerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            SerError::Custom(s) => write!(f, "{}", s),
            SerError::Xml(e) => write!(f, "{}", e),
            SerError::Int(e) => write!(f, "{}", e),
            SerError::Float(e) => write!(f, "{}", e),
            SerError::EndOfAttributes => write!(f, "Unexpected end of attributes"),
            SerError::Eof => write!(f, "Unexpected `Event::Eof`"),
            SerError::InvalidBoolean(v) => write!(f, "Invalid boolean value '{}'", v),
            SerError::InvalidUnit(v) => {
                write!(f, "Invalid unit value '{}', expected empty string", v)
            }
            SerError::InvalidEnum(e) => write!(
                f,
                "Invalid event for Enum, expecting Text or Start, got: {:?}",
                e
            ),
            SerError::Text => write!(f, "Expecting Text event"),
            SerError::Start => write!(f, "Expecting Start event"),
            SerError::End => write!(f, "Expecting End event"),
            SerError::Unsupported(s) => write!(f, "Unsupported operation {}", s),
        }
    }
}

impl ::std::error::Error for SerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SerError::Int(e) => Some(e),
            SerError::Float(e) => Some(e),
            SerError::Xml(e) => Some(e),
            _ => None,
        }
    }
}

impl serde::de::Error for SerError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SerError::Custom(msg.to_string())
    }
}

impl serde::ser::Error for SerError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SerError::Custom(msg.to_string())
    }
}

impl From<quick_xml::Error> for SerError {
    fn from(e: quick_xml::Error) -> Self {
        SerError::Xml(e)
    }
}

impl From<std::num::ParseIntError> for SerError {
    fn from(e: std::num::ParseIntError) -> Self {
        SerError::Int(e)
    }
}

impl From<std::num::ParseFloatError> for SerError {
    fn from(e: std::num::ParseFloatError) -> Self {
        SerError::Float(e)
    }
}
