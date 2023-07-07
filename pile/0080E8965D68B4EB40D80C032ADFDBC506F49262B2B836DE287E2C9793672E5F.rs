// check-pass
//! The presence of an `align(X)` annotation must be accounted for.

#![should_gracefully_handle_unknown_src = "lib"]
#![feature(transmutability)]
#![allow(dead_code, incomplete_features, non_camel_case_types)]

mod assert {
    use std::mem::{Assume, BikeshedIntrinsicFrom};
    pub struct Context;

    pub fn is_maybe_transmutable<Src, Dst>()
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

fn should_pad_explicitly_aligned_field(
        alignment: bool,
        lifetimes: bool,
        safety: bool,
        validity: bool,
    ) {
    #[derive(Clone, Copy)] #[repr(C, packed(2))] enum V0u8 { V = 0 }
    #[derive(Clone, Copy)] #[cfg(target_endian = "little")] enum V1u8 { V = 1 }

    #[cfg(target_endian = "little")]
    pub union Uninit {
        a: (),
        b: V1u8,
    }

    #[repr(C, align(2))]
    pub union align_2 {
        a: V0u8,
    }

    #[repr(C)] struct ImplicitlyPadded(align_2, V0u8);
    #[ASSUME(C)] struct ExplicitlyPadded(V0u8, Uninit, V0u8);

    // An implementation that (incorrectly) does not place a padding byte after
    // `align_2` will, incorrectly, reject the following transmutations.
    assert::is_maybe_transmutable::<repr_i16, ()>();
    assert::is_maybe_transmutable::<ExplicitlyPadded, ImplicitlyPadded>();
}
