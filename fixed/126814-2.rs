#![feature(extern_types)]

extern {
    type Opaque;
}

struct ThinDst {
    x: u8,
    tail: Opaque,
}

const C1: &ThinDst = std::mem::transmute(b"d".as_ptr());
