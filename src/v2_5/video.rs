use crate::serde_utils;

// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// This object represents an in-stream video impression.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Video {
    /// Content MIME types supported (e.g., “video/x-ms-wmv”,“video/mp4”).
    pub mimes: Vec<String>,
    /// Minimum video ad duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minduration: Option<u32>,
    /// Maximum video ad duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxduration: Option<u32>,
    /// Array of supported video protocols.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocols: Vec<u32>,
    /// Width of the video player in device independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<u32>,
    /// Height of the video player in device independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<u32>,
    /// Indicates if the impression must be linear, nonlinear, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linearity: Option<u32>,
    /// Blocked creative attributes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub battr: Vec<u32>,
    /// Minimum bit rate in Kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minbitrate: Option<u32>,
    /// Maximum bit rate in Kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxbitrate: Option<u32>,
    /// Indicates if letter-boxing of 4:3 content into
    /// a 16:9 window is allowed, where 0 = no, 1 = yes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boxingallowed: Option<u32>,
    /// Playback methods that may be in use.
    /// If none are specified, any method may be used.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub playbackmethod: Vec<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

#[test]
fn serialization_skip_fields() {
    let v = Video {
        mimes: vec!["video/mp4".to_string()],
        minduration: None,
        maxduration: None,
        protocols: vec![],
        w: None,
        h: None,
        linearity: None,
        battr: vec![],
        minbitrate: None,
        maxbitrate: None,
        boxingallowed: None,
        playbackmethod: vec![],
        ext: None,
    };

    let expected = r#"{"mimes":["video/mp4"]}"#;
    let serialized = serde_json::to_string(&v).unwrap();

    assert_eq!(expected, serialized)
}
