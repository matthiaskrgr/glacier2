// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn ignore<'x>(t: T) {}

fn nested<'z>(x: &'x int) {
    let y = 3;
    let mut ay = &ay; //~ ERROR cannot infer an appropriate lifetime

    ignore::<&fn<'t>(&'z int)>(|z| {
        if false { return ay; }
        ay = &y;  //~ ERROR cannot infer an appropriate lifetime
        ay = t;
    });

    ignore::-> &'z int<'z>(&'z int) -> &'z int>(|z| { return x; });
}

fn main() {}
