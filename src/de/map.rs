//! Serde `Deserializer` module

use std::io::BufRead;

use quick_xml::events::{BytesStart, Event};
use serde::de::{self, DeserializeSeed, IntoDeserializer};

use crate::error::Reason;
use crate::{
    de::{escape::EscapedDeserializer, Deserializer, INNER_VALUE},
    Error,
};
use std::vec;

enum MapValue {
    Empty,
    Attribute { value: Vec<u8> },
    Nested,
    InnerValue,
}

/// A deserializer for `Attributes`
pub(crate) struct MapAccess<'a, R: BufRead> {
    de: &'a mut Deserializer<R>,
    attributes: vec::IntoIter<(Vec<u8>, Vec<u8>)>,
    value: MapValue,
}

impl<'a, R: BufRead> MapAccess<'a, R> {
    fn create_attr_key(key: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(key.len() + 1);
        result.push(b'@');
        result.extend_from_slice(key);
        result
    }

    /// Create a new MapAccess
    pub fn new(de: &'a mut Deserializer<R>, start: &BytesStart<'static>) -> Result<Self, Error> {
        // TODO: optimize copies!
        let attributes = start
            .attributes()
            .map(|a| {
                let a = a?;
                Ok((Self::create_attr_key(a.key), a.value.into_owned()))
            })
            .collect::<Result<Vec<(Vec<u8>, Vec<u8>)>, Error>>()?
            .into_iter();
        Ok(MapAccess {
            de,
            attributes,
            value: MapValue::Empty,
        })
    }
}

impl<'a, 'de, R: BufRead> de::MapAccess<'de> for MapAccess<'a, R> {
    type Error = Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        if let Some((key, value)) = self.attributes.next() {
            // try getting map from attributes (key= "value")
            self.value = MapValue::Attribute { value };
            seed.deserialize(EscapedDeserializer::new(key, false))
                .map(Some)
        } else {
            let has_value_field = self.de.has_value_field;

            // try getting from events (<key>value</key>)
            match self.de.peek()? {
                Some(Event::Text(_)) => {
                    self.value = MapValue::InnerValue;
                    seed.deserialize(INNER_VALUE.into_deserializer()).map(Some)
                }
                // Used to deserialize collections of enums, like:
                // <root>
                //   <A/>
                //   <B/>
                //   <C/>
                // </root>
                //
                // into
                //
                // enum Enum { A, B, С }
                // struct Root {
                //     #[serde(rename = "$value")]
                //     items: Vec<Enum>,
                // }
                // TODO: This should be handled by #[serde(flatten)]
                // See https://github.com/serde-rs/serde/issues/1905
                Some(Event::Start(_)) if has_value_field => {
                    self.value = MapValue::InnerValue;
                    seed.deserialize(INNER_VALUE.into_deserializer()).map(Some)
                }
                Some(Event::Start(e)) => {
                    let name = e.local_name().to_owned();
                    self.value = MapValue::Nested;
                    seed.deserialize(EscapedDeserializer::new(name, false))
                        .map(Some)
                }
                _ => Ok(None),
            }
        }
    }

    fn next_value_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<K::Value, Self::Error> {
        match std::mem::replace(&mut self.value, MapValue::Empty) {
            MapValue::Attribute { value } => {
                seed.deserialize(EscapedDeserializer::new(value, true))
            }
            MapValue::Nested | MapValue::InnerValue => seed.deserialize(&mut *self.de),
            MapValue::Empty => Err(self.de.error(Reason::EndOfAttributes)),
        }
    }
}
