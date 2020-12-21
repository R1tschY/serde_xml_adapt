pub mod de;
pub mod ser;

pub use crate::de::{error::DeError, from_reader, from_str, Deserializer};
pub use crate::ser::{error::SerError, to_string, to_writer, Serializer};
pub use quick_xml::{self, Writer};
