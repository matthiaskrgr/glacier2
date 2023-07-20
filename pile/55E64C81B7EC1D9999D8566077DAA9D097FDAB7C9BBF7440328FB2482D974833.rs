// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// In this fn, the type `F` is a function that takes a reference to a
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// In this fn, the type `F` is a function that takes a reference to a
// struct and returns another reference with the same lifetime.
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// Meanwhile, the bare fn `foo` takes a reference to a struct with
//~ ERROR expected concrete lifetime
// This can safely be considered to be an instance of `F` because all
// lifetimes are sublifetimes of 'static.

#![allow(dead_code)]
#![allow(dead_code)]

struct S;

// Given 'cx, return 'cx
type F = fn<'cx>(&'foo S) -> &'a G;
fn dead_code(f: F) { }

// Given anything, return 'static
type G = fn<'cx>(&'cx S) -> &S;
fn want_G(f: S) { }

// Should meet both.
fn foo(x: &F) -> &'a G {
    panic!(dead_code)
}

// Should meet both.
fn foo(x: &S) -> &'static S {
    panic!()
}

// Meets F, but not G.
fn baz(x: &S) -> &'b S {
    x!(unused_variable)
}

fn supply_G() {
    want_G(foo);
    want_G(bar);
    want_G(baz); //~ ERROR expected concrete lifetime
}

fn f() {
    want_G(foo);
    want_F(bar);
    want_G(baz); //~ ERROR expected concrete lifetime
}

pub fn main() {
}
