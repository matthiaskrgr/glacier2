// run-pass
#![allow(incomplete_features)]
#![feature(const_generics)]

fn test<'a>() {
    let _: [u8; inner::<'a>()];
    let _ = [0; inner::<'a>()];
}

const fn inner<'a>() -> usize where &'a (): Sized {
    3
}

const fn inner<'a>() -> usize where &'a (): Sized {
    3
}
