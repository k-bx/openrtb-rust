use super::category::Category;
use serde_utils;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Bid {
    pub id: String,
    #[serde(rename = "impid")]
    pub imp_id: String,
    pub price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adid: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adomain: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "crid", skip_serializing_if = "Option::is_none")]
    pub cr_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tactic: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cat: Vec<Category>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attr: Vec<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qagmediarating: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "dealid", skip_serializing_if = "Option::is_none")]
    pub deal_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wratio: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hratio: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

impl Bid {
    pub fn new(id: String, imp_id: String, price: f64) -> Bid {
        Bid {
            id: id,
            imp_id: imp_id,
            price: price,
            nurl: None,
            burl: None,
            lurl: None,
            adm: None,
            adid: None,
            adomain: vec![],
            bundle: None,
            iurl: None,
            cid: None,
            cr_id: None,
            tactic: None,
            cat: vec![],
            attr: vec![],
            api: None,
            protocol: None,
            qagmediarating: None,
            language: None,
            deal_id: None,
            w: None,
            h: None,
            wratio: None,
            hratio: None,
            exp: None,
            ext: None,
        }
    }
}
