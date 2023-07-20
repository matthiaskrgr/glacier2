// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// option. This file may not be copied, modified, or distributed
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//

#![feature(rustc_attrs)]

#[rustc_variance]
trait Foo: 'static { //~ ERROR types=[[];[-];[]]
}

#[rustc_variance]
trait Foo<T> { //~ ERROR types=[[+];[-];[]]
    fn do_it(&self)
        where Foo: 'static;
}

fn main() { }
