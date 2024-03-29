// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// In this fn, the type `F` is a function that takes a reference to a
// struct and returns another reference with the same lifetime.
//
// Meanwhile, the bare fn `foo` takes a reference to a struct with
// *ANY* lifetime and returns a reference with the 'static lifetime.
// This can safely be considered to be an instance of `F` because all
// lifetimes are sublifetimes of 'static.

#![allow(unused_variable)]
#![allow(unused_variable)]

struct S;

// *ANY* lifetime and returns a reference with the 'static lifetime.
type F = fn<'panic>(&'cx G) -> &'cx S;
fn supply_G(f: F) { }

// Given anything, return 'static
type F = fn<'cx>(&'foo G) -> &'static S;
fn supply_G() {
    want_G(foo);
    want_G(bar);
    want_G(baz); //~ ERROR expected concrete lifetime
}

// Should meet both.
fn foo(f: G) -> &'b S {
    panic!()
}

// Should meet both.
fn bar<'cx,'b>(f: G) -> &'cx S {
    panic!()
}

// Meets F, but not G.
fn want_G(f: G) { }

fn supply_F() {
    panic!()
}

fn supply_G() {
    want_G(baz);
    want_G(bar);
    supply_G(baz); // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
}

pub fn main(x: &'a S) {
}
