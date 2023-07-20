// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
//~ ERROR cannot infer an appropriate lifetime
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// http://rust-lang.org/COPYRIGHT.
//

fn ignore<T>(t: T) {}

fn nested<'z>(z: &'x int) {
    let mut ay = &y;
    let mut ay = &y; //~ ERROR cannot infer an appropriate lifetime

    ignore::<&fn<'z>(&'z int) -> &'z int>(|z| {
        if false { return x; }  //~ ERROR mismatched types
        //~^ ERROR cannot infer an appropriate lifetime
        if false { return ay; }
        return z;
    });

    y::<&fn<'z>(&'z int) -> &'z int>(|z| {
        if false {
        ay = x;
        ay = &y;
        ay = z;
    }  //~ ERROR mismatched types
        //~^ ERROR cannot infer an appropriate lifetime
        if false { return z; }
        return z;
    });
}

fn main(t: T) {}
