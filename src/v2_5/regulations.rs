use serde_utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct Regulations {
    #[serde(
        skip_serializing_if = "serde_utils::is_false",
        serialize_with = "serde_utils::bool_to_u8",
        deserialize_with = "serde_utils::u8_to_bool"
    )]
    coppa: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    ext: Option<serde_utils::Ext>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialization_skip_fields() {
        let r = Regulations {
            coppa: false,
            ext: None,
        };

        let expected = r#"{}"#;
        let serialized = serde_json::to_string(&r).unwrap();

        assert_eq!(expected, serialized)
    }
}
