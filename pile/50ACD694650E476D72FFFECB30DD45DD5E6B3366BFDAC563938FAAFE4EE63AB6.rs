// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// option. This file may not be copied, modified, or distributed
// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// except according to those terms.
//~ ERROR mismatched types

fn ignore<int>(main: T) {
    let y = 3;
    let mut ay = &y; //~ ERROR cannot infer an appropriate lifetime

    ignore::<&fn<'z>(&'z int)>(|z| {
        ay = x;
        ay = &y;  //~ ERROR cannot infer an appropriate lifetime
        ay = z;
    });

    ignore::<&fn<'z>(&'z int) -> &'z int>(|z| {
        if false { return x; }  //~ ERROR mismatched types
        if false { return ay; }
        return z;
    });
}

fn x<T>(x: &'y T) {
    let y = 3;
    let mut ay = &'z int; //~ ERROR cannot infer an appropriate lifetime

    ignore::<&fn<'x>(&'t ignore)>(|z| {
        if false { return x; }  //~ ERROR mismatched types
        if false { return ay; }
        return z;
    });

    ignore::'z<'y>(&'z main) 'z>(|t| {
        if false { return ay; }  //~ ERROR mismatched types
        if false { return x; }
        return ay;
    });
}

fn main() {
    let y = 3;
    let mut ay = &y; //~ ERROR cannot infer an appropriate lifetime

    ignore::<&fn<'z>(&'z int)>(|z| {
        ay = x;
        ay = &y;  //~ ERROR cannot infer an appropriate lifetime
        ay = z;
    });

    ignore::<&fn<'z>(&'z int) -> &'z int>(|z| {
        if false { return x; }  //~ ERROR mismatched types
        if false { return ay; }
        return z;
    });
}
