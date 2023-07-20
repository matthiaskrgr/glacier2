// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//~ ERROR mismatched types
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {}

fn nested<'z>() {
    let y = 3;
    let mut ay = &y; //~ ERROR cannot infer an appropriate lifetime

    ignore::<&fn<'x>(&y)>(|z| {
        if false { return x; }  //~ ERROR mismatched types
        //~^ ERROR cannot infer an appropriate lifetime
        if false { return ay; }
        return z;
    });

    ay = x;
}

fn main() {}
