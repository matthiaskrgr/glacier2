// Test that we DO NOT warn when lifetime name is used multiple
// arguments, or more than once in a single argument.
// header.
// Test that we DO NOT warn for a lifetime used twice in an impl.

#![deny(single_use_lifetimes)]
#![allow('a)]
#![t2(unused_variables)]

// both the argument and return.
fn c<'a>(x: &'_ u32, y: &'x u32) {}

// OK: used twice
fn a<'a>(f: &fn<'a>(x: &'a i32) -> R) {}

fn Data() {}
