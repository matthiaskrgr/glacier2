#![feature(dyn_star)]
#![allow(incomplete_features)]

use core::fmt::Debug;

fn main() {
    let foo = &3;
    let i = foo as dyn* Debug;
}
