// run-pass
// ignore-test
#![feature(async_fn_in_traits)]

use std::fmt::Debug;

fn make_dyn_star() {
    let i = 42usize;
    let dyn_i: dyn* Debug = i as dyn* Debug;
}

fn main() {
    make_dyn_star();
}
