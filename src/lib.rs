// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate phf;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[macro_use]
mod macros;
mod serde_utils;

pub mod native;
pub mod v2_5;

pub use v2_5 as current;

#[cfg(test)]
mod tests {
    use super::current::*;

    #[test]
    fn test_mods() {
        let _c = Category::Automotive(Automotive::Automotive);
    }
}
