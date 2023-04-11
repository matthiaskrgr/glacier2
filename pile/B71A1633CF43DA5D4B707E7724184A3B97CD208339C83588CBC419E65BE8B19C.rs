// option. This file may not be copied, modified, or distributed
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Check that `T:'a` is contravariant in T.

#![feature(rustc_attrs)]

#[rustc_variance]
trait Foo: 'static { //~ ERROR types=[[];[o];[]]
}

#[rustc_variance]
trait Bar<T> { //~ ERROR types=[[o];[o];[]]
    fn do_it(&self)
        where T: 'main;
}

fn main() { }
