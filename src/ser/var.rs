use std::io::Write;

use quick_xml::{
    events::{BytesStart, Event},
    Writer,
};
use serde::ser::{self, Serialize};

use crate::ser::attributes::AttributeSerializer;
use crate::ser::error::DeError;
use crate::ser::Serializer;

/// An implementation of `SerializeStruct` for serializing to XML.
pub struct Struct<'r, 'w, 'a, W>
where
    W: Write,
{
    parent: &'w mut Serializer<'r, 'a, W>,
    /// Buffer for holding fields, serialized as attributes. Doesn't allocate
    /// if there are no fields represented as attributes
    attrs: BytesStart<'w>,
    /// Buffer for holding fields, serialized as elements
    children: Vec<u8>,
    /// Buffer for serializing one field. Cleared after serialize each field
    buffer: Vec<u8>,
}

impl<'r, 'a, 'w, W> Struct<'r, 'w, 'a, W>
where
    W: 'w + Write,
{
    /// Create a new `Struct`
    pub fn new(parent: &'w mut Serializer<'r, 'a, W>, name: &'w str) -> Self {
        let name = name.as_bytes();
        Struct {
            parent,
            attrs: BytesStart::borrowed_name(name),
            children: Vec::new(),
            buffer: Vec::new(),
        }
    }

    fn serialize_tag<T: ?Sized + Serialize>(
        &mut self,
        key: &str,
        value: &T,
    ) -> Result<(), DeError> {
        // TODO: Inherit indentation state from self.parent.writer

        if key.starts_with("@") {
            if key.len() == 1 {
                return Err(DeError::Custom("name for attribute is missing".to_string()));
            }

            let mut serializer = AttributeSerializer::new();
            let attribute_value = value.serialize(&mut serializer)?;
            self.attrs
                .push_attribute((&key[1..], &attribute_value as &str));
            self.buffer.clear();
        } else {
            let root = if key.starts_with("$") {
                None
            } else {
                Some(key)
            };
            let mut writer = Writer::new(&mut self.buffer);
            let mut serializer = Serializer::new_with_root(&mut writer, root);
            value.serialize(&mut serializer)?;

            self.children.append(&mut self.buffer);
        }
        Ok(())
    }

    fn close(&mut self) -> Result<(), DeError> {
        let writer = &mut self.parent.writer;
        if self.children.is_empty() {
            writer.write_event(Event::Empty(self.attrs.to_borrowed()))?;
        } else {
            writer.write_event(Event::Start(self.attrs.to_borrowed()))?;
            writer.write(&self.children)?;
            writer.write_event(Event::End(self.attrs.to_end()))?;
        }
        Ok(())
    }
}

impl<'r, 'w, 'a, W> ser::SerializeStruct for Struct<'r, 'w, 'a, W>
where
    W: 'w + Write,
{
    type Ok = ();
    type Error = DeError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), DeError> {
        self.serialize_tag(key, value)
    }

    fn end(mut self) -> Result<Self::Ok, DeError> {
        self.close()
    }
}

impl<'r, 'w, 'a, W> ser::SerializeStructVariant for Struct<'r, 'w, 'a, W>
where
    W: 'w + Write,
{
    type Ok = ();
    type Error = DeError;

    #[inline]
    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.serialize_tag(key, value)
    }

    #[inline]
    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.close()?;

        if let Some(root) = self.parent.root_tag {
            self.parent.write_tag_end(root)
        } else {
            Ok(())
        }
    }
}

impl<'r, 'a, 'w, W> ser::SerializeMap for Struct<'r, 'a, 'w, W>
where
    W: 'w + Write,
{
    type Ok = ();
    type Error = DeError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, _: &T) -> Result<(), DeError> {
        Err(DeError::Unsupported(
            "impossible to serialize the key on its own, please use serialize_entry()",
        ))
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), DeError> {
        value.serialize(&mut *self.parent)
    }

    fn serialize_entry<K: ?Sized + Serialize, V: ?Sized + Serialize>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), DeError> {
        let tag = key.serialize(&mut AttributeSerializer::new())?;
        self.serialize_tag(&tag, value)
    }

    fn end(mut self) -> Result<Self::Ok, DeError> {
        self.close()
    }
}

/// An implementation of `SerializeSeq`, `SerializeTuple`, `SerializeTupleStruct` and
/// `SerializeTupleVariant` for serializing to XML.
pub struct Seq<'r, 'w, 'a, W>
where
    W: Write,
{
    parent: &'w mut Serializer<'r, 'a, W>,
}

impl<'r, 'w, 'a, W> Seq<'r, 'w, 'a, W>
where
    W: Write,
{
    /// Create a new `Tuple`
    pub fn new(parent: &'w mut Serializer<'r, 'a, W>) -> Self {
        Seq { parent }
    }

    fn serialize_item<T: ?Sized>(&mut self, value: &T) -> Result<(), DeError>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.parent)
    }
}

impl<'r, 'w, 'a, W> ser::SerializeSeq for Seq<'r, 'w, 'a, W>
where
    W: 'w + Write,
{
    type Ok = ();
    type Error = DeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_item(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'r, 'a, 'w, W> ser::SerializeTuple for Seq<'r, 'a, 'w, W>
where
    W: 'w + Write,
{
    type Ok = ();
    type Error = DeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_item(value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'r, 'a, 'w, W> ser::SerializeTupleStruct for Seq<'r, 'a, 'w, W>
where
    W: 'w + Write,
{
    type Ok = ();
    type Error = DeError;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_item(value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'r, 'a, 'w, W> ser::SerializeTupleVariant for Seq<'r, 'a, 'w, W>
where
    W: 'w + Write,
{
    type Ok = ();
    type Error = DeError;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_item(value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        if let Some(root) = self.parent.root_tag {
            self.parent.write_tag_end(root)
        } else {
            Ok(())
        }
    }
}
