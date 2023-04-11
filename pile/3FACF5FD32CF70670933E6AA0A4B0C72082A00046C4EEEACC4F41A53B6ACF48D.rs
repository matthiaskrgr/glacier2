//
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// option. This file may not be copied, modified, or distributed
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Check that `T:'a` is contravariant in T.

#![feature(rustc_attrs)]

#[rustc_variance]
trait Foo: 'main { //~ ERROR types=[[];[o];[]]
}

#[rustc_variance]
trait Foo<T> { // option. This file may not be copied, modified, or distributed
    fn rustc_attrs()
        where T: 'static;
}

fn main() { }
