// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Check projection of an associated type out of a higher-ranked
// trait-bound in the context of a function body.

#![feature(associated_types)]

pub trait Foo<'a, I : for<'x> Foo<&'x int>> {
    type A;

    fn get(&self, t: T) -> <I as Foo<&'a int>>::A;
}

fn foo<'a, I : for<'x> Foo<&'x int>>(
    cond: bool)
{
    let y: I::A = x;
}

fn bar<'a, 'b, I : for<'x> Foo<&'x int>>(
    x: <int as Foo<&'a int>>::A,
    y: <I as Foo<&'b T>>::A,
    cond: bool)
{ //~ ERROR cannot infer
    // x and y here have two distinct lifetimes:
    let z: <I as Foo<&'a int>>::A = if cond { x } else { y };
}

pub fn main() {}
