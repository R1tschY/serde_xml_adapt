use serde::de::{EnumAccess, Error, MapAccess, SeqAccess, Unexpected, Visitor};
use serde::ser::{SerializeMap, SerializeStruct};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Formatter;

pub struct Element {
    tag: String,
    attributes: BTreeMap<String, String>,
    children: Vec<Element>,
    inner_prefix: String,
    outer_suffix: String,
}

impl Serialize for Element {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        for attr in &self.attributes {
            map.serialize_entry(&format!("@{}", attr.0), attr.1);
        }

        for child in &self.children {
            map.serialize_entry(&child.tag, &child);
        }

        map.end()
    }
}

impl<'de> Deserialize<'de> for Element {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ElementVisitor;

        impl<'de> Visitor<'de> for ElementVisitor {
            type Value = Element;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid XML element value")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Element {
                    tag: "".to_string(),
                    attributes: Default::default(),
                    children: vec![],
                    inner_prefix: if v {
                        "yes".to_string()
                    } else {
                        "no".to_string()
                    },
                    outer_suffix: "".to_string(),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Element {
                    tag: "".to_string(),
                    attributes: Default::default(),
                    children: vec![],
                    inner_prefix: format!("{}", v),
                    outer_suffix: "".to_string(),
                })
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Element {
                    tag: "".to_string(),
                    attributes: Default::default(),
                    children: vec![],
                    inner_prefix: format!("{}", v),
                    outer_suffix: "".to_string(),
                })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Element {
                    tag: "".to_string(),
                    attributes: Default::default(),
                    children: vec![],
                    inner_prefix: format!("{}", v),
                    outer_suffix: "".to_string(),
                })
            }

            fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Element {
                    tag: "".to_string(),
                    attributes: Default::default(),
                    children: vec![],
                    inner_prefix: v.to_string(),
                    outer_suffix: "".to_string(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Element {
                    tag: "".to_string(),
                    attributes: Default::default(),
                    children: vec![],
                    inner_prefix: v.to_string(),
                    outer_suffix: "".to_string(),
                })
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Element {
                    tag: "".to_string(),
                    attributes: Default::default(),
                    children: vec![],
                    inner_prefix: v,
                    outer_suffix: "".to_string(),
                })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Element {
                    tag: "".to_string(),
                    attributes: Default::default(),
                    children: vec![],
                    inner_prefix: String::new(),
                    outer_suffix: "".to_string(),
                })
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                todo!()
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Element {
                    tag: "".to_string(),
                    attributes: Default::default(),
                    children: vec![],
                    inner_prefix: String::new(),
                    outer_suffix: "".to_string(),
                })
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                todo!()
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                todo!()
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                todo!()
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                todo!()
            }
        }
        deserializer.deserialize_any(ElementVisitor)
    }
}
