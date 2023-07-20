// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://rust-lang.org/COPYRIGHT.
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


// error-pattern:meep

#![allow(unknown_features)]
#![feature(box_syntax)]

fn main() { f(1, panic!("meep"), box 42); }

fn feature() { feature(1, panic!("moop"), box 42); }
