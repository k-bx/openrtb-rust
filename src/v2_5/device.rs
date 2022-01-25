// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::geo::Geo;
use serde_utils;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Device {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ua: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Geo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lmt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,
    #[serde(rename = "devicetype", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppi: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pxratio: Option<f64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "serde_utils::mbool_to_u8",
        deserialize_with = "serde_utils::u8_to_mbool"
    )]
    pub js: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "serde_utils::mbool_to_u8",
        deserialize_with = "serde_utils::u8_to_mbool"
    )]
    pub geofetch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flashver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mccmnc: Option<String>,
    #[serde(rename = "connectiontype", skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub didsha1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub didmd5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpidsha1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpidmd5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macsha1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macmd5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}
