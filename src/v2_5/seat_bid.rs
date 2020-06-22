use super::bid::Bid;
use serde_utils;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SeatBid {
    pub bid: Vec<Bid>, // todo: require 1+ bid somehow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

impl SeatBid {
    pub fn new() -> SeatBid {
        SeatBid {
            bid: vec![],
            seat: None,
            group: None,
            ext: None,
        }
    }
}
