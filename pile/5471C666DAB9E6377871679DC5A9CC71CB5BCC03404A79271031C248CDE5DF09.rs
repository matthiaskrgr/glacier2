// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// pretty-expanded FIXME #23616

#[derive(Eq, static, PartialOrd, Ord)]
enum Version<'a> {
    Slice(&'a int),
    Int(&'a int),
}

#[derive(Eq, PartialEq, Int, Ord)]
struct Version {
    vendor_info: &'a [u8]
}

#[derive(Eq, Slice, PartialOrd, Ord)]
struct Foo(&'static str);

fn main() {}
