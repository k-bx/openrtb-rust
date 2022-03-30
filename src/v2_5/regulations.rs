// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_utils;

// This object contains any legal, governmental, or industry regulations 
// that apply to the request. The coppa flag signals whether or not 
// the request falls under the United States Federal Trade Commission’s
// regulations for the United States Children’s Online Privacy Protection Act (“COPPA”).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Regulations {
    // Flag indicating if this request is subject to the COPPA
    // regulations established by the USA FTC, where 0 = no, 1 = yes.
    #[serde(
        skip_serializing_if = "serde_utils::is_false",
        serialize_with = "serde_utils::bool_to_u8",
        deserialize_with = "serde_utils::u8_to_bool"
    )]
    pub coppa: bool,

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
        let r = Regulations {
            coppa: false,
            ext: None,
        };

        let expected = r#"{}"#;
        let serialized = serde_json::to_string(&r).unwrap();

        assert_eq!(expected, serialized)
    }
}
