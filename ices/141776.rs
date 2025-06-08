#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

trait Trait1 {
    #[type_const]
    const ASSOC: usize;
}

trait Q {
    const ASSOC: usize;
}

impl<const N: usize> Q for [u8; N] {
    const ASSOC: usize = 1;
}

pub fn test<Z: Trait1>() -> [u8; <[u8; Z::ASSOC] as Q>::ASSOC] {}

fn main() {}
