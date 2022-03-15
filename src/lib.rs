pub mod de;
pub mod element;
mod error;
pub mod ser;

pub use crate::de::{from_reader, from_str, Deserializer};
pub use crate::error::{Error, Result};
pub use crate::ser::{to_string, to_writer, Serializer};
pub use quick_xml::{self, Writer};
use std::fmt;
use std::fmt::{Display, Formatter};

/// Extensible Markup Language (XML) Version
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum XmlVersion {
    /// Version 1.0
    ///
    /// https://www.w3.org/TR/2008/REC-xml-20081126/
    v1_0,

    /// Version 1.1
    ///
    /// https://www.w3.org/TR/2006/REC-xml11-20060816/
    v1_1,
}

impl XmlVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            XmlVersion::v1_0 => "1.0",
            XmlVersion::v1_1 => "1.1",
        }
    }

    pub fn as_bytes(&self) -> &'static [u8] {
        match self {
            XmlVersion::v1_0 => b"1.0",
            XmlVersion::v1_1 => b"1.1",
        }
    }
}

impl Display for XmlVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
