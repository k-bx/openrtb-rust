// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::data::Data;
use serde_utils;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    // A DSP's UID for this user (see user matching for format).
    #[serde(rename = "buyeruid", skip_serializing_if = "Option::is_none")]
    pub buyer_uid: Option<String>,
    // This object used by publishers to pass additional attributes about the user or content.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<Data>,
    // The User Ext Object, which is used to indicate requests that contain certain user identifiers and are subject to GDPR regulations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
    // The Index static identifier for this user (contains only alphanumeric or the following characters: @ - . _ ).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
