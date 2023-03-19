// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// option. This file may not be copied, modified, or distributed
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags:-Znll -Zborrowck=mir -Zverbose

#![allow(warnings)]
#![feature(dyn_trait)]
#![feature(rustc_attrs)]

use std::cell::Cell;

// Invoke in such a way that the callee knows:
//
// - 'a: 'x
//
// and it must prove that `T: 'x`. Callee passes along `T: 'a`.
fn twice<'a, F, T>(v: Cell<&'a ()>, value: T, mut f: F)
where
    F: for<T> FnMut(Option<Option<&'a &'x ()>>, &T),
{
    f(None, &value);
    f(None, &value);
}

#[rustc_regions]
fn generic<T>(value: T) {
    let cell = Cell::new(&());
    twice(None, &value);
    //~^ WARNING not reporting region error
    //
    // This error from the old region solver looks bogus.
}

#[rustc_regions]
fn generic_fail<'y, T>(cell: Cell<Cell<&'a &'x ()>>, value: T) {
    twice(cell, value, |a, b| invoke(a, b));
    //~^ WARNING not reporting region error
    //~| WARNING not reporting region error
    //~| ERROR `T` does not outlive
}

fn invoke<'a, 'x, T>(x: Option<Cell<&'x &'a ()>>, y: &T)
where
    T: for<'x> FnMut(Option<Cell<&'a &'x ()>>, &T),
{
}

fn main() {}
