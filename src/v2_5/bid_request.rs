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

#[derive(Debug, PartialEq, Clone)]
pub enum AuctionType {
    FirstPrice,
    SecondPricePlus,
    ExchangeSpecific(u32),
}

// 3.2.1 Object: BidRequest
//
// The top-level bid request object contains a globally unique bid request or
// auction ID. This id attribute is required as is at least one impression
// object (Section 3.2.4). Other attributes in this top-level object establish
// rules and restrictions that apply to all impressions being offered. There
// are also several subordinate objects that provide detailed data to potential
// buyers. Among these are the Site and App objects, which describe the type of
// published media in which the impression(s) appear. These objects are highly
// recommended, but only one applies to a given bid request depending on whether
// the media is browser-based web content or a non-browser application,
// respectively.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BidRequest {
    // Required. Unique ID of the bid request, provided by the exchange.
    pub id: String,

    // Required. Array of Imp objects (Section 3.2.4) representing the
    // impressions offered. At least 1 Imp object is required.
    pub imp: Vec<Imp>,

    // Details via a Site object (Section 3.2.13) about the publisher's
    // website. Only applicable and recommended for websites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,

    // Details via an App object (Section 3.2.14) about the publisher's
    // app (i.e., non-browser applications). Only applicable and
    // recommended for apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,

    // Details via a Device object (Section 3.2.18) about the user's
    // device to which the impression will be delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,

    // Details via a User object (Section 3.2.20) about the human
    // user of the device; the advertising audience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    // Indicator of test mode in which auctions are not billable,
    // where 0 = live mode, 1 = test mode. Default to false.
    #[serde(
        default = "serde_utils::default_false",
        skip_serializing_if = "serde_utils::is_false",
        serialize_with = "serde_utils::bool_to_u8",
        deserialize_with = "serde_utils::u8_to_bool"
    )]
    pub test: bool,

    // Auction type, where 1 = First Price, 2 = Second Price Plus.
    // Exchange-specific auction types can be defined using values
    // greater than 500.
    #[serde(rename = "at")]
    pub auction_type: AuctionType,

    // Maximum time in milliseconds the exchange allows for bids to
    // be received including Internet latency to avoid timeout. This
    // value supersedes any a priori guidance from the exchange.
    pub tmax: u64,

    // White list of buyer seats (e.g., advertisers, agencies) allowed
    // to bid on this impression. IDs of seats and knowledge of the
    // buyer’s customers to which they refer must be coordinated
    // between bidders and the exchange a priori. At most, only one
    // of wseat and bseat should be used in the same request.
    // Omission of both implies no seat restrictions.
    #[serde(rename = "wseat", default, skip_serializing_if = "Vec::is_empty")]
    pub seat_whitelist: Vec<String>,

    // Block list of buyer seats (e.g., advertisers, agencies) restricted
    // from bidding on this impression. IDs of seats and knowledge
    // of the buyer’s customers to which they refer must be
    // coordinated between bidders and the exchange a priori. At
    // most, only one of wseat and bseat should be used in the
    // same request. Omission of both implies no seat restrictions
    #[serde(rename = "bseat", default, skip_serializing_if = "Vec::is_empty")]
    pub seat_blocklist: Vec<String>,

    // Flag to indicate if Exchange can verify that the impressions
    // offered represent all of the impressions available in context
    // (e.g., all on the web page, all video spots such as pre/mid/post
    // roll) to support road-blocking. 0 = no or unknown, 1 = yes, the
    // impressions offered represent all that are available.
    #[serde(
        rename = "allimps",
        default = "serde_utils::default_false",
        skip_serializing_if = "serde_utils::is_false",
        serialize_with = "serde_utils::bool_to_u8",
        deserialize_with = "serde_utils::u8_to_bool"
    )]
    pub all_imps: bool,

    // Array of allowed currencies for bids on this bid request using
    // ISO-4217 alpha codes. Recommended only if the exchange
    // accepts multiple currencies.
    #[serde(rename = "cur", default, skip_serializing_if = "Vec::is_empty")]
    pub currency: Vec<String>,

    // White list of languages for creatives using ISO-639-1-alpha-2.
    // Omission implies no specific restrictions, but buyers would be
    // advised to consider language attribute in the Device and/or
    // Content objects if available.
    #[serde(rename = "wlang", default, skip_serializing_if = "Vec::is_empty")]
    pub language_whitelist: Vec<String>,

    // Blocked advertiser categories using the IAB content
    // categories. Refer to List 5.1.
    #[serde(rename = "bcat", default, skip_serializing_if = "Vec::is_empty")]
    pub category_blocklist: Vec<Category>,

    // Block list of advertisers by their domains (e.g., “ford.com”).
    #[serde(rename = "badv", default, skip_serializing_if = "Vec::is_empty")]
    pub advertiser_blocklist: Vec<String>,

    // Block list of applications by their platform-specific exchange-independent
    // application identifiers. On Android, these should
    //be bundle or package names (e.g., com.foo.mygame). On iOS,
    // these are numeric IDs.
    #[serde(rename = "bapp", default, skip_serializing_if = "Vec::is_empty")]
    pub app_blocklist: Vec<String>,

    // A Source object (Section 3.2.2) that provides data about the
    // inventory source and which entity makes the final decision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    // A Regs object (Section 3.2.3) that specifies any industry, legal,
    // or governmental regulations in force for this request.
    #[serde(rename = "regs", skip_serializing_if = "Option::is_none")]
    pub regulations: Option<Regulations>,

    // Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

impl BidRequest {
    pub fn new(id: String) -> BidRequest {
        BidRequest {
            id: id,
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
        }
    }

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
