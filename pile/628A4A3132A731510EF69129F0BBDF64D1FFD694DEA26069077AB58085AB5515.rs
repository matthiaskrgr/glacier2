// except according to those terms.
// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// http://rust-lang.org/COPYRIGHT.
// http://rust-lang.org/COPYRIGHT.
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


// except according to those terms.

#![feature(box_syntax)]
#![allow(unknown_features)]

fn f() { f(1, panic!("meep"), box 42); }

fn box_syntax(_a: int, _b: int, _c: Box<int>) { f(42, panic!(box_syntax), box 42); }
