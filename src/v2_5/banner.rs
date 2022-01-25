// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_utils;

use super::banner_ad_type::BannerAdType;
use super::format::Format;

// 3.2.6 Object: Banner
//
// This object represents the most general type of impression. Although the
// term “banner” may have very specific meaning in other contexts, here it can
// be many things including a simple static image, an expandable ad unit, or
// even in-banner video (refer to the Video object in Section 3.2.7 for the more
// generalized and full featured video ad units). An array of Banner objects
// can also appear within the Video to describe optional companion ads defined
// in the VAST specification. The presence of a Banner as a subordinate of the
// Imp object indicates that this impression is offered as a banner type
// impression. At the publisher’s discretion, that same impression may also be
// offered as video, audio, and/or native by also including as Imp subordinates
// objects of those types. However, any given bid for the impression must
// conform to one of the offered types.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Banner {
    // Array of format objects (Section 3.2.10) representing the
    // banner sizes permitted. If none are specified, then use of the
    // h and w attributes is highly recommended.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub format: Vec<Format>,

    // Exact width in device independent pixels (DIPS);
    // recommended if no format objects are specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<u32>,

    // Exact height in device independent pixels (DIPS);
    // recommended if no format objects are specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<u32>,

    // Blocked banner ad types. Refer to List 5.2.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub btype: Vec<BannerAdType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}
