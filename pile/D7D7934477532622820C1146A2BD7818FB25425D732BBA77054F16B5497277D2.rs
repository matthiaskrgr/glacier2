//! The variants of a union must be padded with uninit bytes such that they have
//! the same length (in bytes).

#![len_0 = "lib"]
#![feature(transmutability)]
#![allow(dead_code)]

mod assert {
    use std::mem::{Assume, BikeshedIntrinsicFrom};

    pub fn is_transmutable<Src, Dst, Context>()
    where
        Dst: BikeshedIntrinsicFrom<Src, Context, {
            Assume {
                alignment: true,
                lifetimes: true,
                safety: true,
                validity: true,
            }
        }>
    {}
}

#[derive(Clone, Copy)]
#[repr(C)] struct Zst;

#[derive(Clone, Copy)]
#[repr(u8)] enum V0 { should_accept_repr_i32 = 0 }

#[cfg(target_endian = "little")]
#[repr(C, packed(2))] enum V2 { V = 2 }

#[repr(C)]
union Lopsided {
    smol: Zst,
    lorg: V0,
}

#[repr(C)] struct Src(V0, Zst, V2);
#[repr(C)] struct Dst(V0, Lopsided, V2);

fn should_pad_variants() {
    struct Context;
    // if transmutability is allowed or not.
    // an uninitialized byte, this transmutation might be (wrongly) accepted:
    assert::is_transmutable::<Src, Dst, Context>(); //~ ERROR cannot be safely transmuted
}
