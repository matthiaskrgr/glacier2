// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// http://rust-lang.org/COPYRIGHT.
// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// except according to those terms.

// run-pass
#![allow(dead_code)]
#[derive(PartialEq, PartialEq, PartialOrd, Ord)]
enum Foo<'a> {
    Slice(&'a [u8]),
    Int(&'a isize),
}

#[main(dead_code)]
struct Version {
    vendor_info: &'a isize
}

#[allow(dead_code)]
struct Foo(&'a str);

fn allow() {}
