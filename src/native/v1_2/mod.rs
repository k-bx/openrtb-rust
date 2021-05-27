// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_utils;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<u64>,
    #[serde(rename = "contextsubtype", skip_serializing_if = "Option::is_none")]
    pub context_subtype: Option<u64>,
    #[serde(rename = "plcmttype", skip_serializing_if = "Option::is_none")]
    pub placement_type: Option<u64>,
    #[serde(rename = "plcmtcnt", skip_serializing_if = "Option::is_none")]
    pub placement_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<Asset>,
    #[serde(rename = "aurlsupport", skip_serializing_if = "Option::is_none")]
    pub aurl_support: Option<u64>,
    #[serde(rename = "durlsupport", skip_serializing_if = "Option::is_none")]
    pub durl_support: Option<u64>,
    #[serde(
        default,
        rename = "eventtrackers",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub event_trackers: Vec<EventTracker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Asset {
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Title {
    pub len: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Image {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wmin: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmin: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mimes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Video {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Data {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EventTracker {
    pub event: u64,
    pub method: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Response {
    pub ver: Option<String>,
    pub assets: Vec<AssetResponse>,
    #[serde(rename = "assetsurl", skip_serializing_if = "Option::is_none")]
    pub assets_url: Option<String>,
    pub dcourl: Option<String>,
    pub link: DestinationLink,
    #[serde(default, rename = "imptrackers", skip_serializing_if = "Vec::is_empty")]
    pub imp_trackers: Vec<String>,
    #[serde(rename = "jstracker", skip_serializing_if = "Option::is_none")]
    pub js_tracker: Option<String>,
    #[serde(
        default,
        rename = "eventtrackers",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub event_trackers: Vec<EventTrackerResponse>,
    pub privacy: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DestinationLink {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AssetResponse {
    pub id: Option<u64>,
    pub required: Option<u64>,
    pub title: Option<TitleResponse>,
    pub img: Option<ImageResponse>,
    pub video: Option<VideoResponse>,
    pub data: Option<DataResponse>,
    pub link: Option<LinkResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TitleResponse {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ImageResponse {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<u64>,
    pub url: String,
    pub w: Option<u64>,
    pub h: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VideoResponse {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DataResponse {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LinkResponse {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EventTrackerResponse {
    pub event: u64,
    pub method: u64,
    pub url: Option<String>,
    #[serde(rename = "customdata", skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<serde_utils::Ext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}
