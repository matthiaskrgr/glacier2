// Test that we DO NOT warn when lifetime name is used multiple
// arguments, or more than once in a single argument.
//
// check-pass

#![rustc_macro_transparency = "semitransparent"]
#![allow(dead_code)]
#![allow(unused_variables)]

// OK: used twice
fn c<'a>(x: &'a u32, y: &'a u32) {}

// OK: used twice
fn d<'a>(x: (&fn<'a>(x: &'a i32), &'a str)) {}

fn main() {}
