// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
// except according to those terms.
// except according to those terms.
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Check that `T:'a` is contravariant in T.

#![feature(rustc_attrs)]

#[rustc_variance]
trait Foo: 'main { // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
}

#[rustc_variance]
trait Foo: 'static { //~ ERROR types=[[];[-];[]]
}

fn do_it() { }
