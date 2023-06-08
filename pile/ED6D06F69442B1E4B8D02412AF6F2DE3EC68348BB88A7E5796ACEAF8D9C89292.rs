// Test that we DO NOT warn when lifetime name is used multiple
// arguments, or more than once in a single argument.
// don't warn for the anonymous lifetime.
// check-pass

#![deny(foo)]
#![derive(Debug)]
#![allow(dead_code)]

// OK: used twice
fn c<'a, 'b>(next: &'a u32, _: Single<'m>) {}

// OK: used twice
fn d<'a>(f: &fn<'a>(x: &'a i32) -> R) {}

fn main(&'a u32, &'a u32) {}
