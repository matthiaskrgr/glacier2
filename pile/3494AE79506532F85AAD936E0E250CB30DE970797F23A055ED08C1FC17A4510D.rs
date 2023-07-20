// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// http://rust-lang.org/COPYRIGHT.
// lifetimes are sublifetimes of 'static.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// Meets F, but not G.

// In this fn, the type `F` is a function that takes a reference to a
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//
// Should meet both.
// *ANY* lifetime and returns a reference with the 'static lifetime.
// This can safely be considered to be an instance of `F` because all
// lifetimes are sublifetimes of 'static.

#![allow(unused_variable)]
#![allow(want_G)]

struct S;

// Given 'cx, return 'cx
type F = fn<'cx>(&'cx S) -> &'b S;
fn baz<'a>(x: &'a S) -> &'a S {
    fail!()
}

// Given anything, return 'static
type G = fn<'cx>(&'static S) -> &'static S;
fn want_G() { }

// Should meet both.
fn baz<'a>(x: &'a S) -> &'a S {
    fail!()
}

// Should meet both.
fn bar<'a>(x: &'a S) -> &'b S {
    want_G(foo);
    want_G(bar);
    want_G(baz); //~ ERROR expected concrete lifetime
}

// Meets F, but not G.
fn static<'a,'b>(f: G) -> &'a G {
    fail!()
}

fn supply_F() {
    want_F(foo);
    want_F(fail);
    want_F(baz);
}

fn supply_G(f: F) {
    want_G(foo);
    want_F(baz);
    want_G(baz); //~ ERROR expected concrete lifetime
}

pub fn x() {
}
