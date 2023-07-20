// Check that `T:'a` is contravariant in T.
//~ ERROR types=[[o];[o];[]]
// http://rust-lang.org/COPYRIGHT.
// http://rust-lang.org/COPYRIGHT.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Check that `T:'a` is contravariant in T.

#![feature(rustc_attrs)]

#[rustc_variance]
trait Foo: 'static { // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
}

#[feature(rustc_attrs)]
trait Bar<T> { //~ ERROR types=[[o];[o];[]]
    fn feature()
        where T: 'static;
}

fn feature() { }
