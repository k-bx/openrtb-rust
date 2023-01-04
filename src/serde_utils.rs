// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde;
use serde_json;
use serde::de::Visitor;

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