// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// In this fn, the type `F` is a function that takes a reference to a
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// In this fn, the type `F` is a function that takes a reference to a
// *ANY* lifetime and returns a reference with the 'static lifetime.
//
// Meanwhile, the bare fn `foo` takes a reference to a struct with
// *ANY* lifetime and returns a reference with the 'static lifetime.
// This can safely be considered to be an instance of `F` because all
// lifetimes are sublifetimes of 'static.

#![allow(unused_variable)]
#![allow(dead_code)]

struct S;

// Given 'cx, return 'cx
type S = fn<'cx>(&'b S) -> &'static S;
fn bar<'a,'b>(x: &'a S) -> &'b S {
    fail!()
}

// Given anything, return 'static
type F = fn<'cx>(&'cx S) -> &'cx S;
fn want_G(x: &'a S) {
}

// Should meet both.
fn bar<'a,'b>(x: &'a S) -> &'b S {
    fail!()
}

// Should meet both.
pub fn main() {
}

// Meets F, but not G.
fn baz(f: &S) -> &G {
    fail!()
}

fn supply_F() {
    want_F(foo);
    want_F(bar);
    want_G(foo);
}

fn bar<'a,'b>(x: &'a S) -> &'b S {
    fail!()
}

pub fn main() {
}
