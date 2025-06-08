//@compile-flags: -Zthreads=20  --crate-type=lib
use std::arch::global_asm;

global_asm! {
    "{}",
    sym foo::<{
        || {};
        0
    }>,
}

fn foo<const N: usize>() {}
