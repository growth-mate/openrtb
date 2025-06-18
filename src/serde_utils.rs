// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::de::Visitor;
use serde::Deserialize;

pub type Ext = serde_json::map::Map<String, serde_json::value::Value>;

struct BoolVisitor;

impl<'de> Visitor<'de> for BoolVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a bool value or integer 0 or 1")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value)
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(E::custom(format!("a bool value must be 0 or 1: {}", value))),
        }
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(E::custom(format!("a bool value must be 0 or 1: {}", value))),
        }
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(E::custom(format!("a bool value must be 0 or 1: {}", value))),
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(E::custom(format!("a bool value must be 0 or 1: {}", value))),
        }
    }
}

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a bool value or integer 0 or 1")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.to_string())
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.to_string())
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.to_string())
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.to_string())
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.to_string())
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.to_string())
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.to_string())
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.to_string())
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.to_string())
    }

    fn visit_char<E>(self, value: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.to_string())
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.to_string())
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value)
    }
}

pub fn bool_to_u8<S>(x: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u8(*x as u8)
}

pub fn u8_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_any(BoolVisitor)
}

pub fn default_false() -> bool {
    false
}

pub fn is_false(x: &bool) -> bool {
    !*x
}

pub fn mbool_to_u8<S>(mx: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match mx {
        None => serializer.serialize_none(),
        Some(x) => bool_to_u8(x, serializer),
    }
}

pub fn u8_to_mbool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match deserializer.deserialize_any(BoolVisitor) {
        Ok(value) => Ok(Some(value)),
        Err(_) => Ok(None),
    }
}

pub fn is_none_or_empty<T>(vec: &Option<Vec<T>>) -> bool {
    if let Some(vec) = vec {
        vec.is_empty()
    } else {
        true
    }
}

pub fn anything_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_any(StringVisitor)
}

use crate::v2_5::Category;

struct StringOrVecVisitor;

impl<'de> Visitor<'de> for StringOrVecVisitor {
    type Value = Vec<Category>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string or array of strings")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // Parse category using the same logic as Category's deserializer
        let category = match crate::v2_5::category::TO_CATEGORY.get(value).cloned() {
            Some(c) => c,
            None => Category::Unknown(value.to_string()),
        };
        Ok(vec![category])
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&value)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(value) = seq.next_element::<String>()? {
            // Parse category using the same logic as Category's deserializer
            let category = match crate::v2_5::category::TO_CATEGORY.get(value.as_str()).cloned() {
                Some(c) => c,
                None => Category::Unknown(value),
            };
            vec.push(category);
        }
        Ok(vec)
    }
}

/// Deserialize a field that can be either a single string or an array of strings into Vec<Category>
pub fn string_or_vec_category<'de, D>(deserializer: D) -> Result<Option<Vec<Category>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::deserialize(deserializer)?.map(|v| {
        match v {
            serde_json::Value::String(s) => {
                let category = match crate::v2_5::category::TO_CATEGORY.get(s.as_str()).cloned() {
                    Some(c) => c,
                    None => Category::Unknown(s),
                };
                Ok(vec![category])
            },
            serde_json::Value::Array(arr) => {
                let mut categories = Vec::new();
                for item in arr {
                    if let serde_json::Value::String(s) = item {
                        let category = match crate::v2_5::category::TO_CATEGORY.get(s.as_str()).cloned() {
                            Some(c) => c,
                            None => Category::Unknown(s),
                        };
                        categories.push(category);
                    }
                }
                Ok(categories)
            },
            _ => Err(serde::de::Error::custom("expected string or array of strings"))
        }
    }).transpose()
}

/// Deserialize a field that can be either a boolean or an integer into i32
pub fn bool_or_int_to_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::deserialize(deserializer)?.map(|v| {
        match v {
            serde_json::Value::Bool(b) => Ok(if b { 1 } else { 0 }),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(i as i32)
                } else {
                    Err(serde::de::Error::custom("expected integer"))
                }
            },
            _ => Err(serde::de::Error::custom("expected boolean or integer"))
        }
    }).transpose()
}

/// Deserialize a field that can be either a string or an integer into String
pub fn int_or_string_to_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::deserialize(deserializer)?.map(|v| {
        match v {
            serde_json::Value::String(s) => Ok(s),
            serde_json::Value::Number(n) => Ok(n.to_string()),
            _ => Err(serde::de::Error::custom("expected string or integer"))
        }
    }).transpose()
}
