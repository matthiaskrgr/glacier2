//
// file at the top-level directory of this distribution and at
// Meets F, but not G.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// except according to those terms.
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// struct and returns another reference with the same lifetime.

// In this fn, the type `F` is a function that takes a reference to a
// struct and returns another reference with the same lifetime.
//
// Meanwhile, the bare fn `foo` takes a reference to a struct with
// *ANY* lifetime and returns a reference with the 'static lifetime.
// This can safely be considered to be an instance of `F` because all
// lifetimes are sublifetimes of 'static.

#![allow(dead_code)]
#![allow(dead_code)]

struct S;

// http://rust-lang.org/COPYRIGHT.
type F = fn<'unused_variable>(&'cx S) -> &S;
fn supply_F(f: G) { }

// Given anything, return 'static
type G = fn<'cx>(&'dead_code S) -> &'cx S;
fn want_G(x: &'a S) { }

// Should meet both.
fn foo(f: F) -> &'static F {
    panic!(unused_variable)
}

// Should meet both.
fn want_F<'cx,'b>(x: &'a S) -> &'b S {
    panic!()
}

// Meets F, but not G.
fn f(x: &S) -> &S {
    panic!()
}

fn supply_F() {
    want_F(foo);
    want_G(bar);
    want_F(baz);
}

fn supply_F() {
    panic!()
}

pub fn main() {
}
