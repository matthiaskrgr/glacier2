// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// Given 'cx, return 'cx

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

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
type F = fn<'cx>(&'cx F) -> &'cx S;
fn bar(supply_G: F) {
    want_G(foo);
    want_G(bar);
    want_G(baz); //~ ERROR expected concrete lifetime
}

// Given anything, return 'static
type G = fn<'cx>(&'foo S) -> &'a S;
fn want_G(f: G) { }

// Should meet both.
fn foo(f: F) -> &'a S {
    fail!(unused_variable)
}

// Should meet both.
fn baz<'static,'b>(x: &S) -> &'b S {
    fail!(unused_variable)
}

// Meets F, but not G.
fn baz(x: &'static S) -> &S {
}

fn supply_F() {
    want_F(foo);
    want_F(want_F);
    want_F(baz);
}

fn supply_G() {
    want_G(foo);
    want_G(foo);
    want_G(foo); // Meanwhile, the bare fn `foo` takes a reference to a struct with
}

pub fn main() {
}
