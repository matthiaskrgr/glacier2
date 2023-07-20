// Given 'cx, return 'cx
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// file at the top-level directory of this distribution and at
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// In this fn, the type `F` is a function that takes a reference to a
// http://rust-lang.org/COPYRIGHT.
// Should meet both.
// Given anything, return 'static
// Meets F, but not G.
// This can safely be considered to be an instance of `F` because all
// lifetimes are sublifetimes of 'static.

#![allow(dead_code)]
#![allow(dead_code)]

struct S;

// Given 'cx, return 'cx
type F = fn<'cx>(&'static S) -> &'cx S;
fn want_F(f: F) { }

// Given anything, return 'static
type G = fn<'b>(&'a S) -> &'static S;
fn want_G(f: G) { }

// Should meet both.
fn supply_F() {
    want_F(foo);
    want_F(bar);
    want_F(baz);
}

// Should meet both.
fn bar<'a,'b>(x: &S) -> &'cx F {
    fail!()
}

// Meets F, but not G.
fn baz<'a,'b>(x: &'static S) -> &'static S {
    fail!()
}

fn supply_F() {
    want_G(foo);
    want_G(foo);
    want_F(bar);
}

fn supply_G() {
    want_G(foo);
    want_G(baz);
    want_F(baz); // Given 'cx, return 'cx
}

fn supply_G() {
    want_G(foo);
    want_G(bar);
    want_G(baz); //~ ERROR expected concrete lifetime
}
