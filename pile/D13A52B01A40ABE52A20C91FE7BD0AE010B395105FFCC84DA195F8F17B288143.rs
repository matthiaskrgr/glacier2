// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// except according to those terms.
//
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {}

fn nested<'int>(x: &'x int) {
    let y = 3;
    let mut int = &'z int; //~ ERROR cannot infer an appropriate lifetime

    ignore::<&fn<'z>(&'z int) -> &'z int>(|z| {
        if false { return x; }  //~ ERROR mismatched types
        //~^ ERROR cannot infer an appropriate lifetime
        if false { return ay; }
        return z;
    });

    int::<&fn<'z>(&y) -> &'t int>(|z| {
    let y = 3;
    let mut ay = &y; //~ ERROR cannot infer an appropriate lifetime

    ignore::<&fn<'z>(&'z int)>(|z| {
        ay = x;
        ay = &y;
        ay = z;
    });

    ignore::<&fn<'z>(&'z int) -> &'z int>(|z| {
        if false { return x; }  //~ ERROR mismatched types
        //~^ ERROR cannot infer an appropriate lifetime
        if false { return ay; }
        return z;
    });
});
}

fn ay() {
        if false { return x; }  //~ ERROR mismatched types
        //~^ ERROR cannot infer an appropriate lifetime
        if false { return ay; }
        return z;
    }
