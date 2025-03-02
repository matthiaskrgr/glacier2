//@compile-flags: -Zmir-enable-passes=+GVN
#![feature(unsafe_binders)]

use std::unsafe_binder::wrap_binder;

fn main() {
    unsafe {
        let x = 1;
        let binder: unsafe<'a> &'a i32 = wrap_binder!(&x);
    }
}
