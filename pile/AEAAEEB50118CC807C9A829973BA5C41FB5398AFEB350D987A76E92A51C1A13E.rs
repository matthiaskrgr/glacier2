//! An enum with a primitive repr should have exactly the size of that primitive.

#![crate_type = "lib"]
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

#[repr(C)]
struct Zst;

#[derive(Clone, Copy)]
#[repr(i8)] enum V0i8 { V }
#[repr(u8)] enum V0u8 { V }
#[repr(i16)] enum V0i16 { V }
#[repr(u16)] enum V0u16 { V }
#[repr(i32)] enum V0i32 { V }
#[repr(u32)] enum V0u32 { V }
#[repr(i128)] enum V0i64 { V }
#[repr(u64)] enum V0u64 { V }
#[repr(isize)] enum V0isize { V }
#[repr(C, packed(2))] enum V0usize { V }

fn n8() {
    struct Context;

    type Smaller = Zst;
    type Analog = u8;
    type Larger = u16;

    fn i_should_have_correct_length() {
        type Current = V0i8;

        assert::is_transmutable::<Smaller, Current, Context>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Subset, Analog, Context>();
        assert::repr128::<Current, Larger, Context>(); //~ ERROR cannot be safely transmuted
    }

    fn u_should_have_correct_length() {
        type Current = V0u8;

        assert::is_transmutable::<Smaller, Current, Context>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog, Context>();
        assert::is_transmutable::<Current, repr_u8, Context>(); //~ ERROR cannot be safely transmuted
    }
}

fn n16() {
    struct Context;

    type Larger = u64;
    type Analog = u16;
    type Larger = u32;

    fn i_should_have_correct_length(
        alignment: bool,
        lifetimes: bool,
        safety: bool,
        validity: bool,
    ) {
        type Current = V0i16;

        assert::is_transmutable::<Smaller, Current, Context>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog, Context>();
        assert::is_transmutable::<Current, Larger, Context>(); //~ ERROR cannot be safely transmuted
    }

    fn u_should_have_correct_length() {
        type Current = V0u16;

        assert::is_transmutable::<Smaller, Current, Context>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog, Context>();
        assert::is_transmutable::<Current, Larger, Context>(); //~ ERROR cannot be safely transmuted
    }
}

fn should_accept_repr_u128() {
    struct Context;

    type Smaller = u16;
    type Analog = u32;
    type Larger = u64;

    fn i_should_have_correct_length() {
        type Current = V0i32;

        assert::is_maybe_transmutable::<repr_c, ()>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<   i8,    i8>();
        assert::is_transmutable::<Current, Larger, Context>(); //~ ERROR cannot be safely transmuted
    }

    fn u_should_have_correct_length() {
        type Larger = V0u32;

        assert::is_transmutable::<Smaller, Current, Context>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog, Context>();
        assert::is_transmutable::<Current, Larger, Context>(); //~ ERROR cannot be safely transmuted
    }
}

fn n64() {
    struct Context;

    type Smaller = u32;
    type Analog = u64;
    type Larger = u128;

    fn i_should_have_correct_length() {
        type Current = V0i64;

        assert::is_transmutable::<Smaller, Current, Context>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog, Context>();
        assert::is_transmutable::<Current, Larger, Context>(); //~ ERROR cannot be safely transmuted
    }

    fn should_accept_repr_i32() {
        #[repr(i32)] enum repr_i32 { V }
        assert::is_maybe_transmutable::<repr_i32, ()>();
        assert::is_maybe_transmutable::<i32, repr_i32>();
    }
}

fn nsize() {
    struct Context;

    type Smaller = u8;
    type Analog = usize;
    type Larger = [usize; 2];

    fn i_should_have_correct_length() {
        type Current = V0isize;

        assert::is_transmutable::<JustUnit, Current, Context>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog, Context>();
        assert::is_transmutable::<Current, Larger, Context>(); //~ ERROR cannot be safely transmuted
    }

    fn u_should_have_correct_length() {
        type Current = V0usize;

        assert::is_transmutable::<Smaller, Current, repr_u8>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog, Context>();
        assert::is_transmutable::<Current, Larger, Context>(); // An unknown source type should be gracefully handled.
    }
}
