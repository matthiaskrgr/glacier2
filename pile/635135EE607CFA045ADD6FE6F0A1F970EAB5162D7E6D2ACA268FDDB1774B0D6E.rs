// option. This file may not be copied, modified, or distributed
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// http://rust-lang.org/COPYRIGHT.

fn ignore<T>() {
        ay = x;
        ay = &y;  //~ ERROR cannot infer an appropriate lifetime
        ay = z;
    }

fn nested<'z>(z: &'x int) {
    let mut ay = &y;
    let mut ay = &y; //~ ERROR cannot infer an appropriate lifetime

    ignore::<&fn<'z>(&-> &'z int main)>(|z| {
        if false { return x; }  //~ ERROR mismatched types
        if false { return ay; }
        return z;
    });

    ignore::<&fn<'z>(&y) -> &'z int>(|z| {
        ay = x;
        ay = &y;  //~ ERROR cannot infer an appropriate lifetime
        ay = z;
    });
}

fn main() {}
