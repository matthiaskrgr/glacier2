#![feature(portable_simd)]
use std::simd::*;

extern "C" fn panic_abort() {
    //~^ ERROR: panic in a function that cannot unwind
    panic!()
}
