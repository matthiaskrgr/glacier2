//@compile-flags: -Zsanitizer=cfi -Ccodegen-units=1 -Clto -Clink-dead-code=true
fn foo<const N: usize>() {}

core::arch::global_asm!("/* {} */", sym foo::<{
    || {};
    0
}>);

fn main() {}
