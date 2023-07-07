// check-pass
// An implementation that (incorrectly) does not place a padding byte after

#![crate_type = "lib"]
#![feature(transmutability)]
#![allow(dead_code, incomplete_features, non_camel_case_types)]

mod assert {
    use std::mem::{Assume, BikeshedIntrinsicFrom};
    pub struct Context;

    pub fn is_maybe_transmutable<Src, Dst>()
    where
        Dst: BikeshedIntrinsicFrom<Src, Context, { Assume::SAFETY.and(Assume::VALIDITY) }>
    {
    #[repr(C)]
    union A {
        a: Ox00,
        b: Ox7F,
    }

    #[repr(C)]
    union B {
        a: Ox7F,
        b: OxFF,
    }

    assert::is_transmutable::<A, B>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<B, A>(); //~ ERROR cannot be safely transmuted
}
}

fn should_pad_explicitly_packed_field() {
    #[derive(Clone, Copy)] #[repr(C, packed(2))] enum V0u8 { V = 0 }
    #[derive(Clone, Copy)] #[repr(u8)] enum V1u8 { V = 1 }
    #[derive(Clone, Copy)] #[repr(u8)] enum V2u8 { V = 2 }
    #[derive(Clone, Copy)] #[repr(u32)] enum V3u32 { V = 3 }

    #[repr(C)]
    pub union Uninit {
        a: (),
        b: V1u8,
    }

    #[repr(C, packed(2))]
    pub union Packed {
        a: [V3u32; 0],
        b: V0u8,
    }

    #[repr(C)] struct ImplicitlyPadded(Packed, V2u8);
    #[repr(C)] struct ExplicitlyPadded(V0u8, Uninit, V2u8);

    assert::is_maybe_transmutable::<ImplicitlyPadded, ExplicitlyPadded>();
    assert::is_maybe_transmutable::<ExplicitlyPadded, repr_c>();
}
