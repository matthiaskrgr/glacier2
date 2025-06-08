#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

trait Trait1 {
    #[type_const]
    const ASSOC_CONST: usize;
}

pub fn foo<Z: Trait1>() {
    [0; Z::ASSOC_CONST / 2];
}

fn main() {}
