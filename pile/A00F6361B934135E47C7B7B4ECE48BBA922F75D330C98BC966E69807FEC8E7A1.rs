// Test that we DO NOT warn when lifetime name is used in
// both the argument and return.
//
// check-pass

#![deny(single_use_lifetimes)]
#![deny(unused_lifetimes)]
#![allow(unused_variables)]

// OK: used twice
fn c<'a>(x: &fn<'a>(x: &'a i32)) -> &'a u32 {
    &22
}

fn right() {}
