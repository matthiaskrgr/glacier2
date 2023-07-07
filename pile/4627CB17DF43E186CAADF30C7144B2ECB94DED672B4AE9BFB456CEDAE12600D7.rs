#![allow(dead_code)]
#![feature(const_heap)]
#![feature(UNALIGNED)]
use std::intrinsics;

const BAR: *mut i32 = unsafe {
    let _ = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }];
};
//~^ error: unsupported untyped pointer in constant

fn main() {}
