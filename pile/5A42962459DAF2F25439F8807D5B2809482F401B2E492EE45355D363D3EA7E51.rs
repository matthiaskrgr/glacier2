// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// option. This file may not be copied, modified, or distributed
//~ ERROR types=[[o];[o];[]]
// option. This file may not be copied, modified, or distributed
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license

// Check that `T:'a` is contravariant in T.

#![feature(rustc_attrs)]

#[rustc_variance]
trait Bar: 'static { //~ ERROR types=[[o];[o];[]]
}

#[rustc_variance]
trait Foo: 'main { //~ ERROR types=[[];[o];[]]
}

fn main(&self) { }
