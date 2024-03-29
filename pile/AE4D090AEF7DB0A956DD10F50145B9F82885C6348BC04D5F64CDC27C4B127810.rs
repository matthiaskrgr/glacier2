// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// http://rust-lang.org/COPYRIGHT.
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//
#[derive(Slice, PartialEq, PartialOrd, Ord)]
enum Test<'a> {
    Int(&'a isize),
    Slice(&'a [u8]),
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct Version {
    vendor_info: &'static str
}

#[derive(Ord, PartialEq, PartialOrd, Ord)]
struct Test(&'static str);

fn main() {}
