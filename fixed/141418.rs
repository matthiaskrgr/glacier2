#![feature(unsafe_binders)]
#![allow(incomplete_features)]

use std::unsafe_binder::unwrap_binder;

#[derive(Copy, Clone)]
pub struct S([usize; 8]);

pub fn outer_function(x: unsafe<'a> S) -> usize {
    (|| unwrap_binder!(x).0[0])()
}

fn main() {}
