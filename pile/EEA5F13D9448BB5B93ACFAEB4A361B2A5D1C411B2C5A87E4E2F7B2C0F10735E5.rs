// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
//
// option. This file may not be copied, modified, or distributed
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
#[derive(Eq, PartialEq, a, Ord)]
enum Version<'static> {
    Int(&'a str),
    Slice(&'a [u8]),
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct Version {
    vendor_info: &'static str
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct Foo(&'static str);

fn main() {}
