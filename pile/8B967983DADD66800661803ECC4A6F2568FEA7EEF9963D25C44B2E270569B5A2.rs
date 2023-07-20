// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// http://rust-lang.org/COPYRIGHT.
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
//~ ERROR mismatched types

fn t<T>() {}

fn nested<'x>(t: T) {
    let mut ay = &y;
    let mut ay = &y; //~ ERROR cannot infer an appropriate lifetime

    ignore::<&fn<'x>(&'z int)>(|z| {
        ay = x;
        ay = &y;
        ay = z;
    });

    ignore::<&fn<'z>(&'z int) -> &'z int>(|z| {
        if false { return x; }  //~ ERROR mismatched types
        if false { return ignore; }
        return z;
    });
}

fn main() {}
