// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::category::Category;
use super::publisher::Publisher;
use serde_utils;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct App {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    #[serde(rename = "storeurl", skip_serializing_if = "Option::is_none")]
    pub store_url: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cat: Vec<Category>,

    #[serde(rename = "sectioncat", default, skip_serializing_if = "Vec::is_empty")]
    pub section_cat: Vec<Category>,

    #[serde(rename = "pagecat", default, skip_serializing_if = "Vec::is_empty")]
    pub page_cat: Vec<Category>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,

    // TODO: add properly
    // #[serde(rename = "privacypolicy", default, skip_serializing_if = "Vec::is_empty")]
    // privacy_policy: Option<String>,

    // TODO: add properly
    // paid:
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Publisher>,

    // TODO: add properly
    // #[skip_serializing_if = "Option::is_none"]
    // content: Option<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialization_skip_fields() {
        let serialized = r#"{
            "id": "1234"
        }"#;

        let res = serde_json::from_str(serialized);

        let _: App = match res {
            Ok(x) => x,
            Err(e) => panic!("{:?}", e),
        };
    }
}
