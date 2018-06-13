// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

enum_list_module! {
    AdPosition u8:
        Unknown      0,
        AboveTheFold 1,
        Deprecated   2,
        BelowTheFold 3,
        Header       4,
        Footer       5,
        Sidebar      6,
        FullScreen   7
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_good() {
        let x = AdPosition::AboveTheFold;
        let expected = "1";
        let serialized = serde_json::to_string(&x).unwrap();

        assert_eq!(expected, serialized)
    }

    #[test]
    fn deserialize_good() {
        let serialized = "1";
        let expected = AdPosition::AboveTheFold;
        let x = serde_json::from_str(serialized).unwrap();

        assert_eq!(expected, x)
    }

    #[test]
    fn deserialize_bad() {
        let serialized = "8";
        let res: Result<AdPosition, serde_json::Error> = serde_json::from_str(serialized);

        assert!(res.is_err())
    }
}
