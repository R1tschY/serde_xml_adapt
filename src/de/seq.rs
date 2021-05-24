use std::io::BufRead;

use quick_xml::events::{BytesStart, Event};
use serde::de;

use crate::de::Deserializer;
use crate::Error;

#[derive(Debug)]
enum Names {
    Unknown,
    Peek(Vec<u8>),
}

impl Names {
    fn is_valid(&self, start: &BytesStart) -> bool {
        match self {
            Names::Unknown => true,
            Names::Peek(n) => **n == *start.name(),
        }
    }
}

/// A SeqAccess
pub struct SeqAccess<'a, R: BufRead> {
    de: &'a mut Deserializer<R>,
    max_size: Option<usize>,
    names: Names,
}

impl<'a, R: BufRead> SeqAccess<'a, R> {
    /// Get a new SeqAccess
    pub fn new(de: &'a mut Deserializer<R>, max_size: Option<usize>) -> Result<Self, Error> {
        let names = if de.has_value_field {
            Names::Unknown
        } else if let Some(Event::Start(e)) = de.peek()? {
            Names::Peek(e.name().to_vec())
        } else {
            Names::Unknown
        };
        Ok(SeqAccess {
            de,
            max_size,
            names,
        })
    }
}

impl<'de, 'a, R: 'a + BufRead> de::SeqAccess<'de> for SeqAccess<'a, R> {
    type Error = Error;

    fn size_hint(&self) -> Option<usize> {
        self.max_size
    }

    fn next_element_seed<T: de::DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Error> {
        if let Some(s) = self.max_size.as_mut() {
            if *s == 0 {
                return Ok(None);
            }
            *s -= 1;
        }
        match self.de.peek()? {
            None | Some(Event::Eof) | Some(Event::End(_)) => Ok(None),
            Some(Event::Start(e)) if !self.names.is_valid(e) => Ok(None),
            _ => seed.deserialize(&mut *self.de).map(Some),
        }
    }
}
