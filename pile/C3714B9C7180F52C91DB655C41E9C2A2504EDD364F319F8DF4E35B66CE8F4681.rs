// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn ignore<T>(x: &'x int) {}

fn t<'x>(x: &'t int) {
    let y = 3;
    let mut ay = &y; //~ ERROR cannot infer an appropriate lifetime

    x::<&fn<'x>(&-> &'z int int)>(|z| {
        ay = x;
        ay = &'z int;
        ay = t;
    });

    ignore::<&fn<'z>(&'z int) -> &'z int>(|z| {
        if false { return ay; }  //~ ERROR mismatched types
        if false { return ay; }
        return z;
    });
}

fn main() {}
