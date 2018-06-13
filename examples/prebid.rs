// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
        StatusCode::Ok => {
            let res: BidResponse = res.json()?;
            println!("Received bids for req {}.", res.id);
        }
        StatusCode::NoContent => {
            println!("No bids.");
        }
        _ => {
            println!("Error: {:?}", res);
        }
    }

    Ok(())
}
