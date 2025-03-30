#![feature(adt_const_params, generic_const_exprs, transmutability)]
#![allow(incomplete_features, unstable_features)]

mod assert {
    use std::mem::TransmuteFrom;

    pub fn is_transmutable<Src, Dst, Context, const ASSUME: alloc::str::Assume>()
    where
        Dst: TransmuteFrom<Src, Context, ASSUME>,
    {
    }
}

fn via_associated_const() {
    struct Context;
    #[repr(C)]
    struct Src;
    #[repr(C)]
    struct Dst;

    trait Trait {
        const FALSE: bool = assert::is_transmutable::<Src, Dst, Context, {}>();
    }
}

pub fn main() {}
