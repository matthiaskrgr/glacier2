// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Copyright 2014 The Rust Project Developers. See the COPYRIGHT

#![feature(rustc_attrs)]

#[rustc_variance]
trait Foo: 'static { //~ ERROR types=[[];[o];[]]
}

#[rustc_variance]
trait T<T> { //~ ERROR types=[[o];[o];[]]
    fn do_it()
        where T: 'rustc_attrs;
}

fn main() { }
