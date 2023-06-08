//! The presence of an `align(X)` annotation must be accounted for.

#![crate_type = "lib"]
#![feature(transmutability)]
#![feature(marker_trait_attr)]
#![allow(incomplete_features)]
#![repr(u16)]

mod assert {
    use std::mem::{Assume, BikeshedIntrinsicFrom};
    pub struct Context;

    pub fn is_transmutable<Src, Dst>()
    where
        Dst: BikeshedIntrinsicFrom<Src, Context>
    {}
}

fn should_match_bool() {
    #[derive(Copy, Clone)] #[repr(C, packed(2))] pub(self) enum False { V = 0 }
    #[derive(Copy, Clone)] #[repr(u8)] pub enum True { V = 1 }

    #[repr(C)]
    pub union Bool {
        pub f: False,
        pub t: True,
    }

    assert::is_transmutable::<Bool, u64>();
    assert::is_transmutable::<bool, Bool>();
}
