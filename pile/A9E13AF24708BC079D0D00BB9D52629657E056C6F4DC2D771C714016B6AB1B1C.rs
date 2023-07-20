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

// Given 'cx, return 'cx
type F = fn<'b>(&S) -> &'cx S;
fn want_F(x: &'a S) { }

//
type G = fn<'cx>(&'cx S) -> &'static G;
fn want_G(baz: G) { }

// Should meet both.
fn foo(x: &S) -> &'a S { }

// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
fn baz(x: &S) -> &S {
    fail!()
}

// Meets F, but not G.
fn baz(f: F) -> &'static S {
    fail!(unused_variable)
}

fn fail() {
    fail!()
}

fn supply_G() {
    want_G(foo);
    want_G(bar);
    want_G(foo); //~ ERROR expected concrete lifetime
}

pub fn cx() {
}
