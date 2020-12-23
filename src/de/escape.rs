//! Serde `Deserializer` module

use std::borrow::Cow;
use std::str::from_utf8;

use quick_xml::escape::unescape;
use quick_xml::Error as XmlError;
use serde::de::{self, Visitor};
use serde::{self, forward_to_deserialize_any};

use crate::{error::Reason, error::ResultExt, Error, Result};

/// A deserializer for a xml escaped and encoded value
///
/// # Note
///
/// Escaping the value is actually not always necessary, for instance
/// when converting to float, we don't expect any escapable character
/// anyway
#[derive(Clone)]
pub(crate) struct EscapedDeserializer {
    /// Possible escaped value of text/CDATA or attribute value
    escaped_value: Vec<u8>,
    /// If `true`, value requires unescaping before using
    escaped: bool,
}

impl EscapedDeserializer {
    pub fn new(escaped_value: Vec<u8>, escaped: bool) -> Self {
        EscapedDeserializer {
            escaped_value,
            escaped,
        }
    }

    fn unescaped(&self) -> Result<Cow<[u8]>> {
        if self.escaped {
            unescape(&self.escaped_value)
                .map_err(|e| self.error(Reason::Xml(XmlError::EscapeError(e))))
        } else {
            Ok(Cow::Borrowed(&self.escaped_value))
        }
    }

    fn error(&self, reason: Reason) -> Error {
        Error::new(reason, 0)
    }

    fn from_utf8(v: &[u8]) -> Result<&str> {
        from_utf8(&v).map_err(|err| Error::new(Reason::Xml(XmlError::Utf8(err)), err.valid_up_to()))
    }
}

macro_rules! deserialize_num {
    ($method:ident, $ty:path, $visit:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            let value = Self::from_utf8(&self.escaped_value)?
                .parse::<$ty>()
                .at_offset(0)?;
            visitor.$visit(value)
        }
    };
}

impl<'de> serde::Deserializer<'de> for EscapedDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &*self.escaped_value {
            b"true" | b"1" => visitor.visit_bool(true),
            b"false" | b"0" => visitor.visit_bool(false),
            e => Err(self.error(Reason::InvalidBoolean(Self::from_utf8(e)?.into()))),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let unescaped = self.unescaped()?;
        let value = Self::from_utf8(&unescaped)?;
        visitor.visit_str(&value)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.unescaped()?;
        visitor.visit_bytes(&v)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.escaped_value.is_empty() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.escaped_value.is_empty() {
            visitor.visit_unit()
        } else {
            Err(self.error(Reason::InvalidUnit(
                "Expecting unit, got non empty attribute".into(),
            )))
        }
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V: de::Visitor<'de>>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_enum(self)
    }

    deserialize_num!(deserialize_i64, i64, visit_i64);
    deserialize_num!(deserialize_i32, i32, visit_i32);
    deserialize_num!(deserialize_i16, i16, visit_i16);
    deserialize_num!(deserialize_i8, i8, visit_i8);
    deserialize_num!(deserialize_u64, u64, visit_u64);
    deserialize_num!(deserialize_u32, u32, visit_u32);
    deserialize_num!(deserialize_u16, u16, visit_u16);
    deserialize_num!(deserialize_u8, u8, visit_u8);
    deserialize_num!(deserialize_f64, f64, visit_f64);
    deserialize_num!(deserialize_f32, f32, visit_f32);

    forward_to_deserialize_any! {
        unit_struct seq tuple tuple_struct map struct identifier ignored_any
    }
}

impl<'de> de::EnumAccess<'de> for EscapedDeserializer {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V: de::DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self)> {
        let name = seed.deserialize(self.clone())?;
        Ok((name, self))
    }
}

impl<'de> de::VariantAccess<'de> for EscapedDeserializer {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T: de::DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value> {
        seed.deserialize(self)
    }

    fn tuple_variant<V: de::Visitor<'de>>(self, _len: usize, _visitor: V) -> Result<V::Value> {
        unimplemented!()
    }

    fn struct_variant<V: de::Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value> {
        unimplemented!()
    }
}
