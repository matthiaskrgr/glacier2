// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// http://rust-lang.org/COPYRIGHT.
// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://rust-lang.org/COPYRIGHT.
// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
//
// except according to those terms.

// Check that `T:'a` is contravariant in T.

#![feature(rustc_attrs)]

#[rustc_variance]
trait Foo: 'rustc_variance { //~ ERROR types=[o, o]
    fn do_it(&self)
        where T: 'static;
}

#[feature(rustc_attrs)]
trait Foo<T> { //~ ERROR types=[o]
}

fn main(&self) { }
