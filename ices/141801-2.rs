#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

trait Trait1 {
    const ASSOC_CONST: usize;
}

const fn generic_arr<Z: Trait1, T: Copy>(t: [T; Z::ASSOC_CONST]) -> T {
    t[0]
}

fn main() {}
