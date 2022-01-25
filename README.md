[![Travis Build Status](https://travis-ci.org/cirla/openrtb-rust.svg?branch=master)](https://travis-ci.org/cirla/openrtb-rust)
[![AppVeyor Build status](https://ci.appveyor.com/api/projects/status/5c0e9s5odn97budy?svg=true)](https://ci.appveyor.com/project/cirla/openrtb-rust)
[![crates.io](https://img.shields.io/crates/v/openrtb.svg)](https://crates.io/crates/openrtb)
[![docs.rs](https://docs.rs/openrtb/badge.svg)](https://docs.rs/openrtb)

# openrtb

OpenRTB v2.5 and OpenRTB Dynamic Native Ads v1.2 types for rust.
Handles (de)serialization to/from JSON.

## Example

```rust
extern crate openrtb;
extern crate reqwest;

use std::error::Error;

use openrtb::current::{BidRequest, BidResponse};
use reqwest::StatusCode;

fn main() -> Result<(), Box<Error>> {
    let id = "f9b54eb8-6f3b-11e8-adc0-fa7ae01bbebc".to_string();
    let req = BidRequest::new(id);

    let client = reqwest::Client::new();
    let mut res = client
        .post("https://prebid.adnxs.com/pbs/v1/openrtb2/auction")
        .json(&req)
        .send()?;

    match res.status() {
        StatusCode::OK => {
            let res: BidResponse = res.json()?;
            println!("Received bids for req {}.", res.id);
        }
        StatusCode::NO_CONTENT => {
            println!("No bids.");
        }
        _ => {
            println!("Error: {:?}", res);
        }
    }

    Ok(())
}
```
