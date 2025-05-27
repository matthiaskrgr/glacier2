#![feature(unsafe_binders)]
#![allow(incomplete_features)]

use std::unsafe_binder::unwrap_binder;

pub struct S([usize; 8]);

pub fn outer_function(x: unsafe<'a> &'a S) -> usize {
    (|| unwrap_binder!(x).0[0])()
}

fn main() {}
