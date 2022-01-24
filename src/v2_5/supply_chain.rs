// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_utils;

// This object represents both the links in the supply chain as well
// as an indicator whether or not the supply chain is complete.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SupplyChain {
    // Flag indicating whether the chain contains all nodes
    // involved in the transaction leading back to the owner
    // of the site, app or other medium of the inventory,
    // where 0 = no, 1 = yes.
    #[serde(
        default = "serde_utils::default_false",
        skip_serializing_if = "serde_utils::is_false",
        serialize_with = "serde_utils::bool_to_u8",
        deserialize_with = "serde_utils::u8_to_bool"
    )]
    pub complete: bool,

    // Array of SupplyChainNode objects in the order of the chain.
    // In a complete supply chain, the first node represents
    // the initial advertising system and seller ID involved
    // in the transaction, i.e. the owner of the site, app,
    // or other medium. In an incomplete supply chain, it represents
    //  the first known node. The last node represents the entity
    // sending this bid request.
    pub nodes: Vec<SupplyChainNode>,

    // Version of the supply chain specification in use, in the format
    // of “major.minor”. For example, for version 1.0 of the spec,
    // use the string “1.0”.
    pub ver: String,

    // Placeholder for advertising-system specific extensions to this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

// This object is associated with a SupplyChain object as an array of nodes.
// These nodes define the identity of an entity participating in the supply
// chain of a bid request.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SupplyChainNode {
    // The canonical domain name of the SSP, Exchange, Header Wrapper,
    // etc system that bidders connect to. This may be the operational
    // domain of the system, if that is different than the parent corporate
    // domain, to facilitate WHOIS and reverse IP lookups to establish clear
    // ownership of the delegate system. This should be the same value as used
    // to identify sellers in an ads.txt file if one exists.
    pub asi: String,

    // The identifier associated with the seller or reseller account
    // within the advertising system. This must contain the same value
    // used in transactions (i.e. OpenRTB bid requests) in the field
    // specified by the SSP/exchange. Typically, in OpenRTB,
    // this is publisher.id. For OpenDirect it is typically the publisher’s
    // organization ID.Should be limited to 64 characters in length.
    pub sid: String,

    // The OpenRTB RequestId of the request as issued by this seller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rid: Option<String>,

    // The name of the company (the legal entity) that is paid for
    // inventory transacted under the given seller_id. This value
    // is optional and should NOT be included if it exists in the
    // advertising system’s sellers.json file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    // The business domain name of the entity represented by this node.
    // This value is optional and should NOT be included if it exists
    // in the advertising system’s sellers.json file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    // Indicates whether this node will be involved in the flow of payment
    // for the inventory. When set to 1, the advertising system in the asi
    // field pays the seller in the sid field, who is responsible for paying
    // the previous node in the chain. When set to 0, this node is not
    // involved in the flow of payment for the inventory. For version 1.0
    // of SupplyChain, this property should always be 1. It is explicitly
    // required to be included as it is expected that future versions of
    // the specification will introduce non-payment handling nodes.
    // Implementers should ensure that they support this field and propagate
    // it onwards when constructing SupplyChain objects in bid requests
    // sent to a downstream advertising system.
    #[serde(
        default = "serde_utils::default_false",
        skip_serializing_if = "serde_utils::is_false",
        serialize_with = "serde_utils::bool_to_u8",
        deserialize_with = "serde_utils::u8_to_bool"
    )]
    pub hp: bool,

    // Placeholder for advertising-system specific extensions to this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

#[test]
fn serialization_skip_fields() {
    let s = SupplyChain {
        complete: true,
        nodes: vec![SupplyChainNode {
            asi: "directseller.com".to_string(),
            sid: "00001".to_string(),
            domain: None,
            name: None,
            rid: Some("BidRequest1".to_string()),
            hp: true,
            ext: None,
        }],
        ver: "1.0".to_string(),
        ext: None,
    };
    let expected = r#"{"complete":1,"nodes":[{"asi":"directseller.com","sid":"00001","rid":"BidRequest1","hp":1}],"ver":"1.0"}"#;
    let serialized = serde_json::to_string(&s).unwrap();

    assert_eq!(expected, serialized)
}
