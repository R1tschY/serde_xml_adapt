//! Serde `Deserializer` module

use std::io::BufRead;

use quick_xml::events::{BytesStart, Event};
use serde::de::{self, DeserializeSeed, IntoDeserializer};

use crate::{
    de::error::DeError,
    de::{escape::EscapedDeserializer, Deserializer, INNER_VALUE},
};

enum MapValue {
    Empty,
    Attribute { value: Vec<u8> },
    Nested,
    InnerValue,
}

/// A deserializer for `Attributes`
pub(crate) struct MapAccess<'a, R: BufRead> {
    /// Tag -- owner of attributes
    _start: BytesStart<'static>,
    de: &'a mut Deserializer<R>,
    attributes: Vec<(Vec<u8>, Vec<u8>)>,
    attributes_pos: usize,
    value: MapValue,
}

impl<'a, R: BufRead> MapAccess<'a, R> {
    /// Create a new MapAccess
    pub fn new(de: &'a mut Deserializer<R>, start: BytesStart<'static>) -> Result<Self, DeError> {
        // TODO: optimize copies!
        let attributes = start
            .attributes()
            .map(|a| {
                let a = a?;
                Ok((a.key.to_owned(), a.value.into_owned()))
            })
            .collect::<Result<Vec<(Vec<u8>, Vec<u8>)>, DeError>>()?;
        Ok(MapAccess {
            de,
            _start: start,
            attributes,
            attributes_pos: 0,
            value: MapValue::Empty,
        })
    }

    fn next_attr(&mut self) -> Option<(Vec<u8>, Vec<u8>)> {
        // TODO: optimize copies!
        if self.attributes_pos < self.attributes.len() {
            let result = Some(self.attributes[self.attributes_pos].clone());
            self.attributes_pos += 1;
            result
        } else {
            None
        }
    }
}

impl<'a, 'de, R: BufRead> de::MapAccess<'de> for MapAccess<'a, R> {
    type Error = DeError;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        let attr_key_val = self.next_attr();
        let has_value_field = self.de.has_value_field;
        if let Some((key, value)) = attr_key_val {
            // try getting map from attributes (key= "value")
            self.value = MapValue::Attribute { value };
            seed.deserialize(EscapedDeserializer::new(key, false))
                .map(Some)
        } else {
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
            MapValue::Empty => Err(DeError::EndOfAttributes),
        }
    }
}
