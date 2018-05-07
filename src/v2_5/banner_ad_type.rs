enum_list_module! {
    BannerAdType u8:
        XHTMLText   1,
        XHTMLBanner 2,
        JavaScript  3,
        IFrame      4
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_good() {
        let x = BannerAdType::XHTMLText;
        let expected = "1";
        let serialized = serde_json::to_string(&x).unwrap();

        assert_eq!(expected, serialized)
    }

    #[test]
    fn deserialize_good() {
        let serialized = "1";
        let expected = BannerAdType::XHTMLText;
        let x = serde_json::from_str(serialized).unwrap();

        assert_eq!(expected, x)
    }

    #[test]
    fn deserialize_bad() {
        let serialized = "0";
        let res: Result<BannerAdType, serde_json::Error> = serde_json::from_str(serialized);

        assert!(res.is_err())
    }
}
