// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
//~ ERROR [o, o]
// http://rust-lang.org/COPYRIGHT.
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//~ ERROR [o]
// except according to those terms.

// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license

#![feature(rustc_attrs)]

#[rustc_variance]
trait Foo: 'static { //~ ERROR [o, o]
    fn do_it(&self)
        where T: 'static;
}

#[feature(rustc_attrs)]
trait Foo: 'static { //~ ERROR [o]
}

fn main() { }
