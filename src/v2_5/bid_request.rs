// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde;

use serde_utils;

use super::app::App;
use super::category::Category;
use super::device::Device;
use super::imp::Imp;
use super::regulations::Regulations;
use super::site::Site;
use super::source::Source;
use super::user::User;

#[derive(Debug, PartialEq)]
pub enum AuctionType {
    FirstPrice,
    SecondPricePlus,
    ExchangeSpecific(u32),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BidRequest {
    id: String,

    imp: Vec<Imp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    site: Option<Site>,

    #[serde(skip_serializing_if = "Option::is_none")]
    app: Option<App>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device: Option<Device>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<User>,

    #[serde(
        default = "serde_utils::default_false",
        skip_serializing_if = "serde_utils::is_false",
        serialize_with = "serde_utils::bool_to_u8",
        deserialize_with = "serde_utils::u8_to_bool"
    )]
    test: bool,

    #[serde(rename = "at")]
    auction_type: AuctionType,

    tmax: u64,

    #[serde(rename = "wseat", default, skip_serializing_if = "Vec::is_empty")]
    seat_whitelist: Vec<String>,

    #[serde(rename = "bseat", default, skip_serializing_if = "Vec::is_empty")]
    seat_blocklist: Vec<String>,

    #[serde(
        rename = "allimps",
        default = "serde_utils::default_false",
        skip_serializing_if = "serde_utils::is_false",
        serialize_with = "serde_utils::bool_to_u8",
        deserialize_with = "serde_utils::u8_to_bool"
    )]
    all_imps: bool,

    #[serde(rename = "cur", default, skip_serializing_if = "Vec::is_empty")]
    currency: Vec<String>,

    #[serde(rename = "wlang", default, skip_serializing_if = "Vec::is_empty")]
    language_whitelist: Vec<String>,

    #[serde(rename = "bcat", default, skip_serializing_if = "Vec::is_empty")]
    category_blocklist: Vec<Category>,

    #[serde(rename = "badv", default, skip_serializing_if = "Vec::is_empty")]
    advertiser_blocklist: Vec<String>,

    #[serde(rename = "bapp", default, skip_serializing_if = "Vec::is_empty")]
    app_blocklist: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Source>,

    #[serde(rename = "regs", skip_serializing_if = "Option::is_none")]
    regulations: Option<Regulations>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ext: Option<serde_utils::Ext>,
}

impl BidRequest {
    pub fn validate(&self) -> bool {
        self.imp.len() > 0
    }
}

impl serde::Serialize for AuctionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            AuctionType::FirstPrice => serializer.serialize_u32(1),
            AuctionType::SecondPricePlus => serializer.serialize_u32(2),
            AuctionType::ExchangeSpecific(t) => serializer.serialize_u32(t),
        }
    }
}

impl<'de> serde::Deserialize<'de> for AuctionType {
    fn deserialize<D>(deserializer: D) -> Result<AuctionType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match serde::Deserialize::deserialize(deserializer) {
            Ok(1) => Ok(AuctionType::FirstPrice),
            Ok(2) => Ok(AuctionType::SecondPricePlus),
            Ok(t) => Ok(AuctionType::ExchangeSpecific(t)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialization_skip_fields() {
        let b = BidRequest {
            id: "1234".to_string(),
            imp: vec![],
            site: None,
            app: None,
            device: None,
            user: None,
            test: false,
            auction_type: AuctionType::FirstPrice,
            tmax: 0,
            seat_whitelist: vec![],
            seat_blocklist: vec![],
            all_imps: false,
            currency: vec![],
            language_whitelist: vec![],
            category_blocklist: vec![],
            advertiser_blocklist: vec![],
            app_blocklist: vec![],
            source: None,
            regulations: None,
            ext: None,
        };

        let expected = r#"{"id":"1234","imp":[],"at":1,"tmax":0}"#;
        let serialized = serde_json::to_string(&b).unwrap();

        assert_eq!(expected, serialized)
    }

    #[test]
    fn deserialize_defaults() {
        let serialized = r#"{
            "id": "1234",
            "imp": [],
            "at": 2,
            "tmax": 0
        }"#;

        let res = serde_json::from_str(serialized);

        let b: BidRequest = match res {
            Ok(x) => x,
            Err(e) => panic!("{:?}", e),
        };

        let expected = BidRequest {
            id: "1234".to_string(),
            imp: vec![],
            site: None,
            app: None,
            device: None,
            user: None,
            test: false,
            auction_type: AuctionType::SecondPricePlus,
            tmax: 0,
            seat_whitelist: vec![],
            seat_blocklist: vec![],
            all_imps: false,
            currency: vec![],
            language_whitelist: vec![],
            category_blocklist: vec![],
            advertiser_blocklist: vec![],
            app_blocklist: vec![],
            source: None,
            regulations: None,
            ext: None,
        };

        assert_eq!(expected.id, b.id);
        assert_eq!(expected.auction_type, b.auction_type);
    }
}
