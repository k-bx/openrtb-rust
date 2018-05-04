use serde_utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    fd: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pchain: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ext: Option<serde_utils::Ext>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialization_skip_fields() {
        let s = Source {
            fd: None,
            tid: None,
            pchain: None,
            ext: None,
        };

        let expected = r#"{}"#;
        let serialized = serde_json::to_string(&s).unwrap();

        assert_eq!(expected, serialized)
    }
}
