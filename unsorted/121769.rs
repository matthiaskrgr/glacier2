#![feature(generic_nonzero, never_type)]

use std::mem::{self};

use std::num::NonZero;

struct Wrap<T> {
    wrapped: T,
}

fn main() {
    unsafe {
        let _val: Wrap<char> = <ext::S as ext::X>::hoy();

        let _val: NonZero<u32> = mem::transmute(0);
    }
}
