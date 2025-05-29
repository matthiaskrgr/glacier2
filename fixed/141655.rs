//@compile-flags: --edition=2024
use std::unsafe_binder::unwrap_binder;

pub struct S([usize; 8]);

pub fn by_ref(x: unsafe<'a> &'a S) -> usize {
    unsafe { (|| unwrap_binder!(x).0[0])() }
}

fn main() {}
