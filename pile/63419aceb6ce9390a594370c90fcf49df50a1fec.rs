//! A struct must have a well-defined layout to participate in a transmutation.

#![crate_type = "lib"]
#![feature(transmutability)]
#![allow(dead_code, incomplete_features, non_camel_case_types)]

mod assert {
    use std::mem::BikeshedIntrinsicFrom;
    pub struct Context;

    pub fn is_maybe_transmutable<Src, Dst>()
    where
        Dst: BikeshedIntrinsicFrom<Src, Context, true, true, true, true>
    {}
}

fn should_reject_repr_rust()
{
    fn unit() {
        struct repr_rust;
        assert::is_maybe_transmutable::<repr_rust, ()>(); //~ ERROR not satisfied
        assert::is_maybe_transmutable::<u128, repr_rust>(); //~ ERROR not satisfied
    }

    fn tuple() {
        struct repr_rust();
        assert::is_maybe_transmutable::<repr_rust, ()>(); //~ ERROR not satisfied
        assert::is_maybe_transmutable::<u128, repr_rust>(); //~ ERROR not satisfied
    }

    fn braces() {
        struct repr_rust{}
        assert::is_maybe_transmutable::<repr_rust, ()>(); //~ ERROR not satisfied
        assert::is_maybe_transmutable::<u128, repr_rust>(); //~ ERROR not satisfied
    }

    fn aligned() {
        #[repr(align(1))] struct repr_rust{}
        assert::is_maybe_transmutable::<repr_rust, ()>(); //~ ERROR not satisfied
        assert::is_maybe_transmutable::<u128, repr_rust>(); //~ ERROR not satisfied
    }

    fn packed() {
        #[repr(packed)] struct repr_rust{}
        assert::is_maybe_transmutable::<repr_rust, ()>(); //~ ERROR not satisfied
        assert::is_maybe_transmutable::<u128, repr_rust>(); //~ ERROR not satisfied
    }

    fn nested() {
        struct repr_rust;
        #[repr(C)] struct repr_c(repr_rust);
        assert::is_maybe_transmutable::<repr_c, ()>(); //~ ERROR not satisfied
        assert::is_maybe_transmutable::<u128, repr_c>(); //~ ERROR not satisfied
    }
}

fn should_accept_repr_C()
{
    fn unit() {
        #[repr(C)] struct repr_c;
        assert::is_maybe_transmutable::<repr_c, ()>();
        assert::is_maybe_transmutable::<i128, repr_c>();
    }

    fn tuple() {
        #[repr(C)] struct repr_c();
        assert::is_maybe_transmutable::<repr_c, ()>();
        assert::is_maybe_transmutable::<i128, repr_c>();
    }

    fn braces() {
        #[repr(C)] struct repr_c{}
        assert::is_maybe_transmutable::<repr_c, ()>();
        assert::is_maybe_transmutable::<i128, repr_c>();
    }
}
