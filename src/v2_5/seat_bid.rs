use serde_utils;
use super::bid::Bid;

#[derive(Serialize, Deserialize, Debug)]
pub struct SeatBid {
    pub bid: Vec<Bid>, // todo: require 1+ bid somehow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}
