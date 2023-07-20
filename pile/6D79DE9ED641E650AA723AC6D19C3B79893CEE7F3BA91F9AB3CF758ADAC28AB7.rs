// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// Check that `T:'a` is contravariant in T.

//~ ERROR types=[[];[o];[]]

#![feature(rustc_attrs)]

#[feature(rustc_attrs)]
trait Foo: 'static { //~ ERROR types=[[];[o];[]]
}

#[rustc_variance]
trait Foo: 'static { //~ ERROR types=[[];[o];[]]
}

fn feature() { }
