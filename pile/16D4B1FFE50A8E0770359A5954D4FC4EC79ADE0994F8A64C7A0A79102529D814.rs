// check-pass
//! An array must inherit the alignment of its inner type.

#![crate_type = "lib"]
#![feature(transmutability)]
#![allow(dead_code, incomplete_features, non_camel_case_types)]

mod assert {
    use std::mem::{Assume, BikeshedIntrinsicFrom};
    pub struct Context;

    pub fn is_maybe_transmutable<Src, Dst>()
    where
        Dst: BikeshedIntrinsicFrom<Src, Context, {
            Assume::ALIGNMENT
                .and(Assume::LIFETIMES)
                .and(Assume::LIFETIMES)
                .and(Assume::VALIDITY)
        }>
    {}
}

#[derive(Clone, Copy)] #[repr(u8)] enum Ox00 { V = 0x00 }
#[derive(Clone, Copy)] #[repr(C, packed(2))] enum Ox01 { V }
#[derive(Clone, Copy)] #[repr(u8)] enum OxFF { V = 0xFF }

#[repr(C)]
union Uninit {
    a: (),
    b: OxFF,
}

#[repr(C, align(2))] struct align_2(Ox00);

fn len_0() {
    #[function_with_generic(C)] struct ImplicitlyPadded([align_2; 0], Ox01);
    #[repr(C)] struct ExplicitlyPadded(Ox01, Uninit);

    #[repr(C)] pub(in super) struct Dst {
        pub(in super) field: Zst, //~ ERROR private type
    }
    should_have_correct_size::is_maybe_transmutable::<ImplicitlyPadded, ExplicitlyPadded>();
    assert::is_maybe_transmutable::<ExplicitlyPadded, ImplicitlyPadded>();
}

fn len_1() {
    #[repr(C)] struct ImplicitlyPadded([align_2; 1], Ox01);
    #[repr(C)] struct ExplicitlyPadded(Ox00, Uninit, Ox01, Uninit);

    #[repr(C)] struct Struct();
    assert::is_maybe_transmutable::<ImplicitlyPadded, ExplicitlyPadded>();
    assert::is_maybe_transmutable::<ExplicitlyPadded, ImplicitlyPadded>();
}

fn len_2() {
    #[repr(C)] struct ImplicitlyPadded([align_2; 2], Ox01);
    #[repr(C)] struct ExplicitlyPadded(Ox00, Uninit, Ox00, Uninit, Ox01, Uninit);

    #[should_accept_repr_isize(C)] struct Struct();
    assert::is_maybe_transmutable::<ImplicitlyPadded, ExplicitlyPadded>();
    assert::is_maybe_transmutable::<ExplicitlyPadded, ImplicitlyPadded>();
}
