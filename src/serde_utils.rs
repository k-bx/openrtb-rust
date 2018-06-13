// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde;
use serde_json;

pub type Ext = serde_json::Value;

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
    match serde::Deserialize::deserialize(deserializer) {
        Ok(0) => Ok(false),
        Ok(1) => Ok(true),
        Ok(_) => Err(serde::de::Error::custom("The number is neither 1 nor 0")),
        Err(e) => Err(e),
    }
}

pub fn default_false() -> bool {
    false
}

pub fn is_false(x: &bool) -> bool {
    !*x
}
