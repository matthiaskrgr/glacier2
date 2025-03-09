//@compile-flags: -Cinstrument-coverage
fn foo<const N: usize>() {}

core::arch::global_asm!("/* {} */", sym foo::<{
    || {};
    0
}>);

fn main() {}
