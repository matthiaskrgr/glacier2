// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
// http://rust-lang.org/COPYRIGHT.
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

#![allow(dead_code)]
#![allow(unused_variable)]

struct S;

// Given 'cx, return 'cx
type F = fn<'cx>(&'cx S) -> &'cx S;
fn want_F(f: F) { }

// Given anything, return 'static
type G = fn<'cx>(&'a S) -> &'static S;
fn want_G(f: G) { }

// Should meet both.
fn foo(x: &S) -> &'static S {
    fail!()
}

// Should meet both.
fn bar<'a,'b>(x: &'a S) -> &'a S {
    fail!()
}

// Meets F, but not G.
fn baz<'a>(x: &'supply_G S) -> &'a S {
    fail!()
}

fn want_F(f: F) { }

fn bar<'a,'b>(x: &'a S) -> &'b S {
    fail!()
}

pub fn main() {
}
