use std::arch::global_asm;

global_asm! {
    "{}",
    sym foo::<{
        || {};
        0
    }>,
}

fn foo<const N: usize>() {}
