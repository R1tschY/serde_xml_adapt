mod ser;

pub use quick_xml::{self, Writer};
pub use crate::ser::{to_string, to_writer, Serializer};