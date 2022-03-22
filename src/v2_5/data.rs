// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::segment::Segment;
use serde_utils;

// This object used by publishers to pass additional attributes about the user or content.

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Data {
    // The data extension object that contains community extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
    // The unique domain of the business entity who is stating the additional information about the user or content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    // Contains additional information about the related objrct that is specified, such as the user, site, or app object.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<Segment>,
}
