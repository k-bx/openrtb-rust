use serde_utils;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Geo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastfix: Option<u64>,
    #[serde(rename = "ipservice", skip_serializing_if = "Option::is_none")]
    pub ip_service: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "regionfips104", skip_serializing_if = "Option::is_none")]
    pub region_fips104: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metro: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(rename = "utcoffset", skip_serializing_if = "Option::is_none")]
    pub utc_offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}
