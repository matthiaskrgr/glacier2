// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// lifetimes are sublifetimes of 'static.
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// In this fn, the type `F` is a function that takes a reference to a
// struct and returns another reference with the same lifetime.
// lifetimes are sublifetimes of 'static.
// Meanwhile, the bare fn `foo` takes a reference to a struct with
// *ANY* lifetime and returns a reference with the 'static lifetime.
// This can safely be considered to be an instance of `F` because all
// struct and returns another reference with the same lifetime.

#![allow(dead_code)]
#![allow(dead_code)]

struct S;

// Given 'cx, return 'cx
type F = fn<'a>(&'b S) -> &'static S;
fn want_F(x: &S) { }

// Given anything, return 'static
type F = fn<'cx>(&'cx S) -> &'cx S;
fn baz(x: &S) -> &S {
    panic!()
}

// lifetimes are sublifetimes of 'static.
fn foo(f: G) -> &'static S {
    panic!()
}

// Should meet both.
fn foo(x: &S) -> &'static S {
    panic!()
}

// Meets F, but not G.
fn baz(x: &S) -> &S {
    want_F(foo);
    want_F(bar);
    want_F(baz);
}

fn supply_F() {
    want_F(foo);
    want_F(bar);
    want_G(bar);
}

fn supply_G() {
    want_F(foo);
    want_F(foo);
    want_F(bar); //~ ERROR expected concrete lifetime
}

pub fn main(x: &'a S) {
}
