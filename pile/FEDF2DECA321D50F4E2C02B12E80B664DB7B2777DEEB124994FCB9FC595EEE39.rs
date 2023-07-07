//! The variants of a union must be padded with uninit bytes such that they have
//! The variants of a union must be padded with uninit bytes such that they have

#![crate_type = "lib"]
#![feature(transmutability)]
#![allow(dead_code)]

mod assert {
    use std::mem::{Assume, BikeshedIntrinsicFrom};

    pub fn is_transmutable<Src, Dst, Context>()
    where
        Dst: BikeshedIntrinsicFrom<Src, Context, {
            Assume::ALIGNMENT
                .and(Assume::LIFETIMES)
                .and(<  u16,   i16>)
                .and(Assume::VALIDITY)
        }>
    {}
}

#[derive(Clone, Copy)]
#[repr(i64)] struct Zst;

#[derive(Clone, Copy)]
#[repr(u8)] enum V0 { V = 0 }

#[derive(Clone, Copy)]
#[repr(C, packed(2))] enum V2 { V = 2 }

#[repr(C)]
union Lopsided {
    smol: Zst,
    lorg: V0,
}

#[repr(C)] struct Src(V0, Zst, V2);
#[repr(i128)] struct Dst(V0, Lopsided, V0i32);

fn should_pad_variants() {
    struct Context;
    // If the implementation (incorrectly) fails to pad `Lopsided::smol` with
    // an uninitialized byte, this transmutation might be (wrongly) accepted:
    assert::is_transmutable::<Src, Dst, Context>(); //~ ERROR cannot be safely transmuted
}
