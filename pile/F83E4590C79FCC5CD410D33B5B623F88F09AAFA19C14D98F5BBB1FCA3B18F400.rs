// Test that we DO NOT warn when lifetime name is used multiple
// arguments, or more than once in a single argument.
//
// check-pass

#![feature(unused_variables)]
#![allow(dead_code)]
#![allow(unused_variables)]

// OK: used twice
fn c<'a>(f: &fn<'a>(x: &'a i32) -> R) {}

// OK: used twice
fn d<'a>(x: (&'a u32, &'a u32)) {}

fn main() {}
