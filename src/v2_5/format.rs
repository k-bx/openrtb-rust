// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_utils;

// 3.2.10 Object: Format
//
// This object represents an allowed size (i.e., height and width combination)
// or Flex Ad parameters for a banner impression. These are typically used in
// an array where multiple sizes are permitted. It is recommended that either
// the w/h pair or the wratio/hratio/wmin set (i.e., for Flex Ads) be specified.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Format {
    // Width in device independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<u32>,

    // Height in device independent pixels (DIPS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<u32>,

    // Relative width when expressing size as a ratio.
    #[serde(rename = "wratio", skip_serializing_if = "Option::is_none")]
    pub w_ratio: Option<u32>,

    // Relative height when expressing size as a ratio
    #[serde(rename = "hratio", skip_serializing_if = "Option::is_none")]
    pub h_ratio: Option<u32>,

    // The minimum width in device independent pixels (DIPS) at
    // which the ad will be displayed the size is expressed as a ratio.
    #[serde(rename = "w_min", skip_serializing_if = "Option::is_none")]
    pub w_min: Option<u32>,

    // Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialization_skip_fields() {
        let f = Format {
            w: None,
            h: None,
            w_ratio: None,
            h_ratio: None,
            w_min: None,
            ext: None,
        };

        let expected = r#"{}"#;
        let serialized = serde_json::to_string(&f).unwrap();

        assert_eq!(expected, serialized)
    }

    #[test]
    fn deserialize_defaults() {
        let serialized = r#"{}"#;

        let res = serde_json::from_str(serialized);

        let f: Format = match res {
            Ok(x) => x,
            Err(e) => panic!("{:?}", e),
        };

        let expected = Format {
            w: None,
            h: None,
            w_ratio: None,
            h_ratio: None,
            w_min: None,
            ext: None,
        };

        assert_eq!(expected, f);
    }
}
