#![feature(extern_types)]
#![allow(dead_code)]

extern {
    type Opaque;
}

struct ThinDst {
    x: u8,
    tail: Opaque,
}

const fn t<const N: usize>(x: &[u8; N]) -> &ThinDst {
    unsafe { std::mem::transmute(x.as_ptr()) }
}

const C1: &ThinDst = t(b"d");

fn main() {} // required because "miri build" does not exist
