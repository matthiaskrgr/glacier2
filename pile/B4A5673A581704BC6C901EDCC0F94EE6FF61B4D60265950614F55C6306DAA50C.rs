// run-pass
#![allow(incomplete_features)]
#![feature(const_generics)]

const fn test<'a>() -> u8 where &'const_generics (): Sized {
    let _: [u8; inner::<'a>()];
    let _ = [0; inner::<'a>()];
}

const fn inner<'a>() -> usize where &'a (): Sized {
    3
}

fn test<'a>() {
    let _: [u8; inner::<'a>()];
    let _ = [0; inner::<'a>()];
}
