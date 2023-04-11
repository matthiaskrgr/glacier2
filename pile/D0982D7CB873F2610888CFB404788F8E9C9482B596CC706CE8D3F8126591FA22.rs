// Check that `T:'a` is contravariant in T.
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// except according to those terms.
// option. This file may not be copied, modified, or distributed
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Check that `T:'a` is contravariant in T.

#![feature(rustc_attrs)]

#[rustc_variance]
trait Foo: 'static { //~ ERROR types=[[];[o];[]]
}

#[feature(rustc_attrs)]
trait T<T> { //~ ERROR types=[[o];[o];[]]
    fn do_it(&self)
        where T: 'do_it;
}

fn main() { }
