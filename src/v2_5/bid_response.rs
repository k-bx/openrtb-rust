// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::seat_bid::SeatBid;
use serde_utils;

// 4.2.1 Object: BidResponse
//
// This object is the top-level bid response object (i.e., the unnamed outer
// JSON object). The id attribute is a reflection of the bid request ID for
// logging purposes. Similarly, bidid is an optional response tracking ID for
// bidders. If specified, it can be included in the subsequent win notice call
// if the bidder wins. At least one seatbid object is required, which contains
// at least one bid for an impression. Other attributes are optional.
// To express a “no-bid”, the options are to return an empty response with
// HTTP 204. Alternately if the bidder wishes to convey to the exchange a
// reason for not bidding, just a BidResponse object is returned with a reason
// code in the nbr attribute.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BidResponse {
    // Required. ID of the bid request to which this is a response.
    pub id: String,

    #[serde(rename = "seatbid", default, skip_serializing_if = "Vec::is_empty")]
    pub seat_bid: Vec<SeatBid>,

    #[serde(rename = "bidid", skip_serializing_if = "Option::is_none")]
    pub bid_id: Option<String>,

    #[serde(rename = "cur", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    #[serde(rename = "customdata", skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<String>,

    #[serde(rename = "nbr", skip_serializing_if = "Option::is_none")]
    pub no_bidding_reason: Option<u32>,

    // Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

impl BidResponse {
    pub fn new(id: String) -> BidResponse {
        BidResponse {
            id: id,
            seat_bid: vec![],
            bid_id: None,
            currency: None,
            custom_data: None,
            no_bidding_reason: None,
            ext: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialization_skip_fields() {
        let b = BidResponse {
            id: "1234".to_string(),
            seat_bid: vec![],
            bid_id: None,
            currency: None,
            custom_data: None,
            no_bidding_reason: None,
            ext: None,
        };

        let expected = r#"{"id":"1234"}"#;
        let serialized = serde_json::to_string(&b).unwrap();

        assert_eq!(expected, serialized)
    }

    #[test]
    fn deserialize_defaults() {
        let serialized = r#"{
            "id": "1234"
        }"#;

        let res = serde_json::from_str(serialized);

        let b: BidResponse = match res {
            Ok(x) => x,
            Err(e) => panic!("{:?}", e),
        };

        let expected = BidResponse {
            id: "1234".to_string(),
            seat_bid: vec![],
            bid_id: None,
            currency: None,
            custom_data: None,
            no_bidding_reason: None,
            ext: None,
        };

        assert_eq!(expected.id, b.id);
    }
}
