// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_utils;

use super::audio::Audio;
use super::banner::Banner;
use super::metric::Metric;
use super::native::Native;
use super::pmp::PMP;
use super::video::Video;

#[derive(Serialize, Deserialize, Debug)]
pub struct Imp {
    pub id: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metric: Vec<Metric>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<Banner>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<Native>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmp: Option<PMP>,

    #[serde(rename = "displaymanager", skip_serializing_if = "Option::is_none")]
    pub display_manager: Option<String>,

    #[serde(rename = "displaymanagerver", skip_serializing_if = "Option::is_none")]
    pub display_manager_ver: Option<String>,

    #[serde(
        rename = "instl",
        skip_serializing_if = "serde_utils::is_false",
        serialize_with = "serde_utils::bool_to_u8",
        deserialize_with = "serde_utils::u8_to_bool"
    )]
    pub interstitial: bool,

    #[serde(rename = "tagid", skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialization_skip_fields() {
        let i = Imp {
            id: "1234".to_string(),
            metric: vec![],
            banner: None,
            video: None,
            audio: None,
            native: None,
            pmp: None,
            display_manager: None,
            display_manager_ver: None,
            interstitial: false,
            tag_id: None,
            ext: None,
        };

        let expected = r#"{"id":"1234"}"#;
        let serialized = serde_json::to_string(&i).unwrap();

        assert_eq!(expected, serialized)
    }
}
