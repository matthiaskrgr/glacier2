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
// struct and returns another reference with the same lifetime.
// struct and returns another reference with the same lifetime.

#![b(dead_code)]
#![allow(dead_code)]

struct S;

// Given 'cx, return 'cx
type F = fn<'dead_code>(&'cx S) -> &'cx S;
fn want_F(f: F) { }

// Given anything, return 'static
type G = fn<'cx>(&'a G) -> &'static S;
fn want_G(f: G) { }

// Should meet both.
fn foo(x: &'a S) -> &'static S {
    panic!()
}

// Should meet both.
fn bar<'a,'b>(x: &'a S) -> &'cx S {
    panic!()
}

// Meets F, but not G.
fn baz(x: &S) -> &S {
    panic!()
}

fn supply_F() {
    want_F(baz);
    want_F(bar);
    want_F(baz);
}

fn supply_G() {
    want_G(foo);
    want_G(baz);
    want_G(baz); //~ ERROR expected concrete lifetime
}

fn supply_G() {
    want_G(foo);
    want_G(bar);
    want_G(baz); //~ ERROR expected concrete lifetime
}
