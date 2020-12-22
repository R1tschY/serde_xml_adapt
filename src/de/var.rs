use std::io::BufRead;

use quick_xml::events::Event;
use serde::de::{self, Deserializer as SerdeDeserializer};

use crate::de::{escape::EscapedDeserializer, Deserializer};
use crate::Error;

/// An enum access
pub struct EnumAccess<'a, R: BufRead> {
    de: &'a mut Deserializer<R>,
}

impl<'a, R: BufRead> EnumAccess<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>) -> Self {
        EnumAccess { de }
    }
}

impl<'de, 'a, R: 'a + BufRead> de::EnumAccess<'de> for EnumAccess<'a, R> {
    type Error = Error;
    type Variant = VariantAccess<'a, R>;

    fn variant_seed<V: de::DeserializeSeed<'de>>(
        self,
        seed: V,
    ) -> Result<(V::Value, VariantAccess<'a, R>), Error> {
        let de = match self.de.peek()? {
            Some(Event::Text(t)) => EscapedDeserializer::new(t.to_vec(), true),
            Some(Event::Start(e)) => EscapedDeserializer::new(e.name().to_vec(), false),
            Some(e) => return Err(Error::InvalidEnum(e.to_owned())),
            None => return Err(Error::Eof),
        };
        let name = seed.deserialize(de)?;
        Ok((name, VariantAccess { de: self.de }))
    }
}

pub struct VariantAccess<'a, R: BufRead> {
    de: &'a mut Deserializer<R>,
}

impl<'de, 'a, R: BufRead> de::VariantAccess<'de> for VariantAccess<'a, R> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Error> {
        match self.de.next(&mut Vec::new())? {
            Event::Start(e) => self.de.read_to_end(e.name()),
            Event::Text(_) => Ok(()),
            _ => unreachable!(),
        }
    }

    fn newtype_variant_seed<T: de::DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, Error> {
        seed.deserialize(&mut *self.de)
    }

    fn tuple_variant<V: de::Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value, Error> {
        self.de.deserialize_tuple(len, visitor)
    }

    fn struct_variant<V: de::Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        self.de.deserialize_struct("", fields, visitor)
    }
}
