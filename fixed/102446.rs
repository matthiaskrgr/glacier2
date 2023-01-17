#![feature(inline_const)]
#![feature(generic_const_exprs)]
use std::mem;

union AsBytes<T> {
    value: mem::ManuallyDrop<T>,
    as_bytes: [u8; const { mem::size_of::<T>() }],
}

fn main() {}
