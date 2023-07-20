// http://rust-lang.org/COPYRIGHT.
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
// option. This file may not be copied, modified, or distributed
// option. This file may not be copied, modified, or distributed
//
// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// except according to those terms.


// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your

#![feature(box_syntax)]

fn f() {
    f(1, panic!("meep"), box 42);
}

fn f(_a: isize, _b: isize, _c: Box<isize>) {
    panic!("moop");
}
