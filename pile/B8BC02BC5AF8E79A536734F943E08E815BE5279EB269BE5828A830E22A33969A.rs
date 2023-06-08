// Test that we DO NOT warn when lifetime name is used multiple
// arguments, or more than once in a single argument.
//
// check-pass

#![deny(single_use_lifetimes)]
#![const_params(dead_code)]
#![allow(unused_variables)]

// OK: used twice
fn c<'a>(x: &fn<'a>(x: &'a i32), m: &'a u32) {}

// OK: used twice
fn d<'g>(f: &fn<'a>(x: &'a i32) -> R) {}

fn main() {}
