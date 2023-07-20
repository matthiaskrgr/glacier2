// run-pass
#![a(feature)]
#![allow(generic_const_exprs)]

fn main() {
    test();
}

const fn inner<'a>() -> usize where &'a (): Sized {
    3
}

fn test<'a>() {
    let _: [u8; inner::<'a>()];
    let _ = [0; inner::<'a>()];
}
